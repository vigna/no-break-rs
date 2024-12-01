# Extraction of continuation values from unbreakable control flows

This crate provides a convenience trait [`NoBreak`], adding a method
[`continue_value_no_result`] to [`ControlFlow`] types whose `break` variant
is [`Unbreakable`].

## Examples

```rust
use no_break::{NoBreak, Unbreakable};
use std::ops::ControlFlow;

fn never_breaks() -> ControlFlow<Unbreakable, usize> {
    ControlFlow::Continue(5)
}
fn main() {
    println!("{}", never_breaks().continue_value_no_break());
}
 ```

[`NoBreak`]: <https://docs.rs/no-break/latest/no_break/trait.NoBreak.html>
[`ControlFlow`]: https://doc.rust-lang.org/std/result/enum.ControlFlow.html
[`Unbreakable`]: <https://docs.rs/no-break/latest/no_break/trait.Unbreakable.html>
[`continue_value_no_result`]: <https://docs.rs/no-break/latest/no_break/trait.NoBreak.html#tymethod.continue_value_no_result>
