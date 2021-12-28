/***********************************************************************************************************************
 * Copyright (c) 2020 by the authors
 * 
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-mmio-register/||VERSION||")]
#![no_std]

//! # RusPiRo MMIO Register
//!
//! This crate provides a macro to conveniently define memory mapped I/O registers.
//!
//! ```no_run
//! # use ruspiro_mmio_register::*;
//! define_mmio_register!(
//!     /// FOO Register with read/write access, 32 bit wide and mapped at memory
//!     /// address 0x3F20_0000
//!     FOO<ReadWrite<u32>@(0x3F20_0000)> {
//!         /// This register provides a field BAR at offset 0 covering 1 Bit
//!         BAR OFFSET(0),
//!         /// There is another field BAZ at offset 1 covering 3 Bits
//!         BAZ OFFSET(1) BITS(3),
//!         /// The third field BAL also has specific predefined values
//!         BAL OFFSET(4) BITS(2) [
//!             /// Field Value 1
//!             VAL1 = 0b01,
//!             /// Field Value 2
//!             VAL2 = 0b10
//!         ]
//!     }
//! );
//! 
//! fn main() {
//!     // write a specific value to a field of the register
//!     FOO::Register.write_value( FOO::BAL::VAL1 );
//! 
//!     // combine two field values with logical OR
//!     FOO::Register.write_value( FOO::BAL::VAL1 | FOO::BAL::VAL2 );
//! 
//!     // if there is no field defined for the MMIO register or raw value storage
//!     // is preffered the raw value could be written
//!     FOO::Register.write_value(FOO::BAZ::with_value(0b101));
//!     FOO::Register.write(FOO::BAZ, 0b101);
//!     FOO::Register.set(0x1F);
//! 
//!     // reading from the MMIO register works in a simmilar way
//!     let baz_val = FOO::Register.read(FOO::BAL); // return 0b01 or 0b10 eg.
//!     let baz_field = FOO::Register.read_value(FOO::BAL); // returns a FieldValue
//!     let raw_val = FOO::Register.get();
//! }
//! ```
//!

use core::ptr::{read_volatile, write_volatile};

pub use ruspiro_register::*;
pub mod macros;

/// This struct allows read only access to a register.
#[derive(Clone, Debug)]
pub struct ReadOnly<T: RegisterType> {
    ptr: *mut T, // base address for the register
}

/// This struct allows write only access to a register.
#[derive(Clone, Debug)]
pub struct WriteOnly<T: RegisterType> {
    ptr: *mut T, // base address for the register
}

/// This struct allows read/write access to a register.
#[derive(Clone, Debug)]
pub struct ReadWrite<T: RegisterType> {
    ptr: *mut T, // base address for the register
}

/*************** internal used macros to ease implementation ******************/
macro_rules! registernew_impl {
    ($t:ty) => {
        /// Create a new instance of the register access struct.
        #[allow(dead_code)]
        pub const fn new(addr: usize) -> Self {
            Self {
                ptr: addr as *mut $t,
            }
        }
    };
}

macro_rules! registerget_impl {
    ($t:ty) => {
        /// Read raw content of a register.
        #[inline]
        #[allow(dead_code)]
        pub fn get(&self) -> $t {
            unsafe { read_volatile(self.ptr) }
        }

        /// Read the value of a specific register field
        #[inline]
        #[allow(dead_code)]
        pub fn read(&self, field: RegisterField<$t>) -> $t {
            let val = self.get();
            (val & field.mask() ) >> field.shift() 
        }

        /// Read the value of the register into a RegisterFieldValue structure
        #[inline]
        #[allow(dead_code)]
        pub fn read_value(&self, field: RegisterField<$t>) -> RegisterFieldValue<$t> {
            RegisterFieldValue::<$t>::new(field, self.read(field))
        }
    };
}

macro_rules! registerset_impl {
    ($t:ty) => {
        /// Write raw content value to the register.
        #[inline]
        #[allow(dead_code)]
        pub fn set(&self, value: $t) {
            unsafe { write_volatile(self.ptr, value) }
        }

        /// Write the value of a specific register field, this will set all bits not coverd by this field to 0 !
        #[inline]
        #[allow(dead_code)]
        pub fn write(&self, field: RegisterField<$t>, value: $t) {
            let val = (value << field.shift()) & field.mask();
            self.set(val);
        }

        /// Write the value of a given RegisterFieldValue to the register, this will set all bits not coverd by this 
        /// field to 0 !
        #[inline]
        #[allow(dead_code)]
        pub fn write_value(&self, fieldvalue: RegisterFieldValue<$t>) {
            self.set(fieldvalue.raw_value());
        }
    };
}

macro_rules! readonly_impl {
    ($( $t:ty ),*) => { $(
        impl ReadOnly<$t> {
            registernew_impl!($t);
            registerget_impl!($t);
        }
    )* };
}
readonly_impl![u8, u16, u32, u64];

macro_rules! writeonly_impl {
    ($( $t:ty ),*) => { $(
        impl WriteOnly<$t> {
            registernew_impl!($t);
            registerset_impl!($t);
        }
    )* };
}
writeonly_impl![u8, u16, u32, u64];

macro_rules! readwrite_impl {
    ($( $t:ty ),*) => { $(
        impl ReadWrite<$t> {
            registernew_impl!($t);
            registerget_impl!($t);
            registerset_impl!($t);

            /// Udate a register field with a given value. The bits outside of this field remains untouched.
            /// The function returns the register raw value set has been set with this update
            #[inline]
            #[allow(dead_code)]
            pub fn modify(&self, field: RegisterField<$t>, value: $t) -> $t {
                let old_val = self.get();
                let raw_val = (value << field.shift()) & field.mask();
                let new_val = (old_val & !field.mask()) | raw_val;

                self.set(new_val);
                new_val 
            }

            /// Udate a register field with a given register field value. The bits outside of this field remains 
            /// untouched. The function returns the register raw value set has been set with this update
            #[inline]
            #[allow(dead_code)]
            pub fn modify_value(&self, fieldvalue: RegisterFieldValue<$t>) -> $t {
                let old_val = self.get();
                let raw_val = fieldvalue.raw_value() & fieldvalue.mask();
                let new_val = (old_val & !fieldvalue.mask()) | raw_val;

                self.set(new_val);
                new_val
            }
        }
    )* };
}
readwrite_impl![u8, u16, u32, u64];
