import sys

from hypothesis import given

from rithm.enums import Endianness
from . import strategies


@given(strategies.endianesses)
def test_round_trip(endianness: Endianness) -> None:
    result = repr(endianness)

    assert eval(result, sys.modules) is endianness
