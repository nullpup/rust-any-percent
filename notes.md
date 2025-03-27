# Notes

## Ch 1

- Macros!!!
- `$ cargo check` to check w/o compiling

## Ch 3

- Some keywords are reserved for future functionality.
- Two's complement negation (first bit is sign):
  1. Flip all bits
  2. Add 1, ignoring overflow.
- Integers range from:
  - `-(2^{n-1}) to 2^{n-1}`
- Unsigned range from:
  - `0 to 2^{n-1}`
- Numeric type suffix:
  - `69usize`
- Numeric literal visual separator:
  - `1_2345` same as `12345`
- Integers default to `i32`
- Integer overflows panic in debug but not release build.
  - Two's complement wrapping in release. Relying on implicit wrapping
  is ***considered an error.***
  - Handle explicitly with specific methods!
- IEEE-754 floating-point numbers
- `char` is Unicode Scalar Value... but characters aren't a Unicode
  concept.

### Tuples

- PATTERN MATCHING!!!!! YAY `^w^`
- Destructuring: breaking a single tuple into parts.
- Index via period and index:
  - `tuple_name.69`
- `unit`: tuple without values
  - `()`
  - Empty value or empty return type

### Arrays

- Example of array type:
  - `[usize; 69]`
  - Type of each element; array size
- Initialize array with same value for each element:
  - `[420; 69]`
  - Value; array size
- There is both compile time and runtime out of bounds checking.
  - Failed runtime check results in panic.

### Functions

- `snake_case`
- Functions defined later in code are still in scope.
- Yo! Parameter vs arguments stuff reminds me of SICP.
- Parameters variables in a function signature.
- Arguments are the concrete values provided to a function.
- You ***MUST*** specify parameter types
  - Reason: reduce need to annotate elsewhere (for compiler)
  - Reason: more helpful error messages

So far, Rust and its intro book feel like if functional programming
people designed a C-like language.

Seems that Rust macros are inspired by Scheme macros. Looking this up,
woa: "Lisp macro are full-blown Lisp procedures [....] Instead of
text, they get lists that represent the bits of code that you want
to change."[^lisp-macros]

[^lisp-macros]: <https://en.wikibooks.org/wiki/Scheme_Programming/Macros>

- "Function bodies are[...] a series of statements optionally ending in
  an expression."[^function-bodies]
  - Q: What type is a function with an empty body?
    - Removed by compiler.
    - Seems that `main()` remains in symbol table according to
    `objdump`?

[^function-bodies]: <https://doc.rust-lang.org/stable/book/ch03-03-how-functions-work.html#statements-and-expressions>

- Statements perform an action and do not return a value.
- Expressions evaluate to a value.
- Function definitions are statements.
- Function and macro calls are expressions.

Reminds me of OCaml.

- Scope blocks (code surrounded by curly brackets) are expressions.
- You can form a statement by ending an expression with a semicolon.
- You must declare return value type after an arrow:

```rust
fn example() -> usize {
 'a';
 12345
}
```

- `if` conditions must be `bool`. No automatic conversion.
- Multiple conditions by placing `if` expression after `else`. I.e.
`else if`.
  - Prefer `match` for complex branching.

- Pass `loop` return value to `break`:

```rust
loop {
 break 0;
}
```

- You can `return` within a `loop` to exit the current function.
- By default,`break` and `continue` apply to the innermost `loop` that
they are in.
- Use loop labels when nesting loops to specify what loop a `break` or
`continue` applies to. Loop labels begin with a single quote:

```rust
'my_label: loop {
 println!("Outer");
 loop {
  loop {
   println!("Innermost")
   break;
  }
  break `my_label;
  println!("I don't print!")
 }
 println!("I also don't print!")
}
```

- Indexing collections is prone to error (e.g. panic on out of bounds).
  - Must ensure that array *and* bounds guard condition are both
 appropriate.
  - The bounds check also slows code!
- Instead, use `for` loop:

```rust
let ass = [8,2,10,7];
for dick in ass {
 println("{dick}in owo");
}
```

- Want loop indices or something similar? Use a range!

```rust
let sum = 0;
for i in 1..69 {
  sum += i;
}
println!("{sum}");

for i in (1..69).rev() {
  sum -= i;
}
println!("{sum}");
```

## Chapter 2

- Prelude: set of std library items in scope by default
- `String` type is growable.

> [!NOTE]
> The book authors call `String::new` an *associated function* rather
> than a method. I'm a bit reminded of OCaml's module system, which
> allows you to define and utilize related definitions. Interestingly,
> the authors seem like they are referring to some kind of string
> module while also referring to a type of the same name. Will have to
> look into this.
