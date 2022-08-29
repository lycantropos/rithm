import pytest
from hypothesis import given

from rithm import (Fraction,
                   Int)
from tests.utils import (FractionOrIntOrBuiltinInt,
                         FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         is_equivalent_to_builtin_fraction,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.fractions,
       strategies.non_zero_fractions_or_ints_or_builtin_ints)
def test_basic(dividend: Fraction, divisor: FractionOrIntOrBuiltinInt) -> None:
    result = divmod(dividend, divisor)

    assert isinstance(result, tuple)
    assert len(result) == 2
    assert isinstance(result[0], Int)
    assert isinstance(result[1], Fraction)


@given(strategies.fractions,
       strategies.non_zero_fractions_or_ints_or_builtin_ints)
def test_alternatives(dividend: Fraction,
                      divisor: FractionOrIntOrBuiltinInt) -> None:
    result = divmod(dividend, divisor)

    assert result == (dividend // divisor, dividend % divisor)


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(dividend: Fraction,
                      divisor_with_builtin: IntWithBuiltin) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = divmod(dividend, divisor)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(dividend, divisor_builtin)
    else:
        assert result == divmod(dividend, divisor_builtin)


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(
        dividend_with_builtin: FractionWithBuiltin,
        divisor_with_builtin: RationalWithBuiltin
) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        quotient, remainder = divmod(dividend, divisor)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(dividend_builtin, divisor_builtin)
    else:
        builtin_quotient, builtin_remainder = divmod(dividend_builtin,
                                                     divisor_builtin)

        assert is_equivalent_to_builtin_int(quotient, builtin_quotient)
        assert is_equivalent_to_builtin_fraction(remainder, builtin_remainder)


@given(strategies.fractions, strategies.zero_fractions_or_ints_or_builtin_ints)
def test_zero_divisor(dividend: Fraction,
                      divisor: FractionOrIntOrBuiltinInt) -> None:
    with pytest.raises(ZeroDivisionError):
        divmod(dividend, divisor)
