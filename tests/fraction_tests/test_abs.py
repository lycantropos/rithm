from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         is_equivalent_to_builtin_fraction)
from . import strategies


@given(strategies.fractions_with_builtins)
def test_connection_with_builtin(fraction_with_builtin_fraction
                                 : FractionWithBuiltin) -> None:
    fraction, builtin_fraction = fraction_with_builtin_fraction

    assert is_equivalent_to_builtin_fraction(abs(fraction),
                                             abs(builtin_fraction))
