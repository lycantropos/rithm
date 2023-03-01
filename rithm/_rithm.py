from __future__ import annotations

import typing as _t
from enum import Enum as _Enum
from math import gcd as _gcd
from numbers import (Integral as _Integral,
                     Rational as _Rational)
from operator import mul as _mul
from sys import hash_info as _hash_info

import typing_extensions as _te


class _BaseEnum(_Enum):
    __module__ = 'rithm.enums'

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}.{self.name}'


class Endianness(_BaseEnum):
    BIG = 'big'
    LITTLE = 'little'


class TieBreaking(int, _BaseEnum):
    AWAY_FROM_ZERO = 0
    TO_EVEN = 1
    TO_ODD = 2
    TOWARD_ZERO = 3


@_te.final
@_Integral.register
class Int:
    @property
    def denominator(self) -> _te.Self:
        return _ONE

    @property
    def numerator(self) -> _te.Self:
        return self

    def bit_length(self) -> _te.Self:
        return Int(self._value.bit_length())

    def gcd(self, other: _te.Self) -> _te.Self:
        return Int(_gcd(self._value, other._value))

    def is_power_of_two(self) -> bool:
        return self._value > 0 and not (self._value & (self._value - 1))

    def to_bytes(self, endianness: Endianness) -> bytes:
        return self._value.to_bytes(_to_bytes_count(self._value),
                                    endianness.value,
                                    signed=True)

    @classmethod
    def from_bytes(cls, value: bytes, endianness: Endianness) -> _te.Self:
        return cls(int.from_bytes(value, endianness.value,
                                  signed=True))

    _value: int

    __module__ = 'rithm.integer'
    __slots__ = '_value',

    def __init_subclass__(cls, **_kwargs: _t.Any) -> _t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    @_t.overload
    def __new__(cls, _value: _t.Union[_te.Self, float, int] = ...) -> _te.Self:
        ...

    @_t.overload
    def __new__(cls, _value: str, _base: _t.Optional[int] = ...) -> _te.Self:
        ...

    def __new__(cls,
                _value: _t.Union[_te.Self, float, int, str] = 0,
                _base: _t.Optional[int] = None) -> _te.Self:
        self = super().__new__(cls)
        if _base is None:
            self._value = int(_value)
        elif isinstance(_value, str):
            self._value = int(_value, _base)
        else:
            raise TypeError((type(_value), type(_base)))
        return self

    def __abs__(self) -> _te.Self:
        return Int(abs(self._value))

    def __add__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value + other._value)
                if isinstance(other, Int)
                else self.__radd__(other))

    def __and__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value & other._value)
                if isinstance(other, Int)
                else self.__rand__(other))

    def __bool__(self) -> bool:
        return bool(self._value)

    def __ceil__(self) -> _te.Self:
        return self

    def __divmod__(
            self, other: _t.Union[_te.Self, int]
    ) -> _t.Tuple[_te.Self, _te.Self]:
        if not isinstance(other, (Int, int)):
            return NotImplemented
        quotient, remainder = (divmod(self._value, other._value)
                               if isinstance(other, Int)
                               else divmod(self._value, other))
        return Int(quotient), Int(remainder)

    @_t.overload
    def __eq__(self, other: _t.Union[_te.Self, int]) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __eq__(self, other: _t.Any) -> _t.Any:
        return (self._value == other._value
                if isinstance(other, Int)
                else (self._value == other
                      if isinstance(other, int)
                      else NotImplemented))

    def __float__(self) -> float:
        return float(self._value)

    def __floor__(self) -> _te.Self:
        return self

    def __floordiv__(self, other: _t.Union[Int, int]) -> _te.Self:
        return (Int(self._value // other._value)
                if isinstance(other, Int)
                else (Int(self._value // other)
                      if isinstance(other, int)
                      else NotImplemented))

    def __ge__(self, other: _t.Union[Int, int]) -> bool:
        return (self._value >= other._value
                if isinstance(other, Int)
                else (self._value >= other
                      if isinstance(other, int)
                      else NotImplemented))

    def __getstate__(self) -> int:
        return self._value

    def __gt__(self, other: _t.Union[_te.Self, int]) -> bool:
        return (self._value > other._value
                if isinstance(other, Int)
                else (self._value > other
                      if isinstance(other, int)
                      else NotImplemented))

    def __hash__(self) -> int:
        return hash(self._value)

    def __invert__(self) -> _te.Self:
        return Int(~self._value)

    def __index__(self) -> int:
        return self._value

    __int__ = __index__

    def __le__(self, other: _t.Union[_te.Self, int]) -> bool:
        return (self._value <= other._value
                if isinstance(other, Int)
                else (self._value <= other
                      if isinstance(other, int)
                      else NotImplemented))

    def __lshift__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value << other._value)
                if isinstance(other, Int)
                else (Int(self._value << other)
                      if isinstance(other, int)
                      else NotImplemented))

    def __lt__(self, other: _t.Union[Int, int]) -> bool:
        return (self._value < other._value
                if isinstance(other, Int)
                else (self._value < other
                      if isinstance(other, int)
                      else NotImplemented))

    def __mod__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value % other._value)
                if isinstance(other, Int)
                else (Int(self._value % other)
                      if isinstance(other, int)
                      else NotImplemented))

    def __mul__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value * other._value)
                if isinstance(other, Int)
                else self.__rmul__(other))

    def __neg__(self) -> _te.Self:
        return Int(-self._value)

    def __or__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value | other._value)
                if isinstance(other, Int)
                else self.__ror__(other))

    def __pos__(self) -> _te.Self:
        return self

    @_t.overload
    def __pow__(self,
                exponent: _t.Union[_te.Self, int],
                divisor: None) -> _t.Union[Fraction, _te.Self]:
        ...

    @_t.overload
    def __pow__(self,
                exponent: _t.Union[_te.Self, int],
                divisor: _t.Union[_te.Self, int] = ...) -> _te.Self:
        ...

    def __pow__(
            self,
            exponent: _t.Union[_te.Self, int],
            divisor: _t.Union[None, _te.Self, int] = None
    ) -> _t.Union[Fraction, _te.Self]:
        return (((Int(self._value ** int(exponent))
                  if exponent >= _ZERO
                  else Fraction(_ONE, self) ** -exponent)
                 if divisor is None
                 else (Int(pow(self._value, int(exponent), int(divisor)))
                       if isinstance(divisor, (Int, int))
                       else NotImplemented))
                if isinstance(exponent, (Int, int))
                else NotImplemented)

    def __radd__(self, other: int) -> _te.Self:
        return (Int(self._value + other)
                if isinstance(other, int)
                else NotImplemented)

    def __rand__(self, other: int) -> _te.Self:
        return (Int(self._value & other)
                if isinstance(other, int)
                else NotImplemented)

    def __rdivmod__(self, other: int) -> _t.Tuple[_te.Self, _te.Self]:
        if not isinstance(other, int):
            return NotImplemented
        quotient, remainder = divmod(other, self._value)
        return Int(quotient), Int(remainder)

    def __repr__(self) -> str:
        return f'{type(self).__qualname__}({self._value})'

    def __rfloordiv__(self, other: int) -> _te.Self:
        return (Int(other // self._value)
                if isinstance(other, int)
                else NotImplemented)

    def __rlshift__(self, other: int) -> _te.Self:
        return (Int(other << self._value)
                if isinstance(other, int)
                else NotImplemented)

    def __rmod__(self, other: int) -> _te.Self:
        return (Int(other % self._value)
                if isinstance(other, int)
                else NotImplemented)

    def __rmul__(self, other: int) -> _te.Self:
        return (Int(self._value * other)
                if isinstance(other, int)
                else NotImplemented)

    def __ror__(self, other: int) -> _te.Self:
        return (Int(self._value | other)
                if isinstance(other, int)
                else NotImplemented)

    def __round__(self, digits: _t.Optional[int] = None) -> _te.Self:
        return Int(round(self._value, digits))

    def __rpow__(
            self,
            base: int,
            divisor: _t.Union[None, _te.Self, int] = None
    ) -> _t.Union[Fraction, _te.Self]:
        return (Int(base).__pow__(self, divisor)
                if isinstance(base, int)
                else NotImplemented)

    def __rrshift__(self, other: int) -> _te.Self:
        return (Int(other >> self._value)
                if isinstance(other, int)
                else NotImplemented)

    def __rshift__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value >> other._value)
                if isinstance(other, Int)
                else (Int(self._value >> other)
                      if isinstance(other, int)
                      else NotImplemented))

    def __rsub__(self, other: int) -> _te.Self:
        return (Int(other - self._value)
                if isinstance(other, int)
                else NotImplemented)

    def __rtruediv__(self, other: int) -> Fraction:
        return (Fraction(other, self)
                if isinstance(other, int)
                else NotImplemented)

    def __rxor__(self, other: int) -> _te.Self:
        return (Int(self._value ^ other)
                if isinstance(other, int)
                else NotImplemented)

    def __setstate__(self, state: int) -> None:
        self._value = state

    def __str__(self) -> str:
        return str(self._value)

    def __sub__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value - other._value)
                if isinstance(other, Int)
                else (Int(self._value - other)
                      if isinstance(other, int)
                      else NotImplemented))

    def __truediv__(self, other: _t.Union[_te.Self, int]) -> Fraction:
        return (Fraction(self, other)
                if isinstance(other, (Int, int))
                else NotImplemented)

    def __trunc__(self) -> _te.Self:
        return self

    def __xor__(self, other: _t.Union[_te.Self, int]) -> _te.Self:
        return (Int(self._value ^ other._value)
                if isinstance(other, Int)
                else self.__rxor__(other))


def _to_bytes_count(value: int) -> int:
    return (8 + (value + (value < 0)).bit_length()) // 8


_ONE = Int(1)
_ZERO = Int()
_HASH_INF = Int(_hash_info.inf)
_HASH_MODULUS = Int(_hash_info.modulus)


@_te.final
@_Rational.register
class Fraction:
    @property
    def denominator(self) -> Int:
        return self._denominator

    @property
    def numerator(self) -> Int:
        return self._numerator

    def round(self, tie_breaking: TieBreaking) -> Int:
        quotient, remainder = divmod(self.numerator, self.denominator)
        double_remainder = remainder * 2
        if double_remainder == self.denominator:
            if tie_breaking is TieBreaking.AWAY_FROM_ZERO:
                return quotient + _ONE if quotient >= 0 else quotient
            elif tie_breaking is TieBreaking.TO_EVEN:
                return quotient + _ONE if quotient % 2 else quotient
            elif tie_breaking is TieBreaking.TO_ODD:
                return quotient + _ONE if not quotient % 2 else quotient
            else:
                assert tie_breaking is TieBreaking.TOWARD_ZERO, (
                    tie_breaking
                )
                return quotient + _ONE if quotient < 0 else quotient
        else:
            return (quotient + _ONE
                    if double_remainder > self.denominator
                    else quotient)

    _denominator: Int
    _numerator: Int

    __module__ = 'rithm.fraction'
    __slots__ = '_denominator', '_numerator'

    def __init_subclass__(cls, **_kwargs: _t.Any) -> _t.NoReturn:
        raise TypeError(f'type {cls.__qualname__!r} '
                        'is not an acceptable base type')

    def __new__(cls,
                _numerator: _t.Union[Int, int, float] = _ZERO,
                _denominator: _t.Union[Int, None, int] = None,
                *,
                _normalize: bool = True) -> _te.Self:
        self = super().__new__(cls)
        if _denominator is None:
            if isinstance(_numerator, Fraction):
                numerator, denominator = (_numerator.numerator,
                                          _numerator.denominator)
            elif isinstance(_numerator, float):
                raw_numerator, raw_denominator = (
                    _numerator.as_integer_ratio()
                )
                numerator, denominator = (Int(raw_numerator),
                                          Int(raw_denominator))
            elif isinstance(_numerator, (Int, int)):
                numerator, denominator = Int(_numerator), _ONE
            elif isinstance(_numerator, _Rational):
                numerator, denominator = (Int(_numerator.numerator),
                                          Int(_numerator.denominator))
            else:
                raise TypeError('First argument should be of '
                                f'type {Int}, {int} or {float}, '
                                f'but found: {type(_numerator)}.')
        elif isinstance(_denominator, (Int, int)):
            if not isinstance(_numerator, (Int, int)):
                raise TypeError(f'Numerator should be '
                                f'of type {Int} or {int}, '
                                f'but found: {type(_numerator)}.')
            numerator, denominator = Int(_numerator), Int(_denominator)
        else:
            raise TypeError(f'Denominator should be '
                            f'of type {Int} or {int}, '
                            f'but found: {type(_denominator)}.')
        if not denominator:
            raise ZeroDivisionError('Denominator should not be zero.')
        if _normalize:
            numerator, denominator = _normalize_components_sign(
                    *_normalize_components_moduli(numerator, denominator)
            )
        self._numerator, self._denominator = numerator, denominator
        return self

    def __abs__(self) -> _te.Self:
        return Fraction(abs(self.numerator), self.denominator,
                        _normalize=False)

    def __add__(self, other: _t.Union[_te.Self, Int, int]) -> _te.Self:
        return (self._add_fraction(other)
                if isinstance(other, Fraction)
                else self.__radd__(other))

    def __bool__(self) -> bool:
        return bool(self.numerator)

    def __ceil__(self) -> Int:
        return -(-self.numerator // self.denominator)

    def __divmod__(
            self, divisor: _t.Union[_te.Self, Int, int]
    ) -> _t.Tuple[Int, _te.Self]:
        return (_divmod_rationals(self, divisor)
                if isinstance(divisor, (Fraction, Int, int))
                else NotImplemented)

    @_t.overload
    def __eq__(self, other: _t.Union[Int, _te.Self, int]) -> bool:
        ...

    @_t.overload
    def __eq__(self, other: _t.Any) -> _t.Any:
        ...

    def __eq__(self, other: _t.Any) -> _t.Any:
        return (self.numerator == other.numerator
                and self.denominator == other.denominator
                if isinstance(other, Fraction)
                else (self.denominator == _ONE
                      and self.numerator == other
                      if isinstance(other, (Int, int))
                      else NotImplemented))

    def __float__(self) -> float:
        return int(self.numerator) / int(self.denominator)

    def __floor__(self) -> Int:
        return self.numerator // self.denominator

    def __floordiv__(self, divisor: _t.Union[Int, _te.Self, int]) -> Int:
        return ((self.numerator * divisor.denominator)
                // (self.denominator * divisor.numerator)
                if isinstance(divisor, Fraction)
                else (self.numerator // (self.denominator * divisor)
                      if isinstance(divisor, (Int, int))
                      else NotImplemented))

    def __ge__(self, other: _t.Union[Int, _te.Self, int]) -> bool:
        return (self.numerator * other.denominator
                >= other.numerator * self.denominator
                if isinstance(other, Fraction)
                else (self.numerator >= other * self.denominator
                      if isinstance(other, (Int, int))
                      else NotImplemented))

    def __getstate__(self) -> _t.Tuple[Int, Int]:
        return self._numerator, self._denominator

    def __gt__(self, other: _t.Union[_te.Self, Int, int]) -> bool:
        return (self.numerator * other.denominator
                > other.numerator * self.denominator
                if isinstance(other, Fraction)
                else (self.numerator > other * self.denominator
                      if isinstance(other, (Int, int))
                      else NotImplemented))

    def __hash__(self) -> int:
        inverted_denominator = self._denominator.__pow__(_HASH_MODULUS - 2,
                                                         _HASH_MODULUS)
        result = (((abs(self._numerator) * inverted_denominator)
                   % _HASH_MODULUS)
                  if inverted_denominator
                  else _HASH_INF)
        result = result if self >= 0 else -result
        return -2 if result == -1 else int(result)

    def __le__(self, other: _t.Union[Int, _te.Self, int]) -> bool:
        return (self.numerator * other.denominator
                <= other.numerator * self.denominator
                if isinstance(other, Fraction)
                else (self.numerator <= other * self.denominator
                      if isinstance(other, (Int, int))
                      else NotImplemented))

    def __lt__(self, other: _t.Union[Int, _te.Self, int]) -> bool:
        return (self.numerator * other.denominator
                < other.numerator * self.denominator
                if isinstance(other, Fraction)
                else (self.numerator < other * self.denominator
                      if isinstance(other, (Int, int))
                      else NotImplemented))

    def __mod__(self, divisor: _t.Union[Int, _te.Self, int]) -> _te.Self:
        return (Fraction((self.numerator * divisor.denominator)
                         % (self.denominator * divisor.numerator),
                         self.denominator * divisor.denominator)
                if isinstance(divisor, Fraction)
                else (Fraction(self.numerator
                               % (self.denominator * divisor),
                               self.denominator)
                      if isinstance(divisor, (Int, int))
                      else NotImplemented))

    def __mul__(self, other: _t.Union[Int, _te.Self, int]) -> _te.Self:
        return (self._mul_by_fraction(other)
                if isinstance(other, Fraction)
                else self.__rmul__(other))

    def __neg__(self) -> _te.Self:
        return Fraction(-self.numerator, self.denominator,
                        _normalize=False)

    def __pos__(self) -> _te.Self:
        return self

    def __pow__(self,
                exponent: _t.Union[Int, int],
                divisor: None = None) -> _te.Self:
        return ((Fraction(self.numerator ** exponent,
                          self.denominator ** exponent,
                          _normalize=False)
                 if exponent >= _ZERO
                 else
                 Fraction(
                         *_normalize_components_sign(
                                 self.denominator ** -exponent,
                                 self.numerator ** -exponent
                         ),
                         _normalize=False
                 ))
                if isinstance(exponent, (Int, int)) and divisor is None
                else NotImplemented)

    def __radd__(self, other: _t.Union[Int, int]) -> _te.Self:
        return (self._add_int(Int(other))
                if isinstance(other, (Int, int))
                else NotImplemented)

    def __rdivmod__(self,
                    dividend: _t.Union[Int, int]) -> _t.Tuple[Int, _te.Self]:
        return (_divmod_rationals(dividend, self)
                if isinstance(dividend, (Int, int))
                else NotImplemented)

    def __repr__(self) -> str:
        return (f'{type(self).__qualname__}'
                f'({self.numerator!r}, {self.denominator!r})')

    def __rfloordiv__(self, dividend: _t.Union[Int, int]) -> Int:
        return ((dividend * self.denominator) // self.numerator
                if isinstance(dividend, (Int, int))
                else NotImplemented)

    def __rmod__(self, dividend: _t.Union[Int, int]) -> _te.Self:
        return (Fraction((dividend * self.denominator) % self.numerator,
                         self.denominator)
                if isinstance(dividend, (Int, int))
                else NotImplemented)

    def __rmul__(self, other: _t.Union[Int, int]) -> _te.Self:
        return (self._mul_by_int(Int(other))
                if isinstance(other, (Int, int))
                else NotImplemented)

    @_t.overload
    def __round__(self, digits: None = ...) -> Int:
        ...

    @_t.overload
    def __round__(self, digits: int) -> _te.Self:
        ...

    def __round__(self,
                  digits: _t.Optional[int] = None) -> _t.Union[Int, _te.Self]:
        if digits is None:
            return self.round(TieBreaking.TO_EVEN)
        else:
            shift = 10 ** abs(digits)
            return (Fraction((self * shift).round(TieBreaking.TO_EVEN),
                             shift)
                    if digits > 0
                    else Fraction((self / shift).round(TieBreaking.TO_EVEN)
                                  * shift))

    def __rsub__(self, subtrahend: _t.Union[Int, int]) -> _te.Self:
        return (
            Fraction(
                    *_normalize_components_moduli(
                            subtrahend * self.denominator - self.numerator,
                            self.denominator
                    ),
                    _normalize=False
            )
            if isinstance(subtrahend, (Int, int))
            else NotImplemented
        )

    def __setstate__(self, state: _t.Tuple[Int, Int]) -> None:
        self._numerator, self._denominator = state

    def __str__(self) -> str:
        return (str(self.numerator)
                if self.denominator == _ONE
                else f'{self.numerator}/{self.denominator}')

    def __sub__(self, minuend: _t.Union[Int, _te.Self, int]) -> _te.Self:
        return (Fraction(
                *_normalize_components_moduli(
                        self.numerator * minuend.denominator
                        - minuend.numerator * self.denominator,
                        self.denominator * minuend.denominator
                ),
                _normalize=False)
                if isinstance(minuend, Fraction)
                else
                (Fraction(
                        *_normalize_components_moduli(
                                self.numerator - minuend * self.denominator,
                                self.denominator
                        ),
                        _normalize=False
                )
                 if isinstance(minuend, (Int, int))
                 else NotImplemented))

    def __rtruediv__(self, dividend: _t.Union[Int, int]) -> _te.Self:
        return (self._rtruediv_by_int(Int(dividend))
                if isinstance(dividend, (Int, int))
                else NotImplemented)

    def __truediv__(self, divisor: _t.Union[Int, _te.Self, int]) -> _te.Self:
        return (
            Fraction(
                    *_normalize_components_sign(
                            *map(_mul,
                                 _normalize_components_moduli(
                                         self.numerator, divisor.numerator
                                 ),
                                 _normalize_components_moduli(
                                         divisor.denominator,
                                         self.denominator
                                 ))
                    ),
                    _normalize=False
            )
            if isinstance(divisor, Fraction)
            else (self._truediv_by_int(Int(divisor))
                  if isinstance(divisor, (Int, int))
                  else NotImplemented)
        )

    def __trunc__(self) -> Int:
        return (self.__ceil__()
                if self.numerator < _ZERO
                else self.__floor__())

    def _add_fraction(self, other: _te.Self) -> _te.Self:
        return Fraction(
                *_normalize_components_moduli(
                        self.numerator * other.denominator
                        + other.numerator * self.denominator,
                        self.denominator * other.denominator
                ),
                _normalize=False
        )

    def _add_int(self, other: Int) -> _te.Self:
        return Fraction(
                *_normalize_components_moduli(self.numerator
                                              + other * self.denominator,
                                              self.denominator),
                _normalize=False
        )

    def _mul_by_fraction(self, other: _te.Self) -> _te.Self:
        numerator, other_denominator = _normalize_components_moduli(
                self.numerator, other.denominator)
        other_numerator, denominator = _normalize_components_moduli(
                other.numerator, self.denominator)
        return Fraction(numerator * other_numerator,
                        denominator * other_denominator,
                        _normalize=False)

    def _mul_by_int(self, other: Int) -> _te.Self:
        other, denominator = _normalize_components_moduli(other,
                                                          self.denominator)
        return Fraction(self.numerator * other, denominator,
                        _normalize=False)

    def _rtruediv_by_int(self, dividend: Int) -> _te.Self:
        dividend, numerator = _normalize_components_moduli(dividend,
                                                           self.numerator)
        return Fraction(
                *_normalize_components_sign(dividend * self.denominator,
                                            numerator),
                _normalize=False
        )

    def _truediv_by_int(self, divisor: Int) -> _te.Self:
        numerator, divisor = _normalize_components_moduli(self.numerator,
                                                          divisor)
        return Fraction(
                *_normalize_components_sign(numerator,
                                            divisor * self.denominator),
                _normalize=False
        )


def _divmod_rationals(
        dividend: _t.Union[Fraction, Int, int],
        divisor: _t.Union[Fraction, Int, int]
) -> _t.Tuple[Int, Fraction]:
    quotient, remainder_numerator = divmod(
            dividend.numerator * divisor.denominator,
            dividend.denominator * divisor.numerator
    )
    assert isinstance(quotient, Int)
    return quotient, Fraction(remainder_numerator,
                              dividend.denominator * divisor.denominator)


def _normalize_components_moduli(numerator: Int,
                                 denominator: Int) -> _t.Tuple[Int, Int]:
    gcd = numerator.gcd(denominator)
    return numerator // gcd, denominator // gcd


def _normalize_components_sign(numerator: Int,
                               denominator: Int) -> _t.Tuple[Int, Int]:
    return ((-numerator, -denominator)
            if denominator < _ZERO
            else (numerator, denominator))
