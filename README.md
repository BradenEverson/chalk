# Chalk 📋➕
## Simple LL Recursive Descent Parser Interpreted Calculator :)

`chalk` is my first true dive into creating an interpreter, and as such is a simple starting point. While doing my Discrete Math homework this semester, I've wanted a quick calculator I can pull up to evaluate quick expressions in my terminal and had been running Python's REPL beforehand. As such, I decided to cook up this tiny project as an alternative. 

Chalk includes the `chalk-core` crate that handles tokenization and parsing, alongside `chalk` which is a CLI tool with two modes of execution:

1. Running chalk with an immediate expression, such as `chalk 1 + 1` will evaluate the expression and exit, printing it to standard out

2. If no immediate expression is provided, `chalk` enters REPL mode and will evaluate expressions until the program is exited

```bash
Welcome to the Chalk Repl

->> 1 + 1
`1 + 1` = 2

->> lcm(5!, 12!)
`lcm(5!, 12!)` = 479001600

->> floor(1 - 1/100000) == 0
`floor(1 - 1 / 100000) == 0` = true

->> 10^2 >= 101 || 5! == 120
`10 ^ 2 >= 101 || 5! == 120` = true

->> x = 5
`x = 5` = 5

->> y = x^2
`y = x ^ 2` = 25

->> x = 2
`x = 2` = 2

->> y
`y` = 4

->> 
```

`chalk` currently has support for:
- Addition
- Subtraction
- Multiplication
- Division
- Exponentiation
- Operator precedence
- Parenthesis and Absolute value bars 
- Unary negation
- Factorial!
- `gcd` and `lcm`
- `floor` and `ceil`
- Trig functions `sin`, `cos` and `tan`, and their inverses `asin`, `acos` and `atan`
- Extremely simple runtime types system
    - Different operations including `factorial`, `floor`, `ceil`, etc require integers only (and in some cases unsigned only)
    - Enum based type system allows for these assertions and lossless coercions if possible
- Logical operations including Equality and Inequality checks, >, <, >= and <=

- Variable assignment operations with lazy evaluation

If there are any other operations or features you'd like to see, please feel free to make a Pull Request or create an issue :)
