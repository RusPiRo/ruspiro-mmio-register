# RusPiRo MMIO Register

The crate provides macros to conviniently define memory mapped I/O (MMIO) registers.

![CI](https://github.com/RusPiRo/ruspiro-mmio-register/workflows/CI/badge.svg?branch=development)
[![Latest Version](https://img.shields.io/crates/v/ruspiro-mmio-register.svg)](https://crates.io/crates/ruspiro-mmio-register)
[![Documentation](https://docs.rs/ruspiro-mmio-register/badge.svg)](https://docs.rs/ruspiro-mmio-register)
[![License](https://img.shields.io/crates/l/ruspiro-mmio-register.svg)](https://github.com/RusPiRo/ruspiro-mmio-register#license)

## Usage

To use this crate simply add the dependency to your ``Cargo.toml`` file:

```toml
[dependencies]
ruspiro-mmio-register = "||VERSION||"
```

The definition of MMIO registers is straight forward using the provided `define_mmio_register!` macro like so:

```rust
use ruspiro_mmio_register::*;

define_mmio_register!(
    /// FOO Register with read/write access, 32 bit wide and mapped at memory
    /// address 0x3F20_0000
    FOO<ReadWrite<u32>@(0x3F20_0000)> {
        /// This register provides a field BAR at offset 0 covering 1 Bit
        BAR OFFSET(0),
        /// There is another field BAZ at offset 1 covering 3 Bits
        BAZ OFFSET(1) BITS(3),
        /// The third field BAL also has specific predefined values
        BAL OFFSET(4) BITS(2) [
            /// Field Value 1
            VAL1 = 0b01,
            /// Field Value 2
            VAL2 = 0b10
        ]
    }
);
```

Once the register is defined it can be used to read data from or write data to, depending on its type (ReadOnly, WriteOnly, ReadWrite).

```rust
fn main() {
    // write a specific value to a field of the register
    FOO::Register.write_value( FOO::BAL::VAL1 );

    // combine two field values with logical OR
    FOO::Register.write_value( FOO::BAL::VAL1 | FOO::BAL::VAL2 );

    // if there is no field defined for the MMIO register or raw value storage
    // is preffered the raw value could be written
    FOO::Register.write_value(FOO::BAZ::with_value(0b101));
    FOO::Register.write(FOO::BAZ, 0b101);
    FOO::Register.set(0x1F);

    // reading from the MMIO register works in a simmilar way
    let baz_val = FOO::Register.read(FOO::BAL); // return 0b01 or 0b10 eg.
    let baz_field = FOO::Register.read_value(FOO::BAL); // returns a FieldValue
    let raw_val = FOO::Register.get();
}
```

## License

Licensed under Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0) or MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)) at your choice.
