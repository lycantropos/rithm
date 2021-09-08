"""Arbitrary precision arithmetic."""

__version__ = '0.2.0-alpha'

try:
    from ._rithm import (Fraction,
                         Int)
except ImportError:
    from math import gcd as _gcd
    from typing import Optional


    class Int(int):
        def __repr__(self) -> str:
            return f'rithm.Int(\'{self}\')'

        def gcd(self, other: 'Int') -> 'Int':
            return Int(_gcd(self, other))


    class Fraction:
        @property
        def denominator(self) -> Int:
            return self._denominator

        @property
        def numerator(self) -> Int:
            return self._numerator

        __slots__ = '_denominator', '_numerator'

        def __bool__(self) -> bool:
            return bool(self._numerator)

        def __new__(cls,
                    numerator: Int = Int(),
                    denominator: Optional[Int] = Int('1')) -> 'Fraction':
            self = super().__new__(cls)
            if not denominator:
                raise ValueError('Denominator should not be zero.')
            if denominator < 0:
                numerator, denominator = -numerator, -denominator
            gcd = numerator.gcd(denominator)
            self._numerator, self._denominator = (numerator // gcd,
                                                  denominator // gcd)
            return self

        def __repr__(self) -> str:
            return f'rithm.Fraction({self.numerator}, {self.denominator})'
