[![License: GPL](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![pipeline status](https://gitlab.com/0x28/pile-lang/badges/master/pipeline.svg)](https://gitlab.com/0x28/pile-lang/pipelines)
[![coverage report](https://gitlab.com/0x28/pile-lang/badges/master/coverage.svg)](https://0x28.gitlab.io/pile-lang/)

# pile lang

A simple stack-oriented toy programming language.

# Examples

## Fibonacci
```
begin
  -> number # arg1 = number
  0 -> current
  1 -> next
  begin
    next
    current next + -> next
    -> current
  end
  number
  dotimes

  current
end
-> fib

```

## Factorial
```
begin
  -> n
  1 -> result
  begin
    n result * -> result
    n 1 - -> n
  end
  begin
    n 0 >
  end
  while
  result
end
-> fact

```
