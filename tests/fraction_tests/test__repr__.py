import sys

from hypothesis import given

from rithm.fraction import Fraction
from rithm.integer import Int
from . import strategies


@given(strategies.fractions)
def test_round_trip(fraction: Fraction) -> None:
    result = repr(fraction)

    assert (eval(result, {**vars(sys.modules[Fraction.__module__]),
                          **vars(sys.modules[Int.__module__])})
            == fraction)
