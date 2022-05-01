import pytest
from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         is_equivalent_to_builtin_fraction,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(dividend: Fraction, divisor_with_builtin: IntWithBuiltin
                      ) -> None:
    divisor, divisor_builtin = divisor_with_builtin

    try:
        result = divmod(dividend, divisor)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(dividend, divisor_builtin)
    else:
        assert result == divmod(dividend, divisor_builtin)


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(dividend_with_builtin: FractionWithBuiltin,
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
