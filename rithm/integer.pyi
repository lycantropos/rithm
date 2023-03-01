from __future__ import annotations

import typing as _t

import typing_extensions as _te

from .enums import Endianness
from .fraction import Fraction


class Int:
    @property
    def denominator(self) -> _te.Self:
        ...

    @property
    def numerator(self) -> _te.Self:
        ...

    def bit_length(self) -> _te.Self:
        ...

    def gcd(self, other: _te.Self) -> _te.Self:
        ...

    def is_power_of_two(self) -> bool:
        ...

    def to_bytes(self, endianness: Endianness) -> bytes:
        ...

    @classmethod
    def from_bytes(cls, value: bytes, endianness: Endianness) -> _te.Self:
        ...

    @_t.overload
    def __new__(cls, _value: _t.Union[_te.Self, float, int] = ...) -> _te.Self:
        ...

    @_t.overload
    def __new__(cls, _value: str, _base: _t.Optional[int] = ...) -> _te.Self:
        ...

    def __abs__(self) -> _te.Self:
        ...

    def __add__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __and__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __bool__(self) -> bool:
        ...

    def __ceil__(self) -> _te.Self:
        ...

    def __divmod__(
            self, other: _t.Union[_te.Self, int]
    ) -> _t.Tuple[_te.Self, _te.Self]:
        ...

    @_t.overload
    def __eq__(self, other: _te.Self) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __float__(self) -> float:
        ...

    def __floor__(self) -> _te.Self:
        ...

    def __floordiv__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __ge__(self, other: _te.Self) -> bool:
        ...

    def __getstate__(self) -> int:
        ...

    def __gt__(self, other: _te.Self) -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __index__(self) -> int:
        ...

    def __int__(self) -> int:
        ...

    def __invert__(self) -> _te.Self:
        ...

    def __le__(self, other: _te.Self) -> bool:
        ...

    def __lshift__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __lt__(self, other: _te.Self) -> bool:
        ...

    def __mod__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __mul__(self, other: _te.Self) -> _te.Self:
        ...

    def __neg__(self) -> _te.Self:
        ...

    def __or__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __pos__(self) -> _te.Self:
        ...

    def __pow__(
            self,
            exponent: _t.Union[_te.Self, int],
            divisor: _t.Union[_te.Self, None, int] = ...
    ) -> _t.Union[Fraction, _te.Self]:
        ...

    def __radd__(self, other: int) -> _te.Self:
        ...

    def __rand__(self, other: int) -> _te.Self:
        ...

    def __rdivmod__(self, other: int) -> _t.Tuple[_te.Self, _te.Self]:
        ...

    def __repr__(self) -> str:
        ...

    def __rfloordiv__(self, other: int) -> _te.Self:
        ...

    def __rlshift__(self, other: int) -> _te.Self:
        ...

    def __rmod__(self, other: int) -> _te.Self:
        ...

    def __rmul__(self, other: int) -> _te.Self:
        ...

    def __ror__(self, other: int) -> _te.Self:
        ...

    def __round__(self, digits: _t.Optional[int] = ...) -> _te.Self:
        ...

    def __rpow__(
            self,
            base: int,
            divisor: _t.Union[_te.Self, None, int] = ...
    ) -> _t.Union[Fraction, _te.Self]:
        ...

    def __rrshift__(self, other: int) -> _te.Self:
        ...

    def __rshift__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __rsub__(self, other: int) -> _te.Self:
        ...

    def __rtruediv__(self, other: int) -> Fraction:
        ...

    def __rxor__(self, other: int) -> _te.Self:
        ...

    def __setstate__(self, state: int) -> None:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...

    def __truediv__(self, other: _t.Union[_te.Self, int]) -> Fraction:
        ...

    def __trunc__(self) -> _te.Self:
        ...

    def __xor__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        ...
