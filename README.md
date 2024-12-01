# Typesafe Extraction of Continuation Values from Unbreakable Control Flows

This crate provides a convenience trait [`NoBreak`] adding a method
[`continue_value_no_break`] to [`ControlFlow`] types in which the type of the
value contained in the [break variant] is [`Unbreakable`]. The purpose is
identical to that of the [`unwrap_infallible`] crate, but for control flows.

[`Unbreakable`] is a marker type similar to [`Infallible`] (it is an empty
enum), but with a different name as breaks in a control flow are not
thought of as errors. It should ultimately be replaced with the
[never type].

Using [`continue_value_no_break`] on a [`ControlFlow`] forces a typesafe,
compile-time check that the control flow will never break, and extracts the
value from the only possible variant. In particular, type inference should make
it possible to use expressions such as `ControlFlow::Continue(value)`
in the code without specifying the break type.

## Examples

```rust
use no_break::NoBreak;
use std::ops::ControlFlow;

fn never_breaks<B>() -> ControlFlow<B, usize> {
    // Note that we do not need to specify B
    ControlFlow::Continue(5)
}
fn main() {
 // Type inference takes care of setting B to Unbreakable
    println!("{}", never_breaks().continue_value_no_break());
}
 ```

[`NoBreak`]: <https://docs.rs/no-break/latest/no_break/trait.NoBreak.html>
[`ControlFlow`]: https://doc.rust-lang.org/std/ops/enum.ControlFlow.html
[`Unbreakable`]: <https://docs.rs/no-break/latest/no_break/trait.Unbreakable.html>
[`unwrap_infallible`]: <https://crates.io/crates/unwrap-infallible>
[`Infallible`]: <https://doc.rust-lang.org/std/convert/enum.Infallible.html>
[`continue_value_no_break`]: <https://docs.rs/no-break/latest/no_break/trait.NoBreak.html#tymethod.continue_value_no_break>
[never type]: <https://doc.rust-lang.org/std/primitive.never.html>
[break variant]: <https://doc.rust-lang.org/std/ops/enum.ControlFlow.html#variant.Break>
