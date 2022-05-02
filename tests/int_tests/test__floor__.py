import math

from hypothesis import given

from rithm import Int
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints)
def test_basic(int_: Int) -> None:
    assert isinstance(math.floor(int_), Int)


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    assert is_equivalent_to_builtin_int(math.floor(int_),
                                        math.floor(builtin_int))
