import pytest
from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    try:
        quotient, remainder = divmod(first, second)
    except ZeroDivisionError as exception:
        with pytest.raises(type(exception)):
            divmod(first_builtin, second_builtin)
    else:
        builtin_quotient, builtin_remainder = divmod(first_builtin,
                                                     second_builtin)

        assert is_equivalent_to_builtin_int(quotient, builtin_quotient)
        assert is_equivalent_to_builtin_int(remainder, builtin_remainder)
