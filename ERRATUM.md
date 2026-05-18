# 1.5 Bitcoin's Elliptic Curve
## 1 Update the FieldElement struct
- constants.rs: it uses the old crate lazy_static but since Rust 1.70 the recommended away to have static consts lazy initialized is std::sync::OnceLock, see https://crates.io/crates/lazy_static#standard-library
- On instructions 1. Update the FieldElement struct, it asks "Add the pow function" but Pow trait is already implemented stubs
- The 'sqrt' ‘shortcut’ formula is misleading (wrong parentheses), also it is already implemented stubs
- Optionally, add a fmt::Display implementation, it is already implemented in stubs

## 3 Update the Point struct
- Methods a() and b() returns &FieldElement, but tests expect FieldElement

## Overall issues
- field_tests.rs expect subtraction trait implemented for FieldElement, but there is only &FieldElement


# Improvements
- Whenc copying functions frim stubs.rs, instead of just having a commented function body, put `todo!()` so codes compiles and there is no IDE errors
