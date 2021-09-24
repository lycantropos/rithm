from hypothesis import given

from rithm import (Fraction,
                   Int)
from . import strategies


@given(strategies.fractions, strategies.ints)
def test_properties(fraction: Fraction, int_: Int) -> None:
    try:
        result = int_ / fraction
    except ZeroDivisionError:
        assert not fraction
    else:
        assert result * fraction == int_
