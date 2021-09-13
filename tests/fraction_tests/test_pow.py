import pytest
from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         IntWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.fractions_with_builtin_fractions,
       strategies.small_ints_with_builtin_ints)
def test_connection_with_builtin(base_with_builtin: FractionWithBuiltin,
                                 exponent_with_builtin: IntWithBuiltin
                                 ) -> None:
    base, base_builtin = base_with_builtin
    exponent, exponent_builtin = exponent_with_builtin

    try:
        result = base ** exponent
    except ZeroDivisionError:
        with pytest.raises(ZeroDivisionError):
            base_builtin ** exponent_builtin
    else:
        assert is_equivalent_to_builtin_fraction(
            result, base_builtin ** exponent_builtin)
