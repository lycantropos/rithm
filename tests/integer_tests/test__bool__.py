from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         equivalence)
from . import strategies


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    assert equivalence(bool(int_), bool(builtin_int))
