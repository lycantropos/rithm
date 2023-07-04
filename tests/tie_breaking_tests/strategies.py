from hypothesis import strategies as _st

from rithm.enums import TieBreaking as _TieBreaking

_tie_breakings = [_TieBreaking.AWAY_FROM_ZERO, _TieBreaking.TO_EVEN,
                  _TieBreaking.TO_ODD, _TieBreaking.TOWARD_ZERO]
tie_breakings = _st.sampled_from(_tie_breakings)
_tie_breakings_values = [tie_breaking.value for tie_breaking in _tie_breakings]
tie_breakings_values = _st.sampled_from(_tie_breakings_values)
non_tie_breakings_values = (
        _st.integers().filter(lambda value: value not in _tie_breakings_values)
        | _st.from_type(type).filter(lambda value: not isinstance(value, int))
)
