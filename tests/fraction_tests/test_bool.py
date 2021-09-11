from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         equivalence)
from . import strategies


@given(strategies.fractions_with_builtin_fractions)
def test_connection_with_builtin(fraction_with_builtin: FractionWithBuiltin
                                 ) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    assert equivalence(bool(fraction), bool(builtin_fraction))
