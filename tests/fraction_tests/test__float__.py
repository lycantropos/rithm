import pytest
from hypothesis import given

from tests.utils import FractionWithBuiltin
from . import strategies


@given(strategies.fractions_with_builtins)
def test_connection_with_builtin(
        fraction_with_builtin: FractionWithBuiltin
) -> None:
    fraction, builtin_fraction = fraction_with_builtin

    try:
        result = float(fraction)
    except OverflowError as exception:
        with pytest.raises(type(exception)):
            float(builtin_fraction)
    else:
        assert result == float(builtin_fraction)
