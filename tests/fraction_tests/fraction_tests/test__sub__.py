from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import (
    FractionOrIntOrBuiltinInt,
    FractionWithBuiltin,
    IntWithBuiltin,
    RationalWithBuiltin,
    equivalence,
    is_equivalent_to_builtin_fraction,
    is_fraction_valid,
)

from . import strategies


@given(strategies.fractions, strategies.fractions_or_ints_or_builtin_ints)
def test_basic(
    subtrahend: Fraction, minuend: FractionOrIntOrBuiltinInt
) -> None:
    result = subtrahend - minuend

    assert isinstance(result, Fraction)
    assert is_fraction_valid(result)


@given(strategies.fractions)
def test_diagonal(fraction: Fraction) -> None:
    assert not fraction - fraction


@given(strategies.fractions, strategies.fractions)
def test_commutative_case(subtrahend: Fraction, minuend: Fraction) -> None:
    assert equivalence(
        subtrahend - minuend == minuend - subtrahend, subtrahend == minuend
    )


@given(strategies.fractions, strategies.zero_fractions)
def test_right_neutral_element(
    subtrahend: Fraction, minuend: Fraction
) -> None:
    assert subtrahend - minuend == subtrahend


@given(strategies.fractions, strategies.fractions)
def test_alternatives(subtrahend: Fraction, minuend: Fraction) -> None:
    assert subtrahend - minuend == subtrahend + (-minuend)


@given(strategies.fractions, strategies.ints_with_builtins)
def test_polymorphism(
    subtrahend: Fraction, minuend_with_builtin: IntWithBuiltin
) -> None:
    minuend, minuend_builtin = minuend_with_builtin

    assert subtrahend - minuend == subtrahend - minuend_builtin


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(
    subtrahend_with_builtin: FractionWithBuiltin,
    minuend_with_builtin: RationalWithBuiltin,
) -> None:
    subtrahend, subtrahend_builtin = subtrahend_with_builtin
    minuend, minuend_builtin = minuend_with_builtin

    assert is_equivalent_to_builtin_fraction(
        subtrahend - minuend, subtrahend_builtin - minuend_builtin
    )
