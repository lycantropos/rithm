from hypothesis import strategies as _st

from rithm.enums import Endianness as _Endianness

_endiannesses = [_Endianness.BIG, _Endianness.LITTLE]
endiannesses = _st.sampled_from(_endiannesses)
_endiannesses_values = [endianness.value for endianness in _endiannesses]
endiannesses_values = _st.sampled_from(_endiannesses_values)
non_endiannesses_values = (
        _st.text().filter(lambda value: value not in _endiannesses_values)
        | _st.from_type(type).filter(lambda value: not isinstance(value, str))
)
