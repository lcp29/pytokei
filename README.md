# pytokei
---
Pytokei is a python binding to [tokei](https://github.com/XAMPPRocky/tokei):

<p align="center">
    Tokei is a program that displays statistics about your code. Tokei will show the number of files, total lines within those files and code, comments, and blanks grouped by language.
</p>

--- 

This wrapper allows to obtain the same reports directly from python.

```python
$ python
>>> import pytokei
>>> from rich import print
>>> langs = pytokei.Languages()
>>> langs.get_statistics(["."], ["tests/data", "requirements"], pytokei.Config())
>>> print(langs.report_compact_plain())
{
    'YAML': {'blanks': 6, 'code': 63, 'comments': 0, 'files': 1, 'lines': 69},
    'Python': {'lines': 376, 'blanks': 89, 'files': 2, 'code': 280, 'comments': 7},
    'Makefile': {'code': 18, 'lines': 26, 'comments': 0, 'blanks': 8, 'files': 1},
    'Markdown': {'code': 0, 'blanks': 37, 'files': 10, 'comments': 52, 'lines': 89},
    'Rust': {'blanks': 23, 'comments': 23, 'code': 317, 'lines': 363, 'files': 7},
    'TOML': {'code': 14, 'comments': 2, 'lines': 20, 'blanks': 4, 'files': 2}
}
```

For more information about `tokei`, please visit the original repo.

[![PyPI pyversions](https://img.shields.io/pypi/pyversions/pytokei.svg)](https://pypi.org/project/pytokei/)
![example workflow](https://github.com/plaguss/pytokei/actions/workflows/ci.yml/badge.svg)
[![license](https://img.shields.io/github/license/plaguss/pytokei.svg)](https://github.com/plaguss/pytokei/blob/main/LICENSE)


## Installation

```bash
pip install pytokei
```

Requires Python >= 3.7.

Binaries are available for:

* **Linux**: `x86_64`, `aarch64`, `i686`, `armv7l`, `musl-x86_64` & `musl-aarch64`
* **MacOS**: `x86_64` & `arm64` (except python 3.7)
* **Windows**: `amd64` & `win32`

Otherwise, you can install from source which requires Rust stable to be installed.

## Why this library?

Wanted to practice rust, and taking this library to python seemed like a good opportunity. It's awesome, and maybe more people coming from python will find something useful to do with it.

But really? Just for fun :)

## [Documentation](https://plaguss.github.io/pytokei/)

## What times should you expect?

Running `Languages.get_statistics` against [cpython](https://github.com/python/cpython) takes a little less than 200 milliseconds.

Some more info should be found in the [docs](https://plaguss.github.io/pytokei/#time-comparison-tokei-and-pytokei).

## Development

You will need:

- [maturin](https://www.maturin.rs/installation.html) to compile the library

- `maturin develop` / `make develop` to compile the code.

From python side:

Run `make install-dev` inside a virtual environment, `make test`, `make mypy` and `make format` to ensure everything is as expected, and `make docs` to build the documentation.

*There are some problems when building the docs with mkdocstrings, a reminder is in the following [github issue](https://github.com/mkdocstrings/mkdocstrings/issues/404). For the moment, it seems that the best option is to remove the .so file and build the docs without it.*
