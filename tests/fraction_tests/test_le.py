from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         RationalWithBuiltin,
                         equivalence,
                         implication)
from . import strategies


@given(strategies.fractions)
def test_reflexivity(fraction: Fraction) -> None:
    assert fraction <= fraction


@given(strategies.fractions, strategies.fractions)
def test_antisymmetry(first: Fraction, second: Fraction) -> None:
    assert equivalence(first <= second <= first, first == second)


@given(strategies.fractions, strategies.fractions, strategies.fractions)
def test_transitivity(first: Fraction, second: Fraction, third: Fraction
                      ) -> None:
    assert implication(first <= second <= third, first <= third)


@given(strategies.fractions, strategies.fractions)
def test_alternatives(first: Fraction, second: Fraction) -> None:
    assert equivalence(first <= second, first < second or first == second)
    assert equivalence(first <= second, first < second or not first != second)
    assert equivalence(first <= second, second > first or not first != second)
    assert equivalence(first <= second, second > first or first == second)
    assert equivalence(first <= second, second >= first)
    assert equivalence(first <= second, not second < first)
    assert equivalence(first <= second, not first > second)


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(first: Fraction, second_with_builtin: IntWithBuiltin
                      ) -> None:
    second, second_builtin = second_with_builtin

    assert equivalence(first <= second, first <= second_builtin)


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(first_with_builtin: FractionWithBuiltin,
                                 second_with_builtin: RationalWithBuiltin
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first <= second, first_builtin <= second_builtin)
