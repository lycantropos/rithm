from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         is_equivalent_to_builtin_fraction,
                         is_fraction_valid)
from . import strategies


@given(strategies.fractions, strategies.fractions)
def test_basic(first: Fraction, second: Fraction) -> None:
    result = first * second

    assert isinstance(result, Fraction)
    assert is_fraction_valid(result)


@given(strategies.fractions, strategies.fractions)
def test_commutativity(first: Fraction, second: Fraction) -> None:
    assert first * second == second * first


@given(strategies.fractions, strategies.zero_fractions)
def test_absorbing_element(first: Fraction, second: Fraction) -> None:
    assert first * second == second == second * first


@given(strategies.fractions, strategies.fractions, strategies.fractions)
def test_associativity(first: Fraction, second: Fraction, third: Fraction
                       ) -> None:
    assert (first * second) * third == first * (second * third)


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(first: Fraction, second_with_builtin: IntWithBuiltin
                      ) -> None:
    second, second_builtin = second_with_builtin

    assert first * second == first * second_builtin


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(first_with_builtin: FractionWithBuiltin,
                                 second_with_builtin: RationalWithBuiltin
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert is_equivalent_to_builtin_fraction(first * second,
                                             first_builtin * second_builtin)
