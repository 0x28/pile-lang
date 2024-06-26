[![License: GPL](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![pipeline status](https://gitlab.com/0x28/pile-lang/badges/master/pipeline.svg)](https://gitlab.com/0x28/pile-lang/pipelines)
[![coverage report](https://gitlab.com/0x28/pile-lang/badges/master/coverage.svg)](https://0x28.gitlab.io/pile-lang/)

# pile lang

A simple stack-oriented toy programming language.

# Examples

## Fibonacci (iterative)
```
let [number current next]
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

## Fibonacci (recursive)
```
let [n]
    -> n
    begin
        n 1 - fib
        n 2 - fib
        +
    end
    begin
        n
    end
    n 2 >=
    if
end -> fib

0 fib 0 = assert
1 fib 1 = assert
2 fib 1 = assert
3 fib 2 = assert
4 fib 3 = assert
10 fib 55 = assert

```

## Factorial
```
let [n result]
    -> n
    1 -> result
    begin
        n result * -> result
        n 1 - -> n
    end
    n
    dotimes
    result
end
-> fact

```

## Calculating π
```
### leibniz formular for pi ###
let [n result divisor sign]
    -> n      # number of iterations
    1.0 -> result
    3.0 -> divisor
    -1.0 -> sign
    begin
        result sign 1.0 divisor / * + -> result
        2.0 divisor + -> divisor
        -1.0 sign * -> sign
    end
    n
    dotimes
    4.0 result *
end -> calc_pi

1000 calc_pi -> pi

pi 3.14 > assert
pi 3.15 < assert

```

## Quine
```
"\"" -> q "\\" -> b q b q q q b b q q "{}{}{}{} -> q {}{}{}{} -> b q b q q q b b q q {}{}{} dup q swap format print" dup q swap format print
```
