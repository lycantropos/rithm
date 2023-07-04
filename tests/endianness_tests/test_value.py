from hypothesis import given

from rithm.enums import Endianness
from . import strategies


@given(strategies.endiannesses)
def test_basic(endianness: Endianness) -> None:
    result = endianness.value

    assert isinstance(result, str)


@given(strategies.endiannesses)
def test_determinism(endianness: Endianness) -> None:
    result = endianness.value

    assert result == endianness.value
