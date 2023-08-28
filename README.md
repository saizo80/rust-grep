# rust-grep

implementation of grep written in rust

currently not on par with standard gnu utils grep, this project needs optimization

## usage

```
Usage:
  rgrep [OPTIONS] PATTERN [FILES ...]

Rust grep

Positional arguments:
  pattern               Pattern to search for
  files                 Files to search

Optional arguments:
  -h,--help             Show this help message and exit
  -r,--recursive        Recursive search
  -i,--ignore-case      Case insensitive search (not currently implemented)
  -v,--invert-match     Invert match
```

## installation

cargo install --git https://github.com/saizo80/rust-grep.git
