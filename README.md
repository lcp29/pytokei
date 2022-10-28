# pytokei
---
Pytokei is a python binding to [tokei](https://github.com/XAMPPRocky/tokei):

    Tokei is a program that displays statistics about your code. Tokei will show the number of files, total lines within those files and code, comments, and blanks grouped by language.
--- 

This wrapper allows to obtain the same reports directly from python.

For more information about `tokei`, please visit the original repo.

[![Lines Of Code](https://tokei.rs/b1/github/XAMPPRocky/tokei?category=code)](https://github.com/plaguss/pytokei)


## Installation

```bash
pip install pytokei
```

Requires Python >= 3.7.

Binaries are available for:

?

Otherwise, you can install from source which requires Rust stable to be installed.

## Why this library?

Wanted to practice rust, and taking this library to python seemed like a good opportunity. It's awesome, and maybe more people coming from python will find something useful to do with it.

But really? Just for fun :)

## [Documentation](not_ready)

## Development

You will need:

- [maturin](https://www.maturin.rs/installation.html) to compile the library

- `maturin develop` / `make develop` to compile the code.

From python side:

Run `make install-dev` inside a virtual environment, `make test`, `make mypy` and `make format` to ensure everything is as expected, and `make docs` to build the documentation.

*There are some problems when building the docs with mkdocstrings, a reminder is in the following [github issue](https://github.com/mkdocstrings/mkdocstrings/issues/404). For the moment, it seems that the best option is to remove the .so file and build the docs without it.*
