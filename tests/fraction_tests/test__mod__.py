import pytest
from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionOrIntOrBuiltinInt,
                         FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         equivalence,
                         implication,
                         is_equivalent_to_builtin_fraction,
                         is_fraction_valid)
from . import strategies


@given(strategies.fractions,
       strategies.non_zero_fractions_or_ints_or_builtin_ints)
def test_basic(dividend: Fraction, divisor: FractionOrIntOrBuiltinInt) -> None:
    result = dividend % divisor

    assert isinstance(result, Fraction)
    assert is_fraction_valid(result)


@given(strategies.fractions, strategies.non_zero_fractions)
def test_sign(dividend: Fraction, divisor: Fraction) -> None:
    result = dividend % divisor

    assert equivalence(result == 0, dividend / divisor == dividend // divisor)
    assert implication(result > 0, divisor > 0)


@given(strategies.fractions, strategies.non_zero_fractions)
def test_value(dividend: Fraction, divisor: Fraction) -> None:
    result = dividend % divisor

    assert abs(result) < abs(divisor)


@given(strategies.fractions, strategies.non_zero_fractions)
def test_alternatives(dividend: Fraction, divisor: Fraction) -> None:
    result = dividend % divisor

    assert result == dividend - (dividend // divisor) * divisor


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(dividend: Fraction,
                      divisor_with_builtin: IntWithBuiltin) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend % divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend % divisor_builtin
    else:
        assert result == dividend % divisor_builtin


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(
        dividend_with_builtin: FractionWithBuiltin,
        divisor_with_builtin: RationalWithBuiltin
) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend % divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend_builtin % divisor_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
                result, dividend_builtin % divisor_builtin
        )


@given(strategies.fractions, strategies.zero_fractions_or_ints_or_builtin_ints)
def test_zero_divisor(dividend: Fraction,
                      divisor: FractionOrIntOrBuiltinInt) -> None:
    with pytest.raises(ZeroDivisionError):
        dividend % divisor
