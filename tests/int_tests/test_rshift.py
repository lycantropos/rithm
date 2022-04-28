import pytest
from hypothesis import given

from rithm import Int
from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints, strategies.small_ints_with_builtins)
def test_polymorphism(base: Int, shift_with_builtin: IntWithBuiltin) -> None:
    shift, shift_builtin = shift_with_builtin

    try:
        result = base >> shift
    except ValueError as exception:
        with pytest.raises(type(exception)):
            base >> shift_builtin
    else:
        assert result == base >> shift_builtin


@given(strategies.ints_with_builtins, strategies.small_ints_with_builtins)
def test_connection_with_builtin(base_with_builtin: IntWithBuiltin,
                                 shift_with_builtin: IntWithBuiltin) -> None:
    base, base_builtin = base_with_builtin
    shift, shift_builtin = shift_with_builtin

    try:
        result = base >> shift
    except ValueError as exception:
        with pytest.raises(type(exception)):
            base_builtin >> shift_builtin
    else:
        assert is_equivalent_to_builtin_int(result,
                                            base_builtin >> shift_builtin)
