from hypothesis import given

from tests.utils import (FractionWithBuiltin,
                         RationalWithBuiltin,
                         equivalence)
from . import strategies


@given(strategies.fractions_with_builtins, strategies.rationals_with_builtins)
def test_connection_with_builtin(first_with_builtin: FractionWithBuiltin,
                                 second_with_builtin: RationalWithBuiltin
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first >= second, first_builtin >= second_builtin)
