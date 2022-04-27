"""Arbitrary precision arithmetic."""

__version__ = '5.4.0'

try:
    from ._rithm import (Endianness,
                         Fraction,
                         Int)
except ImportError:
    from enum import Enum as _Enum
    from math import gcd as _gcd
    from operator import mul as _mul
    from typing import Tuple as _Tuple


    class Endianness(_Enum):
        BIG = 'big'
        LITTLE = 'little'

        def __repr__(self):
            return f'rithm.{type(self).__qualname__}.{self.name}'


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
                    else NotImplemented)

        def __and__(self, other):
            return (Int(self._value & other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __bool__(self):
            return bool(self._value)

        def __ceil__(self):
            return self

        def __divmod__(self, other):
            return (tuple(Int(value)
                          for value in divmod(self._value, other._value))
                    if isinstance(other, Int)
                    else NotImplemented)

        def __eq__(self, other):
            return (self._value == other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __float__(self):
            return float(self._value)

        def __floor__(self):
            return self

        def __floordiv__(self, other):
            return (Int(self._value // other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __ge__(self, other):
            return (self._value >= other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __getstate__(self):
            return self._value

        def __gt__(self, other):
            return (self._value > other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __hash__(self):
            return hash(self._value)

        def __invert__(self):
            return Int(~self._value)

        def __int__(self):
            return self._value

        def __le__(self, other):
            return (self._value <= other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __lshift__(self, other):
            return (Int(self._value << other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __lt__(self, other):
            return (self._value < other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __mod__(self, other):
            return (Int(self._value % other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __mul__(self, other):
            return (Int(self._value * other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __neg__(self):
            return Int(-self._value)

        def __or__(self, other):
            return (Int(self._value | other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __pos__(self):
            return self

        def __pow__(self, exponent, divisor=None):
            return (((Int(self._value ** exponent._value)
                      if exponent >= _ZERO
                      else Fraction(_ONE, self) ** -exponent)
                     if divisor is None
                     else (Int(pow(self._value, exponent._value,
                                   divisor._value))
                           if isinstance(divisor, Int)
                           else NotImplemented))
                    if isinstance(exponent, Int)
                    else NotImplemented)

        def __repr__(self):
            return f'rithm.Int({self._value})'

        def __rshift__(self, other):
            return (Int(self._value >> other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __setstate__(self, state: int):
            self._value = state

        def __str__(self):
            return str(self._value)

        def __sub__(self, other):
            return (Int(self._value - other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __truediv__(self, other):
            return (Fraction(self, other)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __trunc__(self):
            return self

        def __xor__(self, other):
            return (Int(self._value ^ other._value)
                    if isinstance(other, Int)
                    else NotImplemented)


    def _to_bytes_count(value: int) -> int:
        return (8 + (value + (value < 0)).bit_length()) // 8


    _ONE = Int(1)
    _ZERO = Int()


    class Fraction:
        @property
        def denominator(self):
            return self._denominator

        @property
        def numerator(self):
            return self._numerator

        __slots__ = '_denominator', '_numerator'

        def __new__(cls,
                    _numerator=_ZERO,
                    _denominator=None,
                    *,
                    _normalize=True):
            self = super().__new__(cls)
            if _denominator is None:
                if isinstance(_numerator, float):
                    raw_numerator, raw_denominator = (
                        _numerator.as_integer_ratio()
                    )
                    numerator, denominator = (Int(raw_numerator),
                                              Int(raw_denominator))
                elif isinstance(_numerator, (Int, int)):
                    numerator, denominator = Int(_numerator), _ONE
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
                    else (self._add_int(other)
                          if isinstance(other, Int)
                          else NotImplemented))

        def __bool__(self):
            return bool(self.numerator)

        def __eq__(self, other):
            return (self.numerator == other.numerator
                    and self.denominator == other.denominator
                    if isinstance(other, Fraction)
                    else (self.denominator == _ONE
                          and self.numerator == other
                          if isinstance(other, Int)
                          else NotImplemented))

        def __float__(self):
            return self._numerator._value / self._denominator._value

        def __floordiv__(self, other):
            return ((self.numerator * other.denominator)
                    // (self.denominator * other.numerator)
                    if isinstance(other, Fraction)
                    else (self.numerator // (self.denominator * other)
                          if isinstance(other, Int)
                          else NotImplemented))

        def __ge__(self, other):
            return (self.numerator * other.denominator
                    >= other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator >= other * self.denominator
                          if isinstance(other, Int)
                          else NotImplemented))

        def __getstate__(self):
            return self._numerator, self._denominator

        def __gt__(self, other):
            return (self.numerator * other.denominator
                    > other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator > other * self.denominator
                          if isinstance(other, Int)
                          else NotImplemented))

        def __le__(self, other):
            return (self.numerator * other.denominator
                    <= other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator <= other * self.denominator
                          if isinstance(other, Int)
                          else NotImplemented))

        def __lt__(self, other):
            return (self.numerator * other.denominator
                    < other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else (self.numerator < other * self.denominator
                          if isinstance(other, Int)
                          else NotImplemented))

        def __mod__(self, other):
            return (Fraction((self.numerator * other.denominator)
                             % (self.denominator * other.numerator),
                             self.denominator * other.denominator)
                    if isinstance(other, Fraction)
                    else (Fraction(self.numerator % (self.denominator * other),
                                   self.denominator)
                          if isinstance(other, Int)
                          else NotImplemented))

        def __mul__(self, other):
            return (self._mul_by_fraction(other)
                    if isinstance(other, Fraction)
                    else (self._mul_by_int(other)
                          if isinstance(other, Int)
                          else NotImplemented))

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
                    if isinstance(exponent, Int) and divisor is None
                    else NotImplemented)

        def __repr__(self):
            return f'rithm.Fraction({self.numerator!r}, {self.denominator!r})'

        def __radd__(self, other):
            return (self._add_int(other)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __rfloordiv__(self, other):
            return ((other * self.denominator) // self.numerator
                    if isinstance(other, Int)
                    else NotImplemented)

        def __rmod__(self, other):
            return (Fraction((other * self.denominator) % self.numerator,
                             self.denominator)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __rmul__(self, other):
            return (self._mul_by_int(other)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __rsub__(self, other):
            return (
                Fraction(
                        *_normalize_components_moduli(other * self.denominator
                                                      - self.numerator,
                                                      self.denominator),
                        _normalize=False
                )
                if isinstance(other, Int)
                else NotImplemented
            )

        def __setstate__(self, state):
            self._numerator, self._denominator = state

        def __str__(self):
            return (str(self.numerator)
                    if self.denominator == _ONE
                    else f'{self.numerator}/{self.denominator}')

        def __sub__(self, other):
            return (Fraction(
                    *_normalize_components_moduli(
                            self.numerator * other.denominator
                            - other.numerator * self.denominator,
                            self.denominator * other.denominator
                    ),
                    _normalize=False)
                    if isinstance(other, Fraction)
                    else
                    (Fraction(
                            *_normalize_components_moduli(
                                    self.numerator - other * self.denominator,
                                    self.denominator
                            ),
                            _normalize=False
                    )
                     if isinstance(other, Int)
                     else NotImplemented))

        def __rtruediv__(self, other):
            return (self._rtruediv_by_int(other)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __truediv__(self, other):
            return (
                Fraction(
                        *_normalize_components_sign(
                                *map(_mul,
                                     _normalize_components_moduli(
                                             self.numerator, other.numerator
                                     ),
                                     _normalize_components_moduli(
                                             other.denominator,
                                             self.denominator
                                     ))
                        ),
                        _normalize=False
                )
                if isinstance(other, Fraction)
                else (self._truediv_by_int(other)
                      if isinstance(other, Int)
                      else NotImplemented)
            )

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

        def _rtruediv_by_int(self, other: Int) -> 'Fraction':
            other, numerator = _normalize_components_moduli(other,
                                                            self.numerator)
            return Fraction(
                    *_normalize_components_sign(other * self.denominator,
                                                numerator),
                    _normalize=False
            )

        def _truediv_by_int(self, other: Int) -> 'Fraction':
            numerator, other = _normalize_components_moduli(self.numerator,
                                                            other)
            return Fraction(
                    *_normalize_components_sign(numerator,
                                                other * self.denominator),
                    _normalize=False
            )


    def _normalize_components_moduli(numerator: Int,
                                     denominator: Int) -> _Tuple[Int, Int]:
        gcd = numerator.gcd(denominator)
        return numerator // gcd, denominator // gcd


    def _normalize_components_sign(numerator: Int,
                                   denominator: Int) -> _Tuple[Int, Int]:
        return ((-numerator, -denominator)
                if denominator < _ZERO
                else (numerator, denominator))
