import fractions

from hypothesis import given

from rithm import Fraction
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


def test_no_argument_connection_with_builtin() -> None:
    result = Fraction()

    assert is_equivalent_to_builtin_fraction(result, fractions.Fraction())


@given(strategies.ints_with_builtins, strategies.non_zero_ints_with_builtins)
def test_connection_with_builtin(numerators: IntWithBuiltin,
                                 denominators: IntWithBuiltin) -> None:
    numerator, builtin_numerator = numerators
    denominator, builtin_denominator = denominators

    result = Fraction(numerator, denominator)

    assert is_equivalent_to_builtin_fraction(
        result, fractions.Fraction(builtin_numerator, builtin_denominator))


@given(strategies.ints_with_builtins)
def test_numerator_only_connection_with_builtin(numerators: IntWithBuiltin
                                                ) -> None:
    numerator, builtin_numerator = numerators

    result = Fraction(numerator)

    assert is_equivalent_to_builtin_fraction(
        result, fractions.Fraction(builtin_numerator))
