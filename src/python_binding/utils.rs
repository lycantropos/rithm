use crate::constants::UNDEFINED_DIVISION_ERROR_MESSAGE;
use pyo3::basic::CompareOp;
use pyo3::exceptions::{
    PyMemoryError, PyOverflowError, PyValueError, PyZeroDivisionError,
};
use pyo3::PyResult;
use traiter::numbers::{
    CheckedDivEuclid, CheckedDivRemEuclid, CheckedRemEuclid, CheckedShl,
    CheckedShr,
};

#[cfg(target_arch = "x86")]
pub(super) const HASH_BITS: usize = 31;
#[cfg(not(target_arch = "x86"))]
pub(super) const HASH_BITS: usize = 61usize;
pub(super) const HASH_INF: isize = 314_159isize;
pub(super) const HASH_MODULUS: usize = (1usize << HASH_BITS) - 1usize;

#[inline]
pub(super) fn try_divmod<
    Dividend: CheckedDivRemEuclid<Divisor, Output = Option<(Quotient, Remainder)>>,
    Divisor,
    Quotient,
    Remainder,
>(
    dividend: Dividend,
    divisor: Divisor,
) -> PyResult<(Quotient, Remainder)> {
    match dividend.checked_div_rem_euclid(divisor) {
        Some((quotient, remainder)) => Ok((quotient, remainder)),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
pub(super) fn try_floordiv<
    Dividend: CheckedDivEuclid<Divisor, Output = Option<Value>>,
    Divisor,
    Value,
>(
    dividend: Dividend,
    divisor: Divisor,
) -> PyResult<Value> {
    match dividend.checked_div_euclid(divisor) {
        Some(result) => Ok(result),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
pub(super) fn try_lshift<
    Base: CheckedShl<Shift, Output = Result<Value, crate::big_int::ShlError>>,
    Shift,
    Value,
>(
    base: Base,
    shift: Shift,
) -> PyResult<Value> {
    base.checked_shl(shift).map_err(|error| match error {
        crate::big_int::ShlError::NegativeShift => {
            PyValueError::new_err(error.to_string())
        }
        crate::big_int::ShlError::OutOfMemory => {
            PyMemoryError::new_err(error.to_string())
        }
        crate::big_int::ShlError::TooLarge => {
            PyOverflowError::new_err(error.to_string())
        }
    })
}

#[inline]
pub(super) fn try_mod<
    Dividend: CheckedRemEuclid<Divisor, Output = Option<Value>>,
    Divisor,
    Value,
>(
    dividend: Dividend,
    divisor: Divisor,
) -> PyResult<Value> {
    match dividend.checked_rem_euclid(divisor) {
        Some(result) => Ok(result),
        None => Err(PyZeroDivisionError::new_err(
            UNDEFINED_DIVISION_ERROR_MESSAGE,
        )),
    }
}

#[inline]
pub(super) fn try_rshift<
    Base: CheckedShr<Shift, Output = Result<Value, crate::big_int::ShrError>>,
    Shift,
    Value,
>(
    base: Base,
    shift: Shift,
) -> PyResult<Value> {
    base.checked_shr(shift).map_err(|error| match error {
        crate::big_int::ShrError::NegativeShift => {
            PyValueError::new_err(error.to_string())
        }
    })
}

pub(super) fn compare<T: PartialOrd<U>, U>(
    left: &T,
    right: &U,
    op: CompareOp,
) -> bool {
    match op {
        CompareOp::Eq => left == right,
        CompareOp::Ge => left >= right,
        CompareOp::Gt => left > right,
        CompareOp::Le => left <= right,
        CompareOp::Lt => left < right,
        CompareOp::Ne => left != right,
    }
}
