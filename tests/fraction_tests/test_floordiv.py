from typing import Union

import pytest
from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_int)
from . import strategies


@given(strategies.fractions_with_builtins,
       strategies.fractions_or_ints_with_builtins)
def test_connection_with_builtin(first_with_builtin: FractionWithBuiltin,
                                 second_with_builtin
                                 : Union[FractionWithBuiltin, IntWithBuiltin]
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    try:
        result = first // second
    except ZeroDivisionError:
        with pytest.raises(ZeroDivisionError):
            first_builtin // second_builtin
    else:
        assert is_equivalent_to_builtin_int(result,
                                            first_builtin // second_builtin)
