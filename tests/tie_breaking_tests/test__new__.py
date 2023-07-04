from typing import Any

import pytest
from hypothesis import given

from rithm.enums import TieBreaking
from tests.utils import equivalence
from . import strategies


@given(strategies.tie_breakings_values)
def test_basic(value: int) -> None:
    result = TieBreaking(value)

    assert isinstance(result, TieBreaking)


@given(strategies.tie_breakings_values, strategies.tie_breakings_values)
def test_bijection(first: int, second: int) -> None:
    assert equivalence(first == second,
                       TieBreaking(first) == TieBreaking(second))


@given(strategies.tie_breakings_values)
def test_value_round_trip(value: int) -> None:
    result = TieBreaking(value)

    assert result == TieBreaking(result.value)


@given(strategies.non_tie_breakings_values)
def test_invalid_value(value: Any) -> None:
    with pytest.raises(ValueError):
        TieBreaking(value)
