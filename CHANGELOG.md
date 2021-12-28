# Changelog

## :lemon: v0.1.4

- ### :wrench: Maintenance

  - Update to compile with latest nightly and Rust edition 2021
  - Update dependencies
  
## :lemon: v0.1.3

Update the implementation to work with the current Rust version (1.56.0-nightly)

- ### :wrench: Maintenance

  - Remove the `const_fn` feature as this has been removed
  - build the single crate with `aarch64-unknown-none` target

## :peach: v0.1.2

This a maintenance release migrating the build pipeline to github actions.

## :peach: v0.1.1

The `ruspiro-register` crate was refactored to only contained shared structures and macros eble to be re-used by other crates like this one implementing the register functions. So this version utilizes this crate as dependency.

- ### :wrench: Maintenance

  - Maintained the proper dependency and adjusted the tpe and macros usages
  - Adjusted the file headers to reflect copyright as of 2020 and the correct author
  - Add some unit tests for the register access functions (read, write, update)

- ### :book: Documentation

  - Fixed minor documentation flaws

## :apple: v0.1.0

This is the initial version based on the previous *ruspiro-register* crate. It contains the extracted MMIO macro and related stuff.

- ### :bulb: Features
  
  - provide the `define_mmio_register!` macro to define memory mapped registers

- ### :wrench: Maintenance

  - initial Tracis-CI pipeline setup

- ### :book: Documentation
  
  - Initial documentation for crates.io and doc.rs
  