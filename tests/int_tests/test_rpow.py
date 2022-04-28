import sys

import pytest
from hypothesis import given

from rithm import Int
from tests.utils import (IntOrBuiltin,
                         IntWithBuiltin)
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


@given(strategies.ints_with_builtins,
       strategies.non_negative_one_byte_ints
       if sys.version_info < (3, 8)
       else strategies.ints,
       strategies.ints_or_builtins)
def test_with_modulo_connection_with_pow(
        base_with_builtin: IntWithBuiltin,
        exponent: Int,
        divisor: IntOrBuiltin
) -> None:
    base, base_builtin = base_with_builtin

    try:
        result = pow(base_builtin, exponent, divisor)
    except ValueError as exception:
        with pytest.raises(type(exception)):
            pow(base, exponent, divisor)
    else:
        assert result == pow(base, exponent, divisor)
