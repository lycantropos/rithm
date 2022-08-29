from hypothesis import given

from rithm import Fraction
from tests.utils import (FractionOrIntOrBuiltinInt,
                         FractionWithBuiltin,
                         RationalWithBuiltin,
                         equivalence,
                         implication)
from . import strategies


@given(strategies.fractions)
def test_reflexivity(fraction: Fraction) -> None:
    assert fraction == fraction


@given(strategies.fractions, strategies.fractions)
def test_symmetry(first: Fraction, second: Fraction) -> None:
    assert equivalence(first == second, second == first)


@given(strategies.fractions, strategies.fractions, strategies.fractions)
def test_transitivity(first: Fraction,
                      second: Fraction,
                      third: Fraction) -> None:
    assert implication(first == second and second == third, first == third)


@given(strategies.fractions, strategies.fractions_or_ints_or_builtin_ints)
def test_connection_with_inequality(first: Fraction,
                                    second: FractionOrIntOrBuiltinInt) -> None:
    assert equivalence(first == second, not first != second)


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(
        first_with_builtin: FractionWithBuiltin,
        second_with_builtin: RationalWithBuiltin
) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first == second, first_builtin == second_builtin)
