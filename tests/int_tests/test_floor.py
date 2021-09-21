import math

from hypothesis import given

from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtin_ints)
def test_connection_with_builtin(int_with_builtin_int: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin_int

    assert math.floor(int_) == math.floor(builtin_int)
