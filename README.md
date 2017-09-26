# servox11test

This is a test repository to evaluate libservo for making GUIs.
The main test was how small servo could be:

Results:

- default `--release` flag, no LTO: 280 MB
- default `--release` flag, no LTO, stripped: 83.4 MB
- `-C prefer-dynamic` `--release`, with LTO: 245.5 MB
- `-C prefer-dynamic` `--release`, with LTO, stripped: 62.1 MB
- `-C prefer-dynamic` `--release`, with LTO, stripped, gzip compressed in .deb: 14.7 MB

Notice: `panic='abort'` does not work with servo. Neither does compiling with `musl`
