import math

from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin, is_equivalent_to_builtin_int

from . import strategies


@given(strategies.ints)
def test_basic(int_: Int) -> None:
    assert isinstance(math.ceil(int_), Int)


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    assert is_equivalent_to_builtin_int(math.ceil(int_),
                                        math.ceil(builtin_int))
