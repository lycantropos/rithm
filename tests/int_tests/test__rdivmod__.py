import pytest
from hypothesis import given

from rithm import Int
from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtins, strategies.ints)
def test_connection_with_divmod(dividend_with_builtin: IntWithBuiltin,
                                divisor: Int) -> None:
    dividend, dividend_builtin = dividend_with_builtin

    try:
        result = divmod(dividend_builtin, divisor)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(dividend, divisor)
    else:
        assert result == divmod(dividend, divisor)
