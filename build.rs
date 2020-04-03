use std::fs;

macro_rules! readme {
    () => {"\
[![License: GPL](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![pipeline status](https://gitlab.com/0x28/pile-lang/badges/master/pipeline.svg)](https://gitlab.com/0x28/pile-lang/pipelines)
[![coverage report](https://gitlab.com/0x28/pile-lang/badges/master/coverage.svg)](https://0x28.gitlab.io/pile-lang/)

# pile lang

A simple stack-oriented toy programming language.

# Examples

## Fibonacci
```
{fib}
```

## Factorial
```
{fact}
```
"
    };
}

fn main() -> std::io::Result<()> {
    fs::write(
        "README.md",
        format!(
            readme!(),
            fib = fs::read_to_string(
                "src/interpret/file_test/proj_fibonacci/fibonacci.pile"
            )?,
            fact = fs::read_to_string(
                "src/interpret/file_test/proj_factorial/factorial.pile"
            )?
        ),
    )?;

    Ok(())
}
