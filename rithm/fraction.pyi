from __future__ import annotations

import typing as _t
from numbers import Rational as _Rational

import typing_extensions as _te

from .enums import TieBreaking
from .integer import Int


class Fraction:
    @property
    def denominator(self) -> Int:
        ...

    @property
    def numerator(self) -> Int:
        ...

    def round(self, tie_breaking: TieBreaking) -> Int:
        ...

    @_t.overload
    def __new__(
            cls, _value: _t.Union[_te.Self, Int, _Rational, float, int] = ...
    ) -> _te.Self:
        ...

    @_t.overload
    def __new__(cls,
                _numerator: _t.Union[Int, int],
                _denominator: _t.Union[Int, int]) -> _te.Self:
        ...

    def __abs__(self) -> _te.Self:
        ...

    def __add__(self, other: _t.Union[_te.Self, Int, int]) -> _te.Self:
        ...

    def __bool__(self) -> bool:
        ...

    def __ceil__(self) -> Int:
        ...

    def __divmod__(
            self, divisor: _t.Union[_te.Self, Int, int]
    ) -> _t.Tuple[Int, _te.Self]:
        ...

    @_t.overload
    def __eq__(self, other: _t.Union[_te.Self, Int, int]) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __float__(self) -> float:
        ...

    def __floor__(self) -> Int:
        ...

    def __floordiv__(self, divisor: _t.Union[_te.Self, Int, int]) -> Int:
        ...

    def __ge__(self, other: _t.Union[_te.Self, Int, int]) -> bool:
        ...

    def __getstate__(self) -> _t.Tuple[Int, Int]:
        ...

    def __gt__(self, other: _t.Union[_te.Self, Int, int]) -> bool:
        ...

    def __hash__(self) -> int:
        ...

    def __le__(self, other: _t.Union[_te.Self, Int, int]) -> bool:
        ...

    def __lt__(self, other: _t.Union[_te.Self, Int, int]) -> bool:
        ...

    def __mod__(self, divisor: _t.Union[_te.Self, Int, int]) -> _te.Self:
        ...

    def __mul__(self, other: _t.Union[_te.Self, Int, int]) -> _te.Self:
        ...

    def __neg__(self) -> _te.Self:
        ...

    def __pos__(self) -> _te.Self:
        ...

    def __pow__(self,
                exponent: _t.Union[Int, int],
                divisor: None = ...) -> _te.Self:
        ...

    def __radd__(self, other: _t.Union[Int, int]) -> _te.Self:
        ...

    def __rdivmod__(self,
                    dividend: _t.Union[Int, int]) -> _t.Tuple[Int, _te.Self]:
        ...

    def __repr__(self) -> str:
        ...

    def __rfloordiv__(self, dividend: _t.Union[Int, int]) -> Int:
        ...

    def __rmod__(self, dividend: _t.Union[Int, int]) -> _te.Self:
        ...

    def __rmul__(self, other: _t.Union[Int, int]) -> _te.Self:
        ...

    @_t.overload
    def __round__(self, digits: None = ...) -> Int:
        ...

    @_t.overload
    def __round__(self, digits: int) -> _te.Self:
        ...

    def __rsub__(self, subtrahend: _t.Union[Int, int]) -> _te.Self:
        ...

    def __setstate__(self, state: _t.Tuple[Int, Int]) -> None:
        ...

    def __str__(self) -> str:
        ...

    def __sub__(self, minuend: _t.Union[_te.Self, Int, int]) -> _te.Self:
        ...

    def __rtruediv__(self, dividend: _t.Union[Int, int]) -> _te.Self:
        ...

    def __truediv__(self, divisor: _t.Union[_te.Self, Int, int]) -> _te.Self:
        ...

    def __trunc__(self) -> Int:
        ...
