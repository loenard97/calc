<div align="center">

# calc
A cli calculator using reverse polish notation

![](https://img.shields.io/github/last-commit/loenard97/calc?&style=for-the-badge&color=F74C00)
![](https://img.shields.io/github/repo-size/loenard97/calc?&style=for-the-badge&color=F74C00)

</div>


## ▶️ Usage
In [reverse Polish notation](https://en.wikipedia.org/wiki/Reverse_Polish_notation) operators are appended after their operands, eliminating the need for brackets.
```sh
$ calc 3 2 +
5
```

Accepts command line arguments, as well as stdin:
```sh
$ echo "3 2 +" | calc
5
```


### Supported operators
> **Note**
> The naming of all operators is case insensitive

Constants:
 - `pi` Archimedes’ constant π
 - `e` Euler's constant e

Operators that take one input value:
 - `ln` Loagaritm base e
 - `log2` Logarithm base 2
 - `log10` Logarithm base 10
 - `sin` Sine
 - `cos` Cosine
 - `tan` Tangent

Operators that take two input values:
 - `+` Addition
 - `-` Subtraction
 - `.` Multiplication (Note that `.` is used for multiplication, because `*` represents a wildcard in bash)
 - `/` Division
 - `log` Logarithm with given base


## ⚠️ Invalid expressions
Invalid tokens are ignored for computation, but give a warning.
```sh
$ calc 3 __ 7 + ?? 6 /
Warning: expression contains 2 invalid tokens
 │ calc 3 __ 7 + ?? 6 /
 └────────^^─────^^────
```

An expression can not be computed if an operator does not have enough values on the stack to complete its operation...
```sh
$ calc 3 +
Error: not enough values on stack to apply operator
 │ calc 3 +
 └────────^
```

...or if at the end of the computation more than one value remains on the stack.
```sh
$ calc 3 5 7 +
Error: stack contains more than one value after applying all operators
```
