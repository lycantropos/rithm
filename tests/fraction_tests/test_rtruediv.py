from hypothesis import given

from rithm import (Fraction,
                   Int)
from . import strategies


@given(strategies.non_zero_fractions, strategies.ints)
def test_properties(fraction: Fraction, int_: Int) -> None:
    assert (int_ / fraction) * fraction == int_
