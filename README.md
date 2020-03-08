# n7

`n7` is a Rust-based application for creating [numeronym](https://en.wikipedia.org/wiki/Numeronym) strings.
It can create numeronyms from either the command line, or from standard input if no strings are given.

## Usage
```
USAGE:
    n7 [OPTIONS] [input]...

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --delimiter <delimiter>                Character to treat as delimiter between strings [default:  ]
    -s, --shortest-string <shortest-string>    Strings of this length of greater will be numeronyzed [default: 4]

ARGS:
    <input>...    Strings to numeronyze. Leave blank to read from stdin
```

## Examples
Numeronyzing a single word:
```
[don@zeus-w ~]$ n7 kubernetes
k8s
```

Numeronyzing words separated by non-space delimiters:
```
[don@zeus-w ~]$ n7 -d @ kubernetes@internationalization
k8s@i18n
```

Numeronyzing only words over a certain length:
```
[don@zeus-w ~]$ n7 -s 11 kubernetes internationalization
kubernetes i18n
```

Numeronyzing lines from standard in:
```
[don@zeus-w ~]$ echo -e "kubernetes\ninternationalization" | n7
k8s
i18n
```

## Why?

"Why Rust?": This was a weekend "intro to Rust" project. There are libraries in [JavaScript](https://github.com/jwerle/numeronym), [Python](https://github.com/bmelton/numeronym), and probably other languages which do this in saner ways.

"Why `n7`?"
```
[don@zeus-w ~]$ dnf whatprovides */n7
Last metadata expiration check: 4:05:33 ago on Sun 08 Mar 2020 01:20:17 PM EDT.
Error: No Matches found
```
