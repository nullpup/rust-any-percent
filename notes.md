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

Tuples:

- PATTERN MATCHING!!!!! YAY `^w^`
- Destructuring: breaking a single tuple into parts.
- Index via period and index:
	- `tuple_name.69`
- `unit`: tuple without values
	- `()`
	- Empty value or empty return type
