use std::io::{BufRead, Read, Result, Seek, SeekFrom};

pub struct BufReader<R> {
    inner: R,
    offset: u64,
    buf: Vec<u8>,
    pos: Option<u64>,
}

impl<R> BufReader<R> {
    pub fn new(inner: R) -> Self {
        // https://github.com/rust-lang/rust/blob/1.76.0/library/std/src/sys_common/io.rs#L3
        let capacity = if cfg!(target_os = "espidf") {
            512
        } else {
            8 * 1024
        };
        Self::with_capacity(capacity, inner)
    }

    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        Self {
            inner,
            offset: 0,
            buf: Vec::with_capacity(capacity),
            pos: None,
        }
    }

    pub fn buffer(&self) -> &[u8] {
        if let Some(start) = self.start() {
            &self.buf[start..]
        } else {
            &[]
        }
    }

    pub fn capacity(&self) -> usize {
        self.buf.capacity()
    }

    pub fn into_inner(self) -> R {
        self.inner
    }

    fn start(&self) -> Option<usize> {
        if let Some(pos) = self.pos {
            if self.offset <= pos && pos < self.offset + self.buf.len() as u64 {
                Some((pos - self.offset) as _)
            } else {
                None
            }
        } else {
            None
        }
    }

    fn sync(&mut self) -> Result<u64>
    where
        R: Seek,
    {
        let pos = if let Some(pos) = self.pos {
            self.inner.seek(SeekFrom::Start(pos))?;
            pos
        } else {
            self.inner.stream_position()?
        };
        self.pos = Some(pos);
        Ok(pos)
    }
}

impl<R> Read for BufReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.start().is_none() && self.capacity() <= buf.len() {
            let pos = self.sync()?;
            let len = self.inner.read(buf)?;
            self.pos = Some(pos + len as u64);
            Ok(len)
        } else {
            let len = self.fill_buf()?.read(buf)?;
            self.consume(len);
            Ok(len)
        }
    }
}

impl<R> Seek for BufReader<R>
where
    R: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match pos {
            SeekFrom::Current(pos) => {
                let pos = if let Some(pos) = self.pos {
                    pos
                } else {
                    self.inner.stream_position()?
                }
                .saturating_add_signed(pos);
                self.pos = Some(pos);
                Ok(pos)
            }
            SeekFrom::Start(pos) => {
                self.pos = Some(pos);
                Ok(pos)
            }
            _ => {
                let pos = self.inner.seek(pos)?;
                self.pos = Some(pos);
                Ok(pos)
            }
        }
    }
}

impl<R> BufRead for BufReader<R>
where
    R: Read + Seek,
{
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if let Some(start) = self.start() {
            Ok(&self.buf[start..])
        } else {
            let pos = self.sync()?;
            unsafe {
                self.buf.set_len(self.buf.capacity());
                let len = self.inner.read(&mut self.buf)?;
                self.buf.set_len(len);
            }
            self.offset = pos;
            Ok(&self.buf)
        }
    }

    fn consume(&mut self, amt: usize) {
        if let Some(pos) = &mut self.pos {
            *pos += amt as u64;
        }
    }
}

#[cfg(test)]
mod tests;
