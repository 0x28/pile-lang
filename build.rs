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

## Calculating π
```
{pi}
```
"
    };
}

fn use_example(file: &str) -> std::io::Result<String> {
    println!("cargo:rerun-if-changed={}", file);
    fs::read_to_string(file)
}

fn main() -> std::io::Result<()> {
    fs::write(
        "README.md",
        format!(
            readme!(),
            fib = use_example(
                "src/interpret/file_test/proj_fibonacci/fibonacci.pile"
            )?,
            fact = use_example(
                "src/interpret/file_test/proj_factorial/factorial.pile"
            )?,
            pi = use_example(
                "src/examples/pi.pile"
            )?
        ),
    )?;

    Ok(())
}
