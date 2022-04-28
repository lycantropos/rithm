import fractions

import pytest
from hypothesis import given
from rithm import Fraction
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)

from . import strategies


def test_no_argument_connection_with_builtin() -> None:
    result = Fraction()

    assert is_equivalent_to_builtin_fraction(result, fractions.Fraction())


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(numerators: IntWithBuiltin,
                                 denominators: IntWithBuiltin) -> None:
    numerator, builtin_numerator = numerators
    denominator, builtin_denominator = denominators

    try:
        result = Fraction(numerator, denominator)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            fractions.Fraction(builtin_numerator, builtin_denominator)
    else:
        assert is_equivalent_to_builtin_fraction(
                result,
                fractions.Fraction(builtin_numerator, builtin_denominator)
        )


@given(strategies.floats)
def test_float_connection_with_builtin(float_: float) -> None:
    try:
        result = Fraction(float_)
    except (OverflowError, ValueError) as error:
        with pytest.raises(type(error)):
            fractions.Fraction(float_)
    else:
        assert is_equivalent_to_builtin_fraction(result,
                                                 fractions.Fraction(float_))


@given(strategies.ints_with_builtins)
def test_numerator_only_connection_with_builtin(numerators: IntWithBuiltin
                                                ) -> None:
    numerator, builtin_numerator = numerators

    result = Fraction(numerator)

    assert is_equivalent_to_builtin_fraction(
            result, fractions.Fraction(builtin_numerator)
    )
