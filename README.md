## `lazy_seek::BufReader`
This is an alternative of [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
that calls `seek` of the underlying reader lazily.

`<BufReader as std::io::BufRead>::seek` keeps the content of the internal buffer as much as possible,
where `<BufReader as std::io::BufRead>::seek` flushes it anytime.

## Example usage
[`ZipArchive`](https://docs.rs/zip/0.6.6/zip/read/struct.ZipArchive.html) calls a lot of `seek`s and `read`s.
`lazy_seek::BufReader` can reduce the number of system calls and make the program faster.
