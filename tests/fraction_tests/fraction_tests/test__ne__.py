from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import (
    FractionWithBuiltin,
    IntWithBuiltin,
    RationalWithBuiltin,
    equivalence,
)

from . import strategies


@given(strategies.fractions)
def test_irreflexivity(fraction: Fraction) -> None:
    assert fraction == fraction


@given(strategies.fractions, strategies.fractions)
def test_symmetry(first: Fraction, second: Fraction) -> None:
    assert equivalence(first != second, second != first)


@given(strategies.fractions, strategies.fractions)
def test_equivalents(first: Fraction, second: Fraction) -> None:
    assert equivalence(first != second, first != second)
    assert equivalence(first != second, first > second or first < second)
    assert equivalence(first != second, first > second or second > first)
    assert equivalence(first != second, second < first or second > first)
    assert equivalence(first != second, second < first or first < second)


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(
    first: Fraction, second_with_builtin: IntWithBuiltin
) -> None:
    second, second_builtin = second_with_builtin

    assert equivalence(first != second, first != second_builtin)


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(
    first_with_builtin: FractionWithBuiltin,
    second_with_builtin: RationalWithBuiltin,
) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first != second, first_builtin != second_builtin)
