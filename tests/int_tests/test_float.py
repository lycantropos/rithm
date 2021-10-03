import pytest
from hypothesis import given

from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin_int: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin_int

    try:
        result = float(int_)
    except OverflowError:
        with pytest.raises(OverflowError):
            float(builtin_int)
    else:
        assert result == float(builtin_int)
