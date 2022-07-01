## Integer Types

An _integer_ is a number without a fractional component. We used one integer type in Lesson 2, the `u32` type. This type declaration indicates that the value it’s associated with should be an unsigned integer (signed integer types start with `i`, instead of `u`) that takes up 32 bits of space. Table below shows the built-in integer types in Rust. Each variant in the Signed and Unsigned columns (for example, i16) can be used to declare the type of an integer value.

| Length  | Signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

##### Table: Integer Types in Rust

Each variant can be either signed or unsigned and has an explicit size. _Signed_ and _unsigned_ refer to whether it’s possible for the number to be negative or positive—in other words, whether the number needs to have a sign with it (signed) or whether it will only ever be positive and can therefore be represented without a sign (unsigned). It’s like writing numbers on paper: when the sign matters, a number is shown with a plus sign or a minus sign; however, when it’s safe to assume the number is positive, it’s shown with no sign. Signed numbers are stored using [two’s complement](https://en.wikipedia.org/wiki/Two%27s_complement) representation.

Each signed variant can store numbers from -($2^{n-1}$) to $2^{n - 1}$-1 inclusive, where _n_ is the number of bits that variant uses. So an `i8` can store numbers from -($2^7$) to $2^7$-1, which equals -128 to 127. Unsigned variants can store numbers from 0 to $2^n$-1, so a `u8` can store numbers from 0 to $2^8$-1, which equals 0 to 255.

Additionally, the `isize` and `usize` types depend on the kind of computer your program is running on: 64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.

You can write integer literals in any of the forms shown in Table below. Note that all number literals except the byte literal allow a type suffix, such as `57u8`, and `_` as a visual separator, such as `1_000`.

| Number literals | 	Example    |
|-----------------|-------------|
| Decimal         | 98_222      |
| Hex 	           | 0xff        |
| Octal           | 0o77        |
| Binary 	        | 0b1111_0000 |
| Byte (u8 only)  | 	b'A'       |

##### Table: Integer Literals in Rust

So how do you know which type of integer to use? If you’re unsure, Rust’s defaults are generally good choices, and integer types default to `i32`: this type is generally the fastest, even on 64-bit systems. The primary situation in which you’d use `isize` or `usize` is when indexing some sort of collection.

