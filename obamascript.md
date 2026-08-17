# ObamaScript Language Reference

ObamaScript is a small programming language written in Rust.

It is designed to be simple, readable, and, most importantly, capable of turning code written on paper into executable code through OCR.

---

## 1. Comments

Comments begin with `//`.

```obama
// This is a comment

set x to 10 // This is also a comment
```

Comments are ignored when the program is executed.

---

## 2. Values

ObamaScript currently supports several types of values.

### Numbers

Numbers can be integers or decimal numbers.

```obama
set age to 13
set pi to 3.14
```

### Strings

Strings are surrounded by double quotes.

```obama
set name to "Vir"
set message to "Hello, World!"
```

### Booleans

ObamaScript uses `yes` and `no` for boolean values.

```obama
set is_cactus to yes
set is_obama to no
```

`yes` represents a true value, while `no` represents a false value.

They can also be passed directly to functions:

```obama
out(yes)
out(no)
```

---

## 3. Variables

Variables are created using `set`.

```obama
set name to "Vir"
set age to 13
set score to 100
```

A variable can then be used by its name:

```obama
out(name)
out(age)
out(score)
```

Variables can also be changed after they have been created.

```obama
set score to 10
change score to 20

out(score)
```

---

## 4. Functions

Functions are called using a name followed by parentheses.

```obama
out("Hello, World!")
```

Arguments are separated by commas:

```obama
out("Hello,", "World!")
```

Functions can accept different types of values:

```obama
out(10)
out("Hello")
out(yes)
```

---

## 5. Input

`in()` waits for the user to enter a value.

```obama
set name to in()

out("Hello,", name)
```

---

## 6. Output

`out()` prints values to the terminal.

```obama
out("Hello, World!")
```

Multiple values can be supplied:

```obama
set name to "Vir"
set age to 13

out("Name:", name)
out("Age:", age)
```

---

## 7. Mathematics

ObamaScript provides built-in mathematical functions.

### `sum()`

Adds numbers together.

```obama
sum(10, 5)
```

Result:

```text
15
```

Multiple numbers can be supplied:

```obama
sum(1, 2, 3, 4, 5)
```

Result:

```text
15
```

### `sub()`

Subtracts subsequent numbers from the first number.

```obama
sub(10, 3)
```

Result:

```text
7
```

Multiple numbers can be supplied:

```obama
sub(20, 5, 3)
```

Result:

```text
12
```

### `mult()`

Multiplies numbers together.

```obama
mult(5, 2)
```

Result:

```text
10
```

Multiple numbers can be supplied:

```obama
mult(2, 3, 4)
```

Result:

```text
24
```

## 8. Nested Function Calls

ObamaScript supports nested function calls.

A function can be passed as an argument to another function.

```obama
out(sum(10, 5))
```

The inner function is evaluated first:

```text
sum(10, 5)
15
```

The result is then passed to `out()`.

Multiple levels of nesting are supported:

```obama
out(sum(10, mult(2, 5)))
```

This evaluates as:

```text
mult(2, 5) → 10
sum(10, 10) → 20
out(20)
```

---

## 9. Example Program

Here is a simple ObamaScript program using variables, input, output, and mathematics:

```obama
out("---------")

set name to in()
set a to 10
set b to 5

out("Hello,", name)
out("a =", a)
out("b =", b)
out("sum =", sum(a, b))
out("difference =", sub(a, b))
out("product =", mult(a, b))
```

Example output:

```text
---------
Vir
Hello,
Vir
a =
10
b =
5
sum =
15
difference =
5
product =
50
division =
2
```

---

## 10. Running ObamaScript

ObamaScript programs use the `.obama` file extension.

For example:

```text
main.obama
```

Run a program using:

```powershell
obama -r main.obama
```

---

## 11. PNG OCR

ObamaScript can also execute code written inside a PNG image.

```powershell
obama -r page.png
```

The image is processed using Tesseract OCR. The resulting text is then passed to the ObamaScript parser.

This allows code to be written on paper, photographed, and executed.

```text
Paper
  ↓
PNG
  ↓
Tesseract OCR
  ↓
ObamaScript
  ↓
Output
```

Tesseract is only required when using PNG input.

---

## 12. Complete Example

A complete program might look like this:

```obama
// Ask for the user's name
out("What is your name?")

set name to in()

// Some numbers
set a to 10
set b to 5

// Display results
out("Hello,", name)
out("a =", a)
out("b =", b)

out("sum =", sum(a, b))
out("difference =", sub(a, b))
out("product =", mult(a, b))
out("division =", div(a, b))
```

---

## 13. Current Features

ObamaScript currently supports:

* Variables
* Numbers
* Strings
* Booleans (`yes` / `no`)
* Comments
* Input
* Output
* Mathematical functions
* Nested function calls
* `.obama` source files
* PNG OCR input
* Windows command-line execution

ObamaScript is still under development, so this reference may change as new features are added.

---
