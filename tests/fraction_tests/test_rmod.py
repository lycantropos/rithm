import pytest
from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.fractions_with_builtins, strategies.ints_with_builtins)
def test_connection_with_builtin(fraction_with_builtin: FractionWithBuiltin,
                                 int_with_builtin: IntWithBuiltin) -> None:
    fraction, builtin_fraction = fraction_with_builtin
    int_, builtin_int = int_with_builtin

    try:
        result = int_ % fraction
    except ZeroDivisionError:
        with pytest.raises(ZeroDivisionError):
            builtin_int % builtin_fraction
    else:
        assert is_equivalent_to_builtin_fraction(
            result, builtin_int % builtin_fraction)
