/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/
#![doc(html_root_url = "https://docs.rs/ruspiro-mmio-register/||VERSION||")]
// we require to run with 'std' in unit tests and doc tests to have an allocator in place
#![cfg_attr(not(any(test, doctest)), no_std)]
#![feature(const_fn)]

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
//! ```
//!

use core::cmp::PartialEq;
use core::ops::{BitAnd, BitOr, Not, Shl, Shr};
use core::ptr::{read_volatile, write_volatile};

pub mod macros;

/// This trait is used to describe the register size/length as type specifier. The trait is only implemented for the
/// internal types **u8**, **u16**, **u32** and **u64** to ensure safe register access sizes with compile time checking
pub trait RegisterType:
    Copy
    + Clone
    + PartialEq
    + BitOr<Output = Self>
    + BitAnd<Output = Self>
    + Not<Output = Self>
    + Shl<Self, Output = Self>
    + Shr<Self, Output = Self>
{
}

// Internal macro to ease the assignment of the custom trait to supported register sizes
#[doc(hidden)]
macro_rules! registertype_impl {
    // invoke the macro for a given type t as often as types are provided when invoking the macro
    ($( $t:ty ),*) => ($(
        impl RegisterType for $t { }
    )*)
}

// implement the type trait for specific unsigned types to enable only those register types/sizes
registertype_impl![u8, u16, u32, u64];

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
    () => {
        /// Create a new instance of the register access struct.
        #[allow(dead_code)]
        pub const fn new(addr: u32) -> Self {
            Self {
                ptr: addr as *mut T,
            }
        }
    };
}

macro_rules! registerget_impl {
    () => {
        /// Read raw content of a register.
        #[inline]
        #[allow(dead_code)]
        pub fn get(&self) -> T {
            unsafe { read_volatile(self.ptr) }
        }

        /// Read the value of a specific register field
        #[inline]
        #[allow(dead_code)]
        pub fn read(&self, field: RegisterField<T>) -> T {
            let val = self.get();
            (val >> field.shift) & field.mask
        }

        /// Read the value of the register into a RegisterFieldValue structure
        #[inline]
        #[allow(dead_code)]
        pub fn read_value(&self, field: RegisterField<T>) -> RegisterFieldValue<T> {
            RegisterFieldValue {
                field,
                value: self.read(field),
            }
        }
    };
}

macro_rules! registerset_impl {
    () => {
        /// Write raw content value to the register.
        #[inline]
        #[allow(dead_code)]
        pub fn set(&self, value: T) {
            unsafe { write_volatile(self.ptr, value) }
        }

        /// Write the value of a specific register field
        #[inline]
        #[allow(dead_code)]
        pub fn write(&self, field: RegisterField<T>, value: T) {
            let val = (value & field.mask) << field.shift;
            self.set(val);
        }

        /// Write the value of a given RegisterFieldValue to the register
        #[inline]
        #[allow(dead_code)]
        pub fn write_value(&self, fieldvalue: RegisterFieldValue<T>) {
            self.write(fieldvalue.field, fieldvalue.value);
        }
    };
}

impl<T: RegisterType> ReadOnly<T> {
    registernew_impl!();
    registerget_impl!();
}

impl<T: RegisterType> WriteOnly<T> {
    registernew_impl!();
    registerset_impl!();
}

impl<T: RegisterType> ReadWrite<T> {
    registernew_impl!();
    registerget_impl!();
    registerset_impl!();

    /// Udate a register field with a given value
    #[inline]
    #[allow(dead_code)]
    pub fn modify(&self, field: RegisterField<T>, value: T) -> T {
        let old_val = self.get();
        let new_val =
            (old_val & !(field.mask << field.shift)) | ((value & field.mask) << field.shift);

        self.set(new_val);
        new_val
    }

    #[inline]
    #[allow(dead_code)]
    pub fn modify_value(&self, fieldvalue: RegisterFieldValue<T>) -> RegisterFieldValue<T> {
        let new_val = self.modify(fieldvalue.field, fieldvalue.value);
        RegisterFieldValue {
            field: fieldvalue.field,
            value: new_val,
        }
    }
}

/// Definition of a field contained inside of a register. Each field is defined by a mask and the bit shift value
/// when constructing the field definition the stored mask is already shifted by the shift value
#[derive(Copy, Clone, Debug)]
pub struct RegisterField<T: RegisterType> {
    mask: T,
    shift: T,
}

/// Definition of a specific fieldvalue of a regiser. This structure allows to combine field values with bit operators
/// like ``|`` and ``&`` to build the final value that should be written to a register
#[derive(Copy, Clone, Debug)]
pub struct RegisterFieldValue<T: RegisterType> {
    /// register field definition
    field: RegisterField<T>,
    /// register field value
    value: T,
}

// Internal helper macro to implement:
// - ``RegisterField``struct for all relevant basic types
// - ``FieldValue`` struct for all relevant basic types
// - the operators for ``FieldValue``struct for all relevant basic types
#[doc(hidden)]
macro_rules! registerfield_impl {
    ($($t:ty),*) => ($(
        impl RegisterField<$t> {
            /// Create a new register field definition with the mask and the shift offset for this
            /// mask. The offset is the bit offset this field begins.
            #[inline]
            #[allow(dead_code)]
            pub const fn new(mask: $t, shift: $t) -> RegisterField<$t> {
                Self {
                    mask,
                    shift,
                }
            }

            /// retrieve the current mask of the field shifted to its correct position
            #[inline]
            #[allow(dead_code)]
            pub fn mask(&self) -> $t {
                self.mask.checked_shl(self.shift as u32).unwrap_or(0)
            }

            /// retrieve the current shift of the field
            #[allow(dead_code)]
            #[inline]
            pub fn shift(&self) -> $t {
                self.shift
            }
        }

        impl RegisterFieldValue<$t> {
            /// Create a new fieldvalue based on the field definition and the value given
            #[inline]
            #[allow(dead_code)]
            pub const fn new(field: RegisterField<$t>, value: $t) -> Self {
                RegisterFieldValue {
                    field,
                    value: value & field.mask //<< field.shift
                }
            }

            /// Retrieve the register field value
            #[inline]
            #[allow(dead_code)]
            pub fn value(&self) -> $t {
                self.value //>> self.field.shift()
            }

            /// Retrieve the register field raw value, means the value is returned in it's position
            /// as it appears in the register when read with the field mask applied but not
            /// shifted
            #[inline]
            #[allow(dead_code)]
            pub fn raw_value(&self) -> $t {
                self.value.checked_shl(self.field.shift as u32).unwrap_or(0)
            }

            /// Retrieve the field mask used with this register field. The mask is shifted to it's
            /// corresponding field position
            #[inline]
            #[allow(dead_code)]
            pub fn mask(&self) -> $t {
                self.field.mask()
            }
        }

        impl BitOr for RegisterFieldValue<$t> {
            type Output = RegisterFieldValue<$t>;

            #[inline]
            #[allow(dead_code)]
            fn bitor(self, rhs: RegisterFieldValue<$t>) -> Self {
                let field = RegisterField::<$t>::new( self.field.mask() | rhs.field.mask(), 0);
                RegisterFieldValue {
                    field,
                    value: (self.raw_value() | rhs.raw_value()),
                }
            }
        }

        impl BitAnd for RegisterFieldValue<$t> {
            type Output = RegisterFieldValue<$t>;
            #[inline]
            #[allow(dead_code)]
            fn bitand(self, rhs: RegisterFieldValue<$t>) -> Self {
                let field = RegisterField::<$t>::new( self.field.mask() & rhs.field.mask(), 0);
                RegisterFieldValue {
                    field,
                    value: (self.raw_value() & rhs.raw_value()),
                }
            }
        }
    )*);
}

registerfield_impl![u8, u16, u32, u64];
