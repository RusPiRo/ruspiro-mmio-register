/***********************************************************************************************************************
 * Copyright (c) 2019 by the authors
 *
 * Author: André Borrmann <pspwizard@gmx.de>
 * License: Apache License 2.0 / MIT
 **********************************************************************************************************************/

//! # Register definition macros
//!
//! The macros are used to simplify the definition of system registers as well as MMIO register.
//!

/// Helper macro to define the fields a register may contain of.<br>
/// This is typically part of the register definition and will be applied there. It's not intended for use outside
/// of a register definition.
#[doc(hidden)]
#[macro_export]
macro_rules! register_field {
    ($t:ty, $field:ident, $offset:expr) => {
        #[allow(unused_variables, dead_code)]
        #[doc(hidden)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new(1, $offset);
    };
    ($t:ty, $field:ident, $offset:expr, $bits:expr) => {
        #[allow(unused_variables, dead_code)]
        #[doc(hidden)]
        pub const $field: RegisterField<$t> = RegisterField::<$t>::new((1 << $bits) - 1, $offset);
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! register_field_values {
    ($field:ident, $t:ty, $($($fvdoc:expr)?, $enum:ident = $value:expr),*) => {
        $(
            $(#[doc = $fvdoc])?
            #[allow(unused_variables, dead_code)]
            pub const $enum:RegisterFieldValue::<$t> = RegisterFieldValue::<$t>::new($field, $value);
        )*
    };
}

/// Macro to define a MMIO register with specific defined access mode.<br>
/// The access mode could one of: **ReadOnly**, **WriteOnly**, **ReadWrite**.<br>
/// The register size/width could be one of: **u8**, **u16**, **u32**, **u64**
///
/// # Examples
///
/// Define a simple MMIO register that might only be accessed with it's raw value
/// ```no_run
/// # use ruspiro_mmio_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000+0x10)>
/// );
/// ```
///
/// Define a MMIO register containing a single field at a given offset with 1 bit length.
/// ```no_run
/// # use ruspiro_mmio_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)> {
///         BAR OFFSET(0)
///     }
/// );
/// ```
///
/// Define a MMIO register containing several fields with different offsets and bit length
/// ```no_run
/// # use ruspiro_mmio_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)> {
///         BAR OFFSET(0),
///         BAZ OFFSET(3) BITS(3)
///     }
/// );
/// ```
///
/// Define multiple MMIO register at once
/// ```no_run
/// # use ruspiro_mmio_register::*;
/// define_mmio_register!(
///     FOO<ReadWrite<u32>@(0x3F20_0000)>,
///     BAR<ReadOnly<u32>@(0x3F20_0010)> {
///         BAZ OFFSET(0) BITS(2) [
///             VAL1 = 0b10
///         ]
///     }
/// );
/// ```
///
/// Define a MMIO register where one field has defined specific values to be choosen from when
/// writing to or updating this specific register field
/// ```no_run
/// # use ruspiro_mmio_register::*;
/// define_mmio_register!(
///     /// A MMIO Register FOO
///     /// Defines as Read/Write register
///     FOO<ReadWrite<u32>@(0x3F20_0000)> {
///         /// This is a register field BAR
///         BAR OFFSET(3) BITS(3),
///         /// This is a register field BAZ.
///         /// It contains enum like field value definitions
///         BAZ OFFSET(6) BITS(3) [
///             /// This is a value of the register field
///             VAL1 = 0b000,
///             VAL2 = 0b010
///         ],
///         BAL OFFSET(9) BITS(2) [
///             VAL1 = 0b01,
///             VAL2 = 0b11
///         ]
///     }
/// );
///
/// fn main() {
///     // write a specific value for one field of the MMIO register
///     FOO::Register.write_value(
///         FOO::BAL::VAL1
///     );
///
///     // combine field values of different fields to update the MMIO register
///     FOO::Register.write_value(
///         FOO::BAZ::VAL1 | FOO::BAL::VAL2
///     );
///
///     // write a specific value to a register field that does not provide default/enum values
///     FOO::Register.write_value(
///         FOO::BAR::with_value(0b010)
///     );
/// }
/// ```
#[macro_export]
macro_rules! define_mmio_register {
    // REGISTER_NAME<ReadWrite<TYPE>@ADDRESS> { FIELD OFFSET(num) BITS(num) [ VALUE: val ] }
    ($($(#[doc = $rdoc:expr])* $vis:vis $name:ident<$access:ident<$t:ty>@($addr:expr)> $(
        { $(
                $(#[doc = $fdoc:expr])*
                $field:ident OFFSET($offset:literal) $(BITS($bits:literal))?
                $([$($(#[doc = $fvdoc:expr])* $enum:ident = $value:expr),*])?
        ),* }
    )?),*) => {
        $(
            #[allow(non_snake_case)]
            #[allow(non_upper_case_globals)]
            $vis mod $name {
                #[allow(unused_imports)]
                use $crate::*;
                use super::*;
                $(#[doc = $rdoc])*
                #[allow(unused_variables, dead_code)]
                pub const Register: $access<$t> = $access::<$t>::new($addr);
                $(
                    $(
                        $(#[doc = $fdoc])*
                        $crate::register_field!($t, $field, $offset $(, $bits)?);
                        pub mod $field {
                            use super::*;
                            /// Create a ``RegisterFieldValue`` from the current ``RegisterField``
                            /// of this ``Register`` from a given value
                            #[inline]
                            #[allow(unused_variables, dead_code)]
                            pub const fn with_value(value: $t) -> RegisterFieldValue<$t> {
                                RegisterFieldValue::<$t>::new($field, value)
                            }
                            $(
                                $crate::register_field_values!($field, $t, $($($fvdoc)*, $enum = $value),*);
                            )*
                        }
                    )*
                )*
            }
        )*
    };
}
