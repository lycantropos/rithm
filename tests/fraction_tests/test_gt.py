from hypothesis import given

from tests.utils import (FractionsWithBuiltin,
                         equivalence)
from . import strategies


@given(strategies.fractions_with_builtin_fractions,
       strategies.fractions_with_builtin_fractions)
def test_connection_with_builtin(first_with_builtin: FractionsWithBuiltin,
                                 second_with_builtin: FractionsWithBuiltin
                                 ) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first > second, first_builtin > second_builtin)
