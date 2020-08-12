# Filename Fix

A tool for fix filename

## Features

- Fix filename number prefix, skip when dir or file without number prefix
- ... ...

## Installation

```bash
$ cargo install filenamefix
```

## Usage

```
USAGE:
    filenamefix CMD [<params>] <dirname>
CMD:
 		number
ARGS:
		<params> Number of digits in number prefix of filename, default 2
		<dirname> dirname of files to fix
```

## Examples

```
$ filenamefix number 3 .
"01.test1.rs" -> "001.test1.rs"
"2-test2.rs" -> "002.test2.rs"
"0003.test.rs" -> "003.test.rs"
"4test.rs" -> "004.test.rs"
```

```
$ filenamefix number .
"01--test1.rs" -> "01.test1.rs"
"2-test2.rs" -> "02.test2.rs"
"0003.test.rs" -> "03.test.rs"
"4test.rs" -> "04.test.rs"
```

