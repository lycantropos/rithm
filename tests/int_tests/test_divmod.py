from typing import Tuple

import pytest
from hypothesis import given

from rithm import Int
from tests.utils import is_equivalent_to_builtin_int
from . import strategies


@given(strategies.ints_with_builtin_ints, strategies.ints_with_builtin_ints)
def test_connection_with_builtin(first_int_with_builtin_int: Tuple[Int, int],
                                 second_int_with_builtin_int: Tuple[Int, int]
                                 ) -> None:
    first_int, first_builtin_int = first_int_with_builtin_int
    second_int, second_builtin_int = second_int_with_builtin_int

    try:
        quotient, remainder = divmod(first_int, second_int)
    except ZeroDivisionError:
        with pytest.raises(ZeroDivisionError):
            divmod(first_builtin_int, second_builtin_int)
    else:
        builtin_quotient, builtin_remainder = divmod(first_builtin_int,
                                                     second_builtin_int)

        assert is_equivalent_to_builtin_int(quotient, builtin_quotient)
        assert is_equivalent_to_builtin_int(remainder, builtin_remainder)
