import pytest
from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.ints_with_builtins, strategies.fractions)
def test_polymorphism(dividend_with_builtin: IntWithBuiltin, divisor: Fraction
                      ) -> None:
    dividend, dividend_builtin = dividend_with_builtin

    try:
        result = dividend % divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend_builtin % divisor
    else:
        assert result == dividend_builtin % divisor


@given(strategies.ints_with_builtins, strategies.fractions_with_builtins)
def test_connection_with_builtin(dividend_with_builtin: IntWithBuiltin,
                                 divisor_with_builtin: FractionWithBuiltin
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
