import math

from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin_int: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin_int

    assert is_equivalent_to_builtin_int(math.ceil(int_),
                                        math.ceil(builtin_int))
