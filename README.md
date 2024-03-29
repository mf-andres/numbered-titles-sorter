# Numbered Titles Sorter

## Index

1. [Overview](#overview)
2. [Requirements](#requirements)
3. [Installation](#installation)
4. [Tests](#tests)
5. [Getting started:](#getting_started)

## <a name="overview">Overview</a>

This repository contains a cli program to sort numbered titles (1., 1.1., 1.1.1, ...) in a text.

## <a name="requirements">Requirements</a>

- System requirements

cargo >= 1.62.1

- Package requirements

Listed in Cargo.toml under '[dependencies]'

## <a name="installation">Installation</a>

Install it locally via cargo:

```
cargo install numbered_titles_sorter
```

## <a name="tests">Tests</a>

Dowload this repository and run the tests via cargo:

```
cargo test
```

## <a name="getting_started">Getting started</a>

Install it and run it via cargo:

```
numbered_titles_sorter.exe <file>
```

```
numbered_titles_sorter.exe example_documents/example_1.txt
```

It will process the given file and write a processed file under the same folder named "processed\_<file>"

You can also download this repository and try the cli via cargo:

Type:

```
cargo run -- <file>
```

For example:

```
cargo run -- example_documents/example_1.txt
```
