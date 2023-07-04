from hypothesis import given

from rithm.enums import Endianness
from tests.utils import pickle_round_trip
from . import strategies


@given(strategies.endiannesses)
def test_round_trip(endianness: Endianness) -> None:
    assert pickle_round_trip(endianness) is endianness
