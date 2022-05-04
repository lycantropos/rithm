"""Arbitrary precision arithmetic."""

__version__ = '6.0.0'

try:
    from ._rithm import (Endianness,
                         Fraction,
                         Int,
                         TieBreaking)
except ImportError:
    from enum import Enum as _Enum
    from math import gcd as _gcd
    from numbers import (Integral as _Integral,
                         Rational as _Rational)
    from operator import mul as _mul
    from typing import (Tuple as _Tuple,
                        Union as _Union)


    class _BaseEnum(_Enum):
        def __repr__(self):
            return f'rithm.{type(self).__qualname__}.{self.name}'


    class Endianness(_BaseEnum):
        BIG = 'big'
        LITTLE = 'little'


    class TieBreaking(int, _BaseEnum):
        AWAY_FROM_ZERO = 0
        TO_EVEN = 1
        TO_ODD = 2
        TOWARD_ZERO = 3


    @_Integral.register
    class Int:
        @property
        def denominator(self):
            return _ONE

        @property
        def numerator(self):
            return self

        def bit_length(self):
            return Int(self._value.bit_length())

        def gcd(self, other):
            return Int(_gcd(self._value, other._value))

        def to_bytes(self, endianness):
            return self._value.to_bytes(_to_bytes_count(self._value),
                                        endianness.value,
                                        signed=True)

        @classmethod
        def from_bytes(cls, value, endianness):
            return cls(int.from_bytes(value, endianness.value,
                                      signed=True))

        __slots__ = '_value',

        def __new__(cls, _value=0, _base=None):
            self = super().__new__(cls)
            self._value = (int(_value)
                           if _base is None
                           else int(_value, _base))
            return self

        def __abs__(self):
            return Int(abs(self._value))

        def __add__(self, other):
            return (Int(self._value + other._value)
                    if isinstance(other, Int)
                    else self.__radd__(other))

        def __and__(self, other):
            return (Int(self._value & other._value)
                    if isinstance(other, Int)
                    else self.__rand__(other))

        def __bool__(self):
            return bool(self._value)

        def __ceil__(self):
            return self

        def __divmod__(self, other):
            return (tuple(Int(value)
                          for value in divmod(self._value, other._value))
                    if isinstance(other, Int)
                    else (tuple(Int(value)
                                for value in divmod(self._value, other))
                          if isinstance(other, int)
                          else NotImplemented))

        def __eq__(self, other):
            return (self._value == other._value
                    if isinstance(other, Int)
                    else (self._value == other
                          if isinstance(other, int)
                          else NotImplemented))

        def __float__(self):
            return float(self._value)

        def __floor__(self):
            return self

        def __floordiv__(self, other):
            return (Int(self._value // other._value)
                    if isinstance(other, Int)
                    else (Int(self._value // other)
                          if isinstance(other, int)
                          else NotImplemented))

        def __ge__(self, other):
            return (self._value >= other._value
                    if isinstance(other, Int)
                    else (self._value >= other
                          if isinstance(other, int)
                          else NotImplemented))

        def __getstate__(self):
            return self._value

        def __gt__(self, other):
            return (self._value > other._value
                    if isinstance(other, Int)
                    else (self._value > other
                          if isinstance(other, int)
                          else NotImplemented))

        def __hash__(self):
            return hash(self._value)

        def __invert__(self):
            return Int(~self._value)

        def __int__(self):
            return self._value

        def __le__(self, other):
            return (self._value <= other._value
                    if isinstance(other, Int)
                    else (self._value <= other
                          if isinstance(other, int)
                          else NotImplemented))

        def __lshift__(self, other):
            return (Int(self._value << other._value)
                    if isinstance(other, Int)
                    else (Int(self._value << other)
                          if isinstance(other, int)
                          else NotImplemented))

        def __lt__(self, other):
            return (self._value < other._value
                    if isinstance(other, Int)
                    else (self._value < other
                          if isinstance(other, int)
                          else NotImplemented))

        def __mod__(self, other):
            return (Int(self._value % other._value)
                    if isinstance(other, Int)
                    else (Int(self._value % other)
                          if isinstance(other, int)
                          else NotImplemented))

        def __mul__(self, other):
            return (Int(self._value * other._value)
                    if isinstance(other, Int)
                    else self.__rmul__(other))

        def __neg__(self):
            return Int(-self._value)

        def __or__(self, other):
            return (Int(self._value | other._value)
                    if isinstance(other, Int)
                    else self.__ror__(other))

        def __pos__(self):
            return self

        def __pow__(self, exponent, divisor=None):
            return (((Int(self._value ** int(exponent))
                      if exponent >= _ZERO
                      else Fraction(_ONE, self) ** -exponent)
                     if divisor is None
                     else (Int(pow(self._value, int(exponent), int(divisor)))
                           if isinstance(divisor, (Int, int))
                           else NotImplemented))
                    if isinstance(exponent, (Int, int))
                    else NotImplemented)

        def __radd__(self, other):
            return (Int(self._value + other)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rand__(self, other):
            return (Int(self._value & other)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rdivmod__(self, other):
            return (tuple(Int(value) for value in divmod(other, self._value))
                    if isinstance(other, int)
                    else NotImplemented)

        def __repr__(self):
            return f'rithm.Int({self._value})'

        def __rfloordiv__(self, other):
            return (Int(other // self._value)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rlshift__(self, other):
            return (Int(other << self._value)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rmod__(self, other):
            return (Int(other % self._value)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rmul__(self, other):
            return (Int(self._value * other)
                    if isinstance(other, int)
                    else NotImplemented)

        def __ror__(self, other):
            return (Int(self._value | other)
                    if isinstance(other, int)
                    else NotImplemented)

        def __round__(self, digits=None):
            return Int(round(self._value, digits))

        def __rpow__(self, base, divisor=None):
            return (Int(base).__pow__(self)
                    if isinstance(base, int)
                    else NotImplemented)

        def __rrshift__(self, other):
            return (Int(other >> self._value)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rshift__(self, other):
            return (Int(self._value >> other._value)
                    if isinstance(other, Int)
                    else (Int(self._value >> other)
                          if isinstance(other, int)
                          else NotImplemented))

        def __rsub__(self, other):
            return (Int(other - self._value)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rtruediv__(self, other):
            return (Fraction(other, self)
                    if isinstance(other, int)
                    else NotImplemented)

        def __rxor__(self, other):
            return (Int(self._value ^ other)
                    if isinstance(other, int)
                    else NotImplemented)

        def __setstate__(self, state: int):
            self._value = state

        def __str__(self):
            return str(self._value)

        def __sub__(self, other):
            return (Int(self._value - other._value)
                    if isinstance(other, Int)
                    else (Int(self._value - other)
                          if isinstance(other, int)
                          else NotImplemented))

        def __truediv__(self, other):
            return (Fraction(self, other)
                    if isinstance(other, (Int, int))
                    else NotImplemented)

        def __trunc__(self):
            return self

        def __xor__(self, other):
            return (Int(self._value ^ other._value)
                    if isinstance(other, Int)
                    else self.__rxor__(other))


    def _to_bytes_count(value: int) -> int:
        return (8 + (value + (value < 0)).bit_length()) // 8


    _ONE = Int(1)
    _ZERO = Int()


    @_Rational.register
    class Fraction:
        @property
        def denominator(self):
            return self._denominator

        @property
        def numerator(self):
            return self._numerator

        def round(self, tie_breaking):
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

        __slots__ = '_denominator', '_numerator'

        def __new__(cls,
                    _numerator=_ZERO,
                    _denominator=None,
                    *,
                    _normalize=True):
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
                    numerator, denominator = (_numerator.numerator,
                                              _numerator.denominator)
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

        def __abs__(self):
            return Fraction(abs(self.numerator), self.denominator,
                            _normalize=False)

        def __add__(self, other):
            return (self._add_fraction(other)
                    if isinstance(other, Fraction)
                    else self.__radd__(other))

        def __bool__(self):
            return bool(self.numerator)

        def __ceil__(self):
            return -(-self.numerator // self.denominator)

        def __divmod__(self, divisor):
            return (_divmod_rationals(self, divisor)
                    if isinstance(divisor, (Fraction, Int, int))
                    else NotImplemented)

        def __eq__(self, other):
            return (self.numerator == other.numerator
                    and self.denominator == other.denominator
                    if isinstance(other, Fraction)
                    else (self.denominator == _ONE
                          and self.numerator == other
                          if isinstance(other, (Int, int))
                          else NotImplemented))

        def __float__(self):
            return int(self.numerator) / int(self.denominator)

        def __floor__(self):
            return self.numerator // self.denominator

        def __floordiv__(self, divisor):
            return ((self.numerator * divisor.denominator)
                    // (self.denominator * divisor.numerator)
                    if isinstance(divisor, Fraction)
                    else (self.numerator // (self.denominator * divisor)
                          if isinstance(divisor, (Int, int))
                          else NotImplemented))

        def __ge__(self, other):
            return (self.numerator * other.denominator
                    >= other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator >= other * self.denominator
                          if isinstance(other, (Int, int))
                          else NotImplemented))

        def __getstate__(self):
            return self._numerator, self._denominator

        def __gt__(self, other):
            return (self.numerator * other.denominator
                    > other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator > other * self.denominator
                          if isinstance(other, (Int, int))
                          else NotImplemented))

        def __le__(self, other):
            return (self.numerator * other.denominator
                    <= other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator <= other * self.denominator
                          if isinstance(other, (Int, int))
                          else NotImplemented))

        def __lt__(self, other):
            return (self.numerator * other.denominator
                    < other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator < other * self.denominator
                          if isinstance(other, (Int, int))
                          else NotImplemented))

        def __mod__(self, divisor):
            return (Fraction((self.numerator * divisor.denominator)
                             % (self.denominator * divisor.numerator),
                             self.denominator * divisor.denominator)
                    if isinstance(divisor, Fraction)
                    else (Fraction(self.numerator
                                   % (self.denominator * divisor),
                                   self.denominator)
                          if isinstance(divisor, (Int, int))
                          else NotImplemented))

        def __mul__(self, other):
            return (self._mul_by_fraction(other)
                    if isinstance(other, Fraction)
                    else self.__rmul__(other))

        def __neg__(self):
            return Fraction(-self.numerator, self.denominator,
                            _normalize=False)

        def __pos__(self):
            return self

        def __pow__(self, exponent, divisor=None):
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

        def __radd__(self, other):
            return (self._add_int(Int(other))
                    if isinstance(other, (Int, int))
                    else NotImplemented)

        def __rdivmod__(self, dividend):
            return (_divmod_rationals(dividend, self)
                    if isinstance(dividend, (Int, int))
                    else NotImplemented)

        def __repr__(self):
            return f'rithm.Fraction({self.numerator!r}, {self.denominator!r})'

        def __rfloordiv__(self, dividend):
            return ((dividend * self.denominator) // self.numerator
                    if isinstance(dividend, (Int, int))
                    else NotImplemented)

        def __rmod__(self, dividend):
            return (Fraction((dividend * self.denominator) % self.numerator,
                             self.denominator)
                    if isinstance(dividend, (Int, int))
                    else NotImplemented)

        def __rmul__(self, other):
            return (self._mul_by_int(Int(other))
                    if isinstance(other, (Int, int))
                    else NotImplemented)

        def __round__(self, digits=None):
            if digits is None:
                return self.round(TieBreaking.TO_EVEN)
            else:
                shift = 10 ** abs(digits)
                return (Fraction((self * shift).round(TieBreaking.TO_EVEN),
                                 shift)
                        if digits > 0
                        else Fraction((self / shift).round(TieBreaking.TO_EVEN)
                                      * shift))

        def __rsub__(self, subtrahend):
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

        def __setstate__(self, state):
            self._numerator, self._denominator = state

        def __str__(self):
            return (str(self.numerator)
                    if self.denominator == _ONE
                    else f'{self.numerator}/{self.denominator}')

        def __sub__(self, minuend):
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

        def __rtruediv__(self, dividend):
            return (self._rtruediv_by_int(Int(dividend))
                    if isinstance(dividend, (Int, int))
                    else NotImplemented)

        def __truediv__(self, divisor):
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

        def __trunc__(self):
            return (self.__ceil__()
                    if self.numerator < _ZERO
                    else self.__floor__())

        def _add_fraction(self, other: 'Fraction') -> 'Fraction':
            return Fraction(
                    *_normalize_components_moduli(
                            self.numerator * other.denominator
                            + other.numerator * self.denominator,
                            self.denominator * other.denominator
                    ),
                    _normalize=False
            )

        def _add_int(self, other: Int) -> 'Fraction':
            return Fraction(
                    *_normalize_components_moduli(self.numerator
                                                  + other * self.denominator,
                                                  self.denominator),
                    _normalize=False
            )

        def _mul_by_fraction(self, other: 'Fraction') -> 'Fraction':
            numerator, other_denominator = _normalize_components_moduli(
                    self.numerator, other.denominator)
            other_numerator, denominator = _normalize_components_moduli(
                    other.numerator, self.denominator)
            return Fraction(numerator * other_numerator,
                            denominator * other_denominator,
                            _normalize=False)

        def _mul_by_int(self, other: Int) -> 'Fraction':
            other, denominator = _normalize_components_moduli(other,
                                                              self.denominator)
            return Fraction(self.numerator * other, denominator,
                            _normalize=False)

        def _rtruediv_by_int(self, dividend: Int) -> 'Fraction':
            dividend, numerator = _normalize_components_moduli(dividend,
                                                               self.numerator)
            return Fraction(
                    *_normalize_components_sign(dividend * self.denominator,
                                                numerator),
                    _normalize=False
            )

        def _truediv_by_int(self, divisor: Int) -> 'Fraction':
            numerator, divisor = _normalize_components_moduli(self.numerator,
                                                              divisor)
            return Fraction(
                    *_normalize_components_sign(numerator,
                                                divisor * self.denominator),
                    _normalize=False
            )


    def _divmod_rationals(dividend: _Union[Fraction, Int, int],
                          divisor: _Union[Fraction, Int, int]
                          ) -> _Tuple[Int, Fraction]:
        quotient, remainder_numerator = divmod(
                dividend.numerator * divisor.denominator,
                dividend.denominator * divisor.numerator
        )
        return quotient, Fraction(remainder_numerator,
                                  dividend.denominator * divisor.denominator)


    def _normalize_components_moduli(numerator: Int, denominator: Int
                                     ) -> _Tuple[Int, Int]:
        gcd = numerator.gcd(denominator)
        return numerator // gcd, denominator // gcd


    def _normalize_components_sign(numerator: Int, denominator: Int
                                   ) -> _Tuple[Int, Int]:
        return ((-numerator, -denominator)
                if denominator < _ZERO
                else (numerator, denominator))
