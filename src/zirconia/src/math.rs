use crate::prelude::*;
use core::range::{self, Range};

use num_traits::{Float, FloatConst};

/// Performs a basic sigmoid operation on `x`, outputing between `0.0` and `1.0`.
pub fn sigmoid<F: Float + FloatConst>(x: F) -> F {
  F::one() / (F::one() + F::E().powf(-x))
}

/// Performs a sigmoid that stretches (or squishes) its output to fit `range` (as a simple multiplication of the basic sigmoid range of `0.0` to `1.0`).
///
/// # Parameters
/// - `x`: The input to the sigmoid
/// - `range`: The output range
#[instrument(skip_all, level = Level::TRACE)]
pub fn sigmoid_range<F: Float + FloatConst>(x: F, range: range::Range<F>) -> F {
  let sigmoid_result = sigmoid(x);
  let range_interval_size = (range.end - range.start).abs();

  sigmoid_result * range_interval_size + range.start
}
