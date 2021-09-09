from hypothesis import given

from tests.utils import (IntWithBuiltin,
                         equivalence)
from . import strategies


@given(strategies.ints_with_builtin_ints, strategies.ints_with_builtin_ints)
def test_connection_with_builtin(first_with_builtin: IntWithBuiltin,
                                 second_with_builtin: IntWithBuiltin) -> None:
    first, first_builtin = first_with_builtin
    second, second_builtin = second_with_builtin

    assert equivalence(first > second, first_builtin > second_builtin)
