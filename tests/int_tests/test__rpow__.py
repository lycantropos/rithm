import pytest
from hypothesis import given

from rithm import Int
from tests.utils import IntWithBuiltin
from . import strategies


@given(strategies.ints_with_builtins, strategies.non_negative_one_byte_ints)
def test_non_negative_exponent_no_modulo_connection_with_pow(
        base_with_builtin: IntWithBuiltin,
        exponent: Int
) -> None:
    base, base_builtin = base_with_builtin

    assert base_builtin ** exponent == base ** exponent


@given(strategies.ints_with_builtins, strategies.negative_one_byte_ints)
def test_negative_exponent_no_modulo_connection_with_pow(
        base_with_builtin: IntWithBuiltin,
        exponent: Int
) -> None:
    base, base_builtin = base_with_builtin

    try:
        result = base_builtin ** exponent
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            base ** exponent
    else:
        assert result == base ** exponent
