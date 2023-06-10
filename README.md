# tmplr

A command line tool to expand a MiniJinja template file
using variables in a YAML file.

## Usage

```
$ tmplr -h
Usage: tmplr [OPTIONS] --var <VAR> --tmpl <TMPL>

Options:
  -v, --var <VAR>    variable yaml file path
  -t, --tmpl <TMPL>  template file path
  -d, --dest <DEST>  destination file path [default: -]
  -h, --help         Print help
  -V, --version      Print version
```

## How to build a Debian/Ubuntu package

cargo deb --target x86_64-unknown-linux-musl
