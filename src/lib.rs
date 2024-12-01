/*
 * SPDX-FileCopyrightText: 2024 Sebastiano Vigna
 *
 * SPDX-License-Identifier: Apache-2.0 OR LGPL-2.1-or-later
 */

#![doc = include_str!("../README.md")]

use core::ops::ControlFlow;

/// A [`ControlFlow::Break`] type that can never happen.
///
/// This enum has the same purpose of [`core::convert::Infallible`], but it is
/// specific to control flows. It should be ultimately replaced by the
/// [`!`] type if it becomes stable.
pub enum Unbreakable {}

/// A trait to extract continuation values from control flows that
/// never break.
///
/// This trait is useful to avoid the need to specify the error type when
/// calling a visit method that cannot be interrupted. It forces a
/// [`ControlFlow`] to have `Break` variant
/// [`Infallible`](std::convert::Infallible).
pub trait NoBreak {
    /// The type of the value in the [`Continue`](ControlFlow::Continue)
    /// variant of the control flow.
    type Continue;
    /// Returns the continue value of the control flow.
    fn continue_value_no_break(self) -> Self::Continue;
}

impl<C> NoBreak for ControlFlow<Unbreakable, C> {
    type Continue = C;

    #[inline(always)]
    fn continue_value_no_break(self) -> C {
        unsafe {
            // SAFETY: If E is Unbreakable, continue_value must be Some
            self.continue_value().unwrap_unchecked()
        }
    }
}
