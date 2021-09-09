import fractions

import pytest
from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.ints_with_builtin_ints, strategies.ints_with_builtin_ints)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    try:
        result = first / second
    except ZeroDivisionError:
        with pytest.raises(ZeroDivisionError):
            first_builtin / second_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
            result, fractions.Fraction(first_builtin, second_builtin))
