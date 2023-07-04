from hypothesis import given

from rithm.enums import TieBreaking
from . import strategies


@given(strategies.tie_breakings)
def test_basic(tie_breaking: TieBreaking) -> None:
    result = tie_breaking.value

    assert isinstance(result, int)


@given(strategies.tie_breakings)
def test_determinism(tie_breaking: TieBreaking) -> None:
    result = tie_breaking.value

    assert result == tie_breaking.value
