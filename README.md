# Chalk
## Simple LL Recursive Descent Parser Interpreted Calculator :)

`Chalk` is my first true dive into creating an interpreter, and as such is a simple starting point. While doing my Discrete Math homework this semester, I've wanted a quick calculator I can pull up to evaluate quick expressions in my terminal and had been running Python's repl beforehand. As such, I decided to cook up this tiny project as an alternative. 

Chalk includes the `chalk-core` crate that handles tokenization and parsing, alongside `chalk` which is a CLI tool with two modes of execution:

1. Running chalk with an immediate expression, such as `chalk 1 + 1` will evaluate the expression and exit, printing it to standard out

2. If no immediate expression is provided, `chalk` enters REPL mode and will evaluate expressions until the program is exited

```bash
Welcome to the Chalk Repl

--> 1+1
2
--> |86^2*-1|
7396
--> 

```
