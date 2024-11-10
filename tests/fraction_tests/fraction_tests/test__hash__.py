from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import FractionWithBuiltin, implication

from . import strategies


@given(strategies.fractions)
def test_basic(fraction: Fraction) -> None:
    result = hash(fraction)

    assert isinstance(result, int)


@given(strategies.fractions)
def test_determinism(fraction: Fraction) -> None:
    result = hash(fraction)

    assert result == hash(fraction)


@given(strategies.fractions, strategies.fractions)
def test_connection_with_equality(left: Fraction, right: Fraction) -> None:
    assert implication(left == right, hash(left) == hash(right))


@given(strategies.fractions_with_builtins)
def test_connection_with_builtin(
    fraction_with_builtin: FractionWithBuiltin,
) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    assert hash(fraction) == hash(builtin_fraction)
