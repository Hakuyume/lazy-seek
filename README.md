## `lazy_seek::LazySeekBufReader`
This is an alternative of [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
that calls `seek` of the underlying reader lazily.

`<LazySeekBufReader as std::io::BufRead>::seek` keeps the content of the internal buffer as much as possible,
where `<BufReader as std::io::BufRead>::seek` flushs it anytime.
