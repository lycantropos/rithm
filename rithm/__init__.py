"""Arbitrary precision arithmetic."""

__version__ = '2.0.0'

try:
    from ._rithm import (Fraction,
                         Int)
except ImportError:
    from math import gcd as _gcd
    from operator import mul as _mul
    from typing import (Optional as _Optional,
                        Tuple as _Tuple,
                        Union as _Union)


    class Int:
        def gcd(self, other: 'Int') -> 'Int':
            return Int(_gcd(self._value, other._value))

        __slots__ = '_value',

        def __new__(cls,
                    _value: _Union[str, int] = 0,
                    _base: _Optional[int] = None) -> 'Int':
            self = super().__new__(cls)
            self._value = (int(_value, _base)
                           if _base is not None
                           else int(_value))
            return self

        def __abs__(self) -> 'Int':
            return Int(abs(self._value))

        def __add__(self, other: 'Int') -> 'Int':
            return (Int(self._value + other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __bool__(self) -> bool:
            return bool(self._value)

        def __ceil__(self) -> int:
            return self._value

        def __floor__(self) -> int:
            return self._value

        def __floordiv__(self, other: 'Int') -> 'Int':
            return (Int(self._value // other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __divmod__(self, other: 'Int') -> _Tuple['Int', 'Int']:
            return (tuple(Int(value)
                          for value in divmod(self._value, other._value))
                    if isinstance(other, Int)
                    else NotImplemented)

        def __eq__(self, other: 'Int') -> bool:
            return (self._value == other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __ge__(self, other: 'Int') -> bool:
            return (self._value >= other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __getstate__(self) -> int:
            return self._value

        def __gt__(self, other: 'Int') -> bool:
            return (self._value > other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __hash__(self) -> int:
            return hash(self._value)

        def __invert__(self) -> 'Int':
            return Int(~self._value)

        def __le__(self, other: 'Int') -> bool:
            return (self._value <= other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __lt__(self, other: 'Int') -> bool:
            return (self._value < other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __mod__(self, other: 'Int') -> 'Int':
            return (Int(self._value % other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __mul__(self, other: 'Int') -> 'Int':
            return (Int(self._value * other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __neg__(self) -> 'Int':
            return Int(-self._value)

        def __pow__(self, exponent: 'Int') -> _Union['Fraction', 'Int']:
            return ((Int(self._value ** exponent._value)
                     if exponent >= _ZERO
                     else Fraction(_ONE, self) ** -exponent)
                    if isinstance(exponent, Int)
                    else NotImplemented)

        def __repr__(self) -> str:
            return f'rithm.Int(\'{self._value}\')'

        def __setstate__(self, state: int) -> None:
            self._value = state

        def __str__(self) -> str:
            return str(self._value)

        def __sub__(self, other: 'Int') -> 'Int':
            return (Int(self._value - other._value)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __truediv__(self, other: 'Int') -> 'Fraction':
            return (Fraction(self, other)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __trunc__(self) -> int:
            return self._value


    _ONE = Int('1')
    _ZERO = Int()


    class Fraction:
        @property
        def denominator(self) -> Int:
            return self._denominator

        @property
        def numerator(self) -> Int:
            return self._numerator

        __slots__ = '_denominator', '_numerator'

        def __new__(cls,
                    numerator: Int = _ZERO,
                    denominator: Int = _ONE,
                    *,
                    _normalize: bool = True) -> 'Fraction':
            self = super().__new__(cls)
            if not isinstance(numerator, Int):
                raise TypeError(f'First argument should be of type {Int}, '
                                f'but found: {type(denominator)}.')
            if not isinstance(denominator, Int):
                raise TypeError(f'Denominator should be of type {Int}, '
                                f'but found: {type(denominator)}.')
            if not denominator:
                raise ZeroDivisionError('Denominator should not be zero.')
            if _normalize:
                numerator, denominator = _normalize_components_sign(
                    *_normalize_components_moduli(numerator, denominator))
            self._numerator, self._denominator = numerator, denominator
            return self

        def __abs__(self) -> 'Fraction':
            return Fraction(abs(self.numerator), self.denominator,
                            _normalize=False)

        def __add__(self, other: 'Fraction') -> 'Fraction':
            return (
                Fraction(
                    *_normalize_components_moduli(
                        self.numerator * other.denominator
                        + other.numerator * self.denominator,
                        self.denominator * other.denominator),
                    _normalize=False)
                if isinstance(other, Fraction)
                else NotImplemented)

        def __bool__(self) -> bool:
            return bool(self.numerator)

        def __eq__(self, other: 'Fraction') -> bool:
            return (self.numerator == other.numerator
                    and self.denominator == other.denominator
                    if isinstance(other, Fraction)
                    else NotImplemented)

        def __ge__(self, other: 'Fraction') -> bool:
            return (self.numerator * other.denominator
                    >= other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else NotImplemented)

        def __getstate__(self) -> _Tuple[Int, Int]:
            return self._numerator, self._denominator

        def __gt__(self, other: 'Fraction') -> bool:
            return (self.numerator * other.denominator
                    > other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else NotImplemented)

        def __le__(self, other: 'Fraction') -> bool:
            return (self.numerator * other.denominator
                    <= other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else NotImplemented)

        def __lt__(self, other: 'Fraction') -> bool:
            return (self.numerator * other.denominator
                    < other.numerator * self.denominator
                    if isinstance(other, Fraction)
                    else NotImplemented)

        def __mul__(self, other: 'Fraction') -> 'Fraction':
            return (self._mul_by_fraction(other)
                    if isinstance(other, Fraction)
                    else (self._mul_by_int(other)
                          if isinstance(other, Int)
                          else NotImplemented))

        def __neg__(self) -> 'Fraction':
            return Fraction(-self.numerator, self.denominator,
                            _normalize=False)

        def __pow__(self, exponent: 'Int') -> 'Fraction':
            return (
                (Fraction(self.numerator ** exponent,
                          self.denominator ** exponent,
                          _normalize=False)
                 if exponent >= _ZERO
                 else Fraction(
                    *_normalize_components_sign(self.denominator ** -exponent,
                                                self.numerator ** -exponent),
                    _normalize=False))
                if isinstance(exponent, Int)
                else NotImplemented)

        def __repr__(self) -> str:
            return f'rithm.Fraction({self.numerator!r}, {self.denominator!r})'

        def __rmul__(self, other: Int) -> 'Fraction':
            return (self._mul_by_int(other)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __setstate__(self, state: _Tuple[Int, Int]) -> None:
            self._numerator, self._denominator = state

        def __str__(self) -> str:
            return (str(self.numerator)
                    if self.denominator == _ONE
                    else f'{self.numerator}/{self.denominator}')

        def __sub__(self, other: 'Fraction') -> 'Fraction':
            return (
                Fraction(
                    *_normalize_components_moduli(
                        self.numerator * other.denominator
                        - other.numerator * self.denominator,
                        self.denominator * other.denominator),
                    _normalize=False)
                if isinstance(other, Fraction)
                else NotImplemented)

        def __truediv__(self, other: 'Fraction') -> 'Fraction':
            return (
                Fraction(*_normalize_components_sign(
                    *map(_mul,
                         _normalize_components_moduli(self.numerator,
                                                      other.numerator),
                         _normalize_components_moduli(other.denominator,
                                                      self.denominator))),
                         _normalize=False)
                if isinstance(other, Fraction)
                else NotImplemented)

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


    def _normalize_components_moduli(numerator: Int,
                                     denominator: Int) -> _Tuple[Int, Int]:
        gcd = numerator.gcd(denominator)
        return numerator // gcd, denominator // gcd


    def _normalize_components_sign(numerator: Int,
                                   denominator: Int) -> _Tuple[Int, Int]:
        return ((-numerator, -denominator)
                if denominator < _ZERO
                else (numerator, denominator))
