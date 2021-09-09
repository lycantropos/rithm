from hypothesis import given

from tests.utils import FractionsWithBuiltin
from . import strategies


@given(strategies.fractions_with_builtin_fractions)
def test_connection_with_builtin(fraction_with_builtin: FractionsWithBuiltin) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    assert str(fraction) == str(builtin_fraction)
