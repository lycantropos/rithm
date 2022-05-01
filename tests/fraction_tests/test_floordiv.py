import math

import pytest
from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionOrIntOrBuiltinInt,
                         FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.fractions, strategies.non_zero_fractions_or_ints_or_integers)
def test_basic(dividend: Fraction, divisor: FractionOrIntOrBuiltinInt) -> None:
    result = dividend // divisor

    assert isinstance(result, int)


@given(strategies.fractions, strategies.non_zero_fractions)
def test_value(dividend: Fraction, divisor: Fraction) -> None:
    result = dividend // divisor

    assert (dividend % divisor == 0 and result == dividend / divisor
            or result < dividend / divisor)


@given(strategies.fractions)
def test_division_by_one(dividend: Fraction) -> None:
    result = dividend // 1

    assert result == math.floor(dividend)


@given(strategies.fractions, strategies.non_zero_fractions)
def test_connection_with_mod(dividend: Fraction, divisor: Fraction) -> None:
    result = dividend // divisor

    assert result * divisor + dividend % divisor == dividend


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(dividend: Fraction, divisor_with_builtin: IntWithBuiltin
                      ) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend // divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend // divisor_builtin
    else:
        assert result == dividend // divisor_builtin


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(dividend_with_builtin: FractionWithBuiltin,
                                 divisor_with_builtin: RationalWithBuiltin
                                 ) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend // divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend_builtin // divisor_builtin
    else:
        assert is_equivalent_to_builtin_int(
                result, dividend_builtin // divisor_builtin
        )


@given(strategies.fractions, strategies.zero_fractions_or_ints_or_integers)
def test_zero_divisor(dividend: Fraction, divisor: FractionOrIntOrBuiltinInt
                      ) -> None:
    with pytest.raises(ZeroDivisionError):
        dividend // divisor
