import fractions
from typing import Tuple

from hypothesis import given
from rithm import (Fraction,
                   Int)
from tests.utils import is_equivalent_to_builtin_fraction

from . import strategies


def test_no_argument_connection_with_builtin() -> None:
    result = Fraction()

    assert is_equivalent_to_builtin_fraction(result, fractions.Fraction())


@given(strategies.ints_with_builtin_ints,
       strategies.non_zero_ints_with_non_zero_builtin_ints)
def test_connection_with_builtin(numerators_pair: Tuple[Int, int],
                                 denominators_pair: Tuple[Int, int]) -> None:
    numerator, builtin_numerator = numerators_pair
    denominator, builtin_denominator = denominators_pair

    result = Fraction(numerator, denominator)

    assert is_equivalent_to_builtin_fraction(
        result, fractions.Fraction(builtin_numerator, builtin_denominator))


@given(strategies.ints_with_builtin_ints)
def test_numerator_only_connection_with_builtin(numerators_pair
                                                : Tuple[Int, int]) -> None:
    numerator, builtin_numerator = numerators_pair

    result = Fraction(numerator)

    assert is_equivalent_to_builtin_fraction(
        result, fractions.Fraction(builtin_numerator))
