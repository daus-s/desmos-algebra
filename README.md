# Project
## Algebra System

### Daus Carmichael

#### Requirements

Linux System
Rust installed - 1.66
BNFC installed - 2.9.1
GHC installed - 8.10.7

Setup...
`
build
`


To generate all permuations of an equation...
`
./solve <equation>
`

To evaluate an expression without variables...

`
echo "<rhs>" | ./Desmos
`

<rhs> is an expression wihtout variables. the right hand side of equations with 1 variable when evaluated.

Examples:

Solve this equation for x: 56=5\cdot(2^{x})
`
$ ./solve "56=5\cdot(2^{x})"


> x=\frac{(\ln\frac{(56)}{(5)})}{(\ln2)}

`
Find the value of x from the previous example
<rhs> = \frac{(\ln\frac{(56)}{(5)})}{(\ln2)} --this errors
***ADD PARENTHESES***
<rhs> = \frac{(\ln(\frac{(56)}{(5)}))}{(\ln2)}
		  ^                 ^
`
$ echo "<rhs>" | ./Desmos


> 3.4854268271702415

`
>
To verify the value copy the output from solve into desmos.

Desmos.com:
3.485426827

As mentioned in the limitations, parentheses proved to be an issue...
In this example, it demonstrates how the parentheses may need to be inserted in when a expression is to be evaluated still inside another function.