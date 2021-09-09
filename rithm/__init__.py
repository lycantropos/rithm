"""Arbitrary precision arithmetic."""

__version__ = '0.2.0-alpha'

try:
    from ._rithm import (Fraction,
                         Int)
except ImportError:
    from math import gcd as _gcd
    from typing import (Optional as _Optional,
                        Tuple as _Tuple,
                        Union as _Union)


    class Int:
        def gcd(self, other: 'Int') -> 'Int':
            return Int(_gcd(self._value, other._value),
                       _parse=False)

        __slots__ = '_value',

        def __new__(cls, _value: _Union[str, int] = '0', _base: int = 10,
                    *,
                    _parse: bool = True) -> 'Int':
            self = super().__new__(cls)
            if _parse and not isinstance(_value, str):
                raise TypeError(f'First argument should be of type {str}, '
                                f'but found {type(_value)}.')
            self._value = int(_value, _base) if _parse else _value
            return self

        def __abs__(self) -> 'Int':
            return Int(abs(self._value),
                       _parse=False)

        def __add__(self, other: 'Int') -> 'Int':
            return (Int(self._value + other._value,
                        _parse=False)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __bool__(self) -> bool:
            return bool(self._value)

        def __floordiv__(self, other: 'Int') -> 'Int':
            return (Int(self._value // other._value,
                        _parse=False)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __divmod__(self, other: 'Int') -> _Tuple['Int', 'Int']:
            return (tuple(Int(value,
                              _parse=False)
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

        def __gt__(self, other: 'Int') -> bool:
            return (self._value > other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __hash__(self) -> int:
            return hash(self._value)

        def __le__(self, other: 'Int') -> bool:
            return (self._value <= other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __lt__(self, other: 'Int') -> bool:
            return (self._value < other._value
                    if isinstance(other, Int)
                    else NotImplemented)

        def __mod__(self, other: 'Int') -> 'Int':
            return (Int(self._value % other._value,
                        _parse=False)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __mul__(self, other: 'Int') -> 'Int':
            return (Int(self._value * other._value,
                        _parse=False)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __neg__(self) -> 'Int':
            return Int(-self._value,
                       _parse=False)

        def __repr__(self) -> str:
            return f'rithm.Int(\'{self._value}\')'

        def __str__(self) -> str:
            return str(self._value)

        def __sub__(self, other: 'Int') -> 'Int':
            return (Int(self._value - other._value,
                        _parse=False)
                    if isinstance(other, Int)
                    else NotImplemented)

        def __truediv__(self, other: 'Int') -> 'Fraction':
            return (Fraction(self, other)
                    if isinstance(other, Int)
                    else NotImplemented)


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
                    denominator: Int = _ONE) -> 'Fraction':
            self = super().__new__(cls)
            if not isinstance(numerator, Int):
                raise TypeError(f'First argument should be of type {Int}, '
                                f'but found: {type(denominator)}.')
            if not isinstance(denominator, Int):
                raise TypeError(f'Denominator should be of type {Int}, '
                                f'but found: {type(denominator)}.')
            if not denominator:
                raise ZeroDivisionError('Denominator should not be zero.')
            if denominator < _ZERO:
                numerator, denominator = -numerator, -denominator
            gcd = numerator.gcd(denominator)
            self._numerator, self._denominator = (numerator // gcd,
                                                  denominator // gcd)
            return self

        def __bool__(self) -> bool:
            return bool(self._numerator)

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

        def __repr__(self) -> str:
            return f'rithm.Fraction({self.numerator!r}, {self.denominator!r})'

        def __str__(self) -> str:
            return (str(self.numerator)
                    if self.denominator == _ONE
                    else f'{self.numerator}/{self.denominator})')
