# Number types in rust
- Unsigned Integers: u8, u16, u32, u64, u128
- Signed Integers: i8, i16, i32, i64, i128
- Floating Point Numbers: f32, f64

# Integer Types in Rust
| length  | signed | Unsigned |
|---------|--------|----------|
| 8-bit   | i8     | u8       |
| 16-bit  | i16    | u16      |
| 32-bit  | i32    | u32      |
| 64-bit  | i64    | u64      |
| 128-bit | i128   | u128     |
| arch    | isize  | usize    |

# Integer Literals in Rust
| Number Literals | Example     |
|-----------------|-------------|
| Decimal         | 98222       |
| Hex             | 0xff        |
| Octal           | O077        |
| Binary          | 0b1111_0000 |
| Byte (u8 only)  | b'A'        |

# Turbofish
- `S::<T>`    --    Turbofish STD call site type disambiguation, e.g. `f::<u32>()`
