import pytest
from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionOrIntOrBuiltinInt,
                         FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         equivalence,
                         is_equivalent_to_builtin_fraction,
                         is_fraction_valid)
from . import strategies


@given(strategies.fractions,
       strategies.non_zero_fractions_or_ints_or_builtin_ints)
def test_basic(dividend: Fraction, divisor: FractionOrIntOrBuiltinInt) -> None:
    result = dividend / divisor

    assert isinstance(result, Fraction)
    assert is_fraction_valid(result)


@given(strategies.non_zero_fractions, strategies.non_zero_fractions)
def test_commutative_case(dividend: Fraction, divisor: Fraction) -> None:
    assert equivalence(dividend / divisor == divisor / dividend,
                       abs(dividend) == abs(divisor))


@given(strategies.zero_fractions, strategies.non_zero_fractions)
def test_left_absorbing_element(dividend: Fraction, divisor: Fraction) -> None:
    assert dividend / divisor == dividend


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(dividend: Fraction,
                      divisor_with_builtin: IntWithBuiltin) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend / divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend / divisor_builtin
    else:
        assert result == dividend / divisor_builtin


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(
        dividend_with_builtin: FractionWithBuiltin,
        divisor_with_builtin: RationalWithBuiltin
) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend / divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend_builtin / divisor_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
                result, dividend_builtin / divisor_builtin
        )
