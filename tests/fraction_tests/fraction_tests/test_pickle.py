from hypothesis import given

from rithm.fraction import Fraction
from tests.utils import pickling_round_trip

from . import strategies


@given(strategies.fractions)
def test_round_trip(fraction: Fraction) -> None:
    assert pickling_round_trip(fraction) == fraction
