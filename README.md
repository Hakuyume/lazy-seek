## `lazy_seek::LazySeekBufReader`
This is an alternative of [`std::io::BufReader`](https://doc.rust-lang.org/std/io/struct.BufReader.html)
that calls `seek` of the underlying reader lazily.

`<LazySeekBufReader as std::io::BufRead>::seek` keeps the content of the internal buffer as much as possible,
where `<BufReader as std::io::BufRead>::seek` flushs it anytime.

## Example usage
[`ZipArchive`](https://mvdnes.github.io/rust-docs/zip-rs/zip/read/struct.ZipArchive.html) calls a lot of `seek`s and `read`s.
`LazySeekBufReader` can reduce the number of system calls and make the program faster.

with `BufReader`
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
 55.04    2.301494           8    272370           lseek
 44.85    1.875107          10    182384           read
  0.03    0.001237         137         9           munmap
...
----- ----------- ----------- --------- --------- ----------------
100.00    4.181157                454974         2 total
```

with `LazySeekBufReader`
```
% time     seconds  usecs/call     calls    errors syscall
------ ----------- ----------- --------- --------- ----------------
 57.73    0.067559           5     11742           read
 40.10    0.046922           4     10902           lseek
  1.44    0.001683         187         9           munmap
...
------ ----------- ----------- --------- --------- ----------------
100.00    0.117022                 22864         2 total
```
