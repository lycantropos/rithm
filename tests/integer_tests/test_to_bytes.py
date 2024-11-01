from hypothesis import given

from rithm.enums import Endianness
from rithm.integer import Int
from tests.utils import IntWithBuiltin

from . import strategies


@given(strategies.ints, strategies.endianesses)
def test_basic(int_: Int, endianness: Endianness) -> None:
    result = int_.to_bytes(endianness)

    assert isinstance(result, bytes)


@given(strategies.ints, strategies.endianesses)
def test_round_trip(int_: Int, endianness: Endianness) -> None:
    result = int_.to_bytes(endianness)

    assert Int.from_bytes(result, endianness) == int_


@given(strategies.ints_with_builtins, strategies.endianesses)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin,
                                 endianness: Endianness) -> None:
    int_, builtin_int = int_with_builtin

    assert (int_.to_bytes(endianness)
            == builtin_int.to_bytes(to_bytes_count(builtin_int),
                                    'big'
                                    if endianness is Endianness.BIG
                                    else 'little',
                                    signed=True))


def to_bytes_count(value: int) -> int:
    return (8 + (value + (value < 0)).bit_length()) // 8
