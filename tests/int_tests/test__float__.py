import pytest
from hypothesis import given

from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtins)
def test_connection_with_builtin(int_with_builtin: IntWithBuiltin) -> None:
    int_, builtin_int = int_with_builtin

    try:
        result = float(int_)
    except OverflowError as exception:
        with pytest.raises(type(exception)):
            float(builtin_int)
    else:
        assert result == float(builtin_int)
