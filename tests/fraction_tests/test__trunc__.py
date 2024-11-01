import math

from hypothesis import given

from rithm.fraction import Fraction
from rithm.integer import Int
from tests.utils import FractionWithBuiltin, is_equivalent_to_builtin_int

from . import strategies


@given(strategies.fractions)
def test_basic(fraction: Fraction) -> None:
    result = math.trunc(fraction)

    assert isinstance(result, Int)


@given(strategies.fractions)
def test_value(fraction: Fraction) -> None:
    result = math.trunc(fraction)

    assert abs(result - fraction) < 1
    assert result % 1 == 0


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(
    fraction_with_builtin: FractionWithBuiltin,
) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    assert is_equivalent_to_builtin_int(
        math.trunc(fraction), math.trunc(builtin_fraction)
    )
