import pytest
from hypothesis import given

from rithm.integer import Int
from tests.utils import IntWithBuiltin

from . import strategies


@given(strategies.ints_with_builtins, strategies.small_ints)
def test_connection_with__rshift__(base_with_builtin: IntWithBuiltin,
                                   shift: Int) -> None:
    base, base_builtin = base_with_builtin

    try:
        result = base_builtin >> shift
    except ValueError as exception:
        with pytest.raises(type(exception)):
            base >> shift
    else:
        assert result == base >> shift
