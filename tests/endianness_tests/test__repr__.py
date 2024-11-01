import sys

from hypothesis import given

from rithm.enums import Endianness

from . import strategies


@given(strategies.endiannesses)
def test_round_trip(endianness: Endianness) -> None:
    result = repr(endianness)

    assert eval(result, vars(sys.modules[Endianness.__module__])) is endianness
