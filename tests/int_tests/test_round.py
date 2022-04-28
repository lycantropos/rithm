from typing import Optional

from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints_with_builtins, strategies.maybe_small_integers)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin,
                                 digits: Optional[int]) -> None:
    int_, builtin_int = int_with_builtin

    result = round(int_, digits)

    assert is_equivalent_to_builtin_int(result, round(builtin_int, digits))
