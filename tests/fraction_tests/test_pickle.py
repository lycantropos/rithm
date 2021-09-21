from hypothesis import given

from rithm import Fraction
from tests.utils import pickle_round_trip
from . import strategies


@given(strategies.fractions)
def test_round_trip(fraction: Fraction) -> None:
    assert pickle_round_trip(fraction) == fraction
