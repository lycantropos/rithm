from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints_with_builtin_ints,
       strategies.non_negative_ints_with_builtin_ints)
def test_non_negative_exponent_no_modulo_connection_with_builtin(
        base_with_builtin: IntWithBuiltin,
        exponent_with_builtin: IntWithBuiltin) -> None:
    base, base_builtin = base_with_builtin
    exponent, exponent_builtin = exponent_with_builtin

    assert is_equivalent_to_builtin_int(base ** exponent,
                                        base_builtin ** exponent_builtin)
