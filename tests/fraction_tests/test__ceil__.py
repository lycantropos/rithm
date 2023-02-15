import math

from hypothesis import given

from rithm.fraction import Fraction
from rithm.integer import Int
from . import strategies


@given(strategies.fractions)
def test_basic(fraction: Fraction) -> None:
    result = math.ceil(fraction)

    assert isinstance(result, Int)


@given(strategies.fractions)
def test_value(fraction: Fraction) -> None:
    result = math.ceil(fraction)

    assert fraction <= result < fraction + 1
    assert result % 1 == 0


@given(strategies.fractions)
def test_alternatives(fraction: Fraction) -> None:
    result = math.ceil(fraction)

    assert result == -math.floor(-fraction)
    assert result == -(-fraction // 1)
