## `lazy_seek::BufReader`
This is an alternative of [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
that calls `seek` of the underlying reader lazily.

`lazy_seek::BufReader::seek` keeps the content of the internal buffer as much as possible,
where `std::io::BufReader::seek` flushes it anytime.

## Example usage
[`ZipArchive`](https://docs.rs/zip/0.6.6/zip/read/struct.ZipArchive.html) calls a lot of `read`s and `seek`s.
`lazy_seek::BufReader` can reduce the number of system calls and make the program faster.
