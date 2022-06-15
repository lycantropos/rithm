from operator import index

from hypothesis import given

from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    assert index(int_) == builtin_int
