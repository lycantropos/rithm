import math

from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin, is_equivalent_to_builtin_int

from . import strategies


@given(strategies.ints)
def test_basic(int_: Int) -> None:
    assert isinstance(math.trunc(int_), Int)


@given(strategies.ints)
def test_value(int_: Int) -> None:
    result = math.trunc(int_)

    assert abs(result - int_) < 1
    assert result % 1 == 0


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    assert is_equivalent_to_builtin_int(math.trunc(int_),
                                        math.trunc(builtin_int))
