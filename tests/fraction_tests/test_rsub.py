from hypothesis import given

from rithm import (Fraction,
                   Int)
from . import strategies


@given(strategies.fractions, strategies.ints)
def test_connection_with_add(fraction: Fraction, int_: Int) -> None:
    assert int_ - fraction == -(fraction - int_)
