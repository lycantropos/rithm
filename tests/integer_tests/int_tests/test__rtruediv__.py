import pytest
from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin

from . import strategies


@given(strategies.ints_with_builtins, strategies.ints)
def test_connection_with__truediv__(
    dividend_with_builtin: IntWithBuiltin, divisor: Int
) -> None:
    dividend, dividend_builtin = dividend_with_builtin

    try:
        result = dividend_builtin / divisor
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            dividend / divisor
    else:
        assert result == dividend / divisor
