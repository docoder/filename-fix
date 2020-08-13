# Filename Fix

A tool for fix filename

## Features

- Fix filename number prefix, skip when dir or file without number prefix
- Fix filename space with separator replacement, ignore the space before filename and file extension
- Fix file extension lowercase or uppercase, ignore file without extension
- ... ...

## Installation

```bash
$ cargo install filenamefix
```

## Usage

    USAGE:
        filenamefix CMD [<params>] <dirname>
    CMD:
        number
        space
        ext
    ARGS:
        <params> 
            (CMD: number)
            Number of digits in number prefix of filename, default 2
            (CMD: space)
            The separator that spaces are replaced with, default "_"
            (CMD: ext)
            If "upper", make the file extension uppercase, otherwise lowercase, default lowercase
        <dirname> dirname of files to fix

## Examples

    $ filenamefix number 3 /example/path
    "01.test1.rs" -> "001.test1.rs"
    "2-test2.rs" -> "002.test2.rs"
    "0003.test.rs" -> "003.test.rs"
    "4test.rs" -> "004.test.rs"

    $ filenamefix number /example/path
    "01--test1.rs" -> "01.test1.rs"
    "2-test2.rs" -> "02.test2.rs"
    "0003.test.rs" -> "03.test.rs"
    "4test.rs" -> "04.test.rs"

    $ filenamefix space - /example/path
    "a b c.rs" -> "a-b-c.rs"
    "a b c" -> "a-b-c"
    "b c d .rs" -> "b-c-d.rs"
    " c d e.rs" -> "c-d-e.rs"

    $ filenamefix space /example/path
    "a b c.rs" -> "a_b_c.rs"
    "a b c" -> "a_b_c"
    "b c d .rs" -> "b_c_d.rs"
    " c d e.rs" -> "c_d_e.rs"

    $ filenamefix ext upper /example/path
    "a.rs" -> "a.RS"
    "b" -> "b"
    "c.RS" -> "c.RS"

    $ filenamefix ext /example/path
    "a.rs" -> "a.rs"
    "b" -> "b"
    "c.RS" -> "c.rs"


