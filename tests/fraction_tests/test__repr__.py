import sys

from hypothesis import given

from rithm import Fraction
from . import strategies


@given(strategies.fractions)
def test_round_trip(fraction: Fraction) -> None:
    result = repr(fraction)

    assert eval(result, sys.modules) == fraction
