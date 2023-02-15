import fractions

import pytest
from hypothesis import given

from rithm import Int
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.ints, strategies.ints_with_builtins)
def test_polymorphism(dividend: Int,
                      divisor_with_builtin: IntWithBuiltin) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend / divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend / divisor_builtin
    else:
        assert result == dividend / divisor_builtin


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(dividend_with_builtin: IntWithBuiltin,
                                 divisor_with_builtin: IntWithBuiltin) -> None:
    dividend, dividend_builtin = dividend_with_builtin
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = dividend / divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend_builtin / divisor_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
                result, fractions.Fraction(dividend_builtin, divisor_builtin)
        )
