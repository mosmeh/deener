# deener

[![build](https://github.com/mosmeh/deener/workflows/build/badge.svg)](https://github.com/mosmeh/deener/actions)

Replace non-ACGT bases in FASTA file with random bases

## Usage

```
USAGE:
    deener [OPTIONS] <file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -s, --seed <seed>    Seed to initialize random number generator with [default: 1234]
    -w, --wrap <wrap>    Maximum width of sequences

ARGS:
    <file>    FASTA file
```
