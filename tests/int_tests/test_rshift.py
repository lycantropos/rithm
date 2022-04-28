import pytest
from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.ints_with_builtins, strategies.small_ints_with_builtins)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    try:
        result = first >> second
    except ValueError as exception:
        with pytest.raises(type(exception)):
            first_builtin >> second_builtin
    else:
        assert is_equivalent_to_builtin_int(result,
                                            first_builtin >> second_builtin)
