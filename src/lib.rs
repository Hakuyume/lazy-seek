use std::io::{BufRead, Read, Result, Seek, SeekFrom};

pub struct LazySeekBufReader<R> {
    inner: R,
    buf: Box<[u8]>,
    pos: i64,
    cap: usize,
    offset: Option<u64>,
}

impl<R> LazySeekBufReader<R> {
    pub fn with_capacity(capacity: usize, inner: R) -> Self {
        unsafe {
            let mut buf = Vec::with_capacity(capacity);
            buf.set_len(capacity);
            Self {
                inner,
                buf: buf.into_boxed_slice(),
                pos: 0,
                cap: 0,
                offset: None,
            }
        }
    }
}

impl<R> Read for LazySeekBufReader<R>
where
    R: Read + Seek,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        if self.pos == self.cap as i64 && buf.len() >= self.buf.len() {
            let len = self.inner.read(buf)?;
            if let Some(offset) = self.offset.as_mut() {
                *offset += len as u64;
            }
            self.pos = 0;
            self.cap = 0;
            Ok(len)
        } else {
            let len = self.fill_buf()?.read(buf)?;
            self.consume(len);
            Ok(len)
        }
    }
}

impl<R> Seek for LazySeekBufReader<R>
where
    R: Seek,
{
    fn seek(&mut self, pos: SeekFrom) -> Result<u64> {
        match (self.offset, pos) {
            (Some(offset), SeekFrom::Current(pos)) => {
                self.pos += pos;
                Ok((offset as i64 + self.pos) as _)
            }
            (Some(offset), SeekFrom::Start(pos)) => {
                self.pos = pos as i64 - offset as i64;
                Ok((offset as i64 + self.pos) as _)
            }
            _ => {
                let offset = match pos {
                    SeekFrom::Current(pos) => self
                        .inner
                        .seek(SeekFrom::Current(pos + self.pos - self.cap as i64))?,
                    _ => self.inner.seek(pos)?,
                };
                self.offset = Some(offset);
                self.pos = 0;
                self.cap = 0;
                Ok(offset)
            }
        }
    }
}

impl<R> BufRead for LazySeekBufReader<R>
where
    R: Read + Seek,
{
    fn fill_buf(&mut self) -> Result<&[u8]> {
        if self.pos < 0 || self.cap as i64 <= self.pos {
            if self.pos == self.cap as i64 {
                if let Some(offset) = self.offset.as_mut() {
                    *offset += self.cap as u64;
                }
            } else {
                self.offset = Some(
                    self.inner
                        .seek(SeekFrom::Current(self.pos - self.cap as i64))?,
                );
            }
            self.cap = self.inner.read(&mut self.buf)?;
            self.pos = 0;
        }
        Ok(&self.buf[self.pos as usize..self.cap])
    }

    fn consume(&mut self, amt: usize) {
        assert!(0 <= self.pos && self.pos + (amt as i64) <= self.cap as i64);
        self.pos += amt as i64;
    }
}

#[cfg(test)]
mod tests;
