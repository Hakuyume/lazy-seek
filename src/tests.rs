use super::*;
use std::io::Cursor;

#[test]
fn test_buffered_reader() {
    let inner = Cursor::new(&[5, 6, 7, 0, 1, 2, 3, 4]);
    let mut reader = LazySeekBufReader::with_capacity(2, inner);

    let mut buf = [0, 0, 0];
    let nread = reader.read(&mut buf);
    assert_eq!(nread.unwrap(), 3);
    assert_eq!(buf, [5, 6, 7]);
    // assert_eq!(reader.buffer(), []);

    let mut buf = [0, 0];
    let nread = reader.read(&mut buf);
    assert_eq!(nread.unwrap(), 2);
    assert_eq!(buf, [0, 1]);
    // assert_eq!(reader.buffer(), []);

    let mut buf = [0];
    let nread = reader.read(&mut buf);
    assert_eq!(nread.unwrap(), 1);
    assert_eq!(buf, [2]);
    // assert_eq!(reader.buffer(), [3]);

    let mut buf = [0, 0, 0];
    let nread = reader.read(&mut buf);
    assert_eq!(nread.unwrap(), 1);
    assert_eq!(buf, [3, 0, 0]);
    // assert_eq!(reader.buffer(), []);

    let nread = reader.read(&mut buf);
    assert_eq!(nread.unwrap(), 1);
    assert_eq!(buf, [4, 0, 0]);
    // assert_eq!(reader.buffer(), []);

    assert_eq!(reader.read(&mut buf).unwrap(), 0);
}

#[test]
fn test_buffered_reader_seek() {
    let inner = Cursor::new(&[5, 6, 7, 0, 1, 2, 3, 4]);
    let mut reader = LazySeekBufReader::with_capacity(2, inner);

    assert_eq!(reader.seek(SeekFrom::Start(3)).ok(), Some(3));
    assert_eq!(reader.fill_buf().ok(), Some(&[0, 1][..]));
    assert_eq!(reader.seek(SeekFrom::Current(0)).ok(), Some(3));
    assert_eq!(reader.fill_buf().ok(), Some(&[0, 1][..]));
    assert_eq!(reader.seek(SeekFrom::Current(1)).ok(), Some(4));
    assert_eq!(reader.fill_buf().ok(), Some(&[1][..]));
    reader.consume(1);
    assert_eq!(reader.seek(SeekFrom::Current(-2)).ok(), Some(3));
}
