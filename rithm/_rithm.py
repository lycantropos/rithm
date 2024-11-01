from __future__ import annotations

from math import gcd as _gcd
from numbers import Integral, Rational
from operator import mul
from sys import hash_info
from typing import Any, NoReturn, overload

from typing_extensions import Self, final

from .enums import Endianness as _Endianness, TieBreaking as _TieBreaking


@final
@Integral.register
class Int:
    @property
    def denominator(self, /) -> Self:
        return _ONE

    @property
    def numerator(self, /) -> Self:
        return self

    def bit_length(self, /) -> Self:
        return Int(self._value.bit_length())

    def gcd(self, other: Self, /) -> Self:
        return Int(_gcd(self._value, other._value))

    def is_power_of_two(self, /) -> bool:
        return self._value > 0 and not (self._value & (self._value - 1))

    def to_bytes(self, endianness: _Endianness, /) -> bytes:
        return self._value.to_bytes(
            _to_bytes_count(self._value),
            'big' if endianness is _Endianness.BIG else 'little',
            signed=True,
        )

    @classmethod
    def from_bytes(cls, value: bytes, endianness: _Endianness, /) -> Self:
        return cls(
            int.from_bytes(
                value,
                'big' if endianness is _Endianness.BIG else 'little',
                signed=True,
            )
        )

    _value: int

    __module__ = 'rithm.integer'
    __slots__ = ('_value',)

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} ' 'is not an acceptable base type'
        )

    @overload
    def __new__(cls, value: Self | float | int = ..., /) -> Self: ...

    @overload
    def __new__(cls, value: str, base: int | None = ..., /) -> Self: ...

    def __new__(
        cls, value: Self | float | int | str = 0, base: int | None = None, /
    ) -> Self:
        self = super().__new__(cls)
        if base is None:
            self._value = int(value)
        elif isinstance(value, str):
            self._value = int(value, base)
        else:
            raise TypeError((type(value), type(base)))
        return self

    def __abs__(self, /) -> Self:
        return Int(abs(self._value))

    def __add__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value + other._value)
            if isinstance(other, Int)
            else self.__radd__(other)
        )

    def __and__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value & other._value)
            if isinstance(other, Int)
            else self.__rand__(other)
        )

    def __bool__(self, /) -> bool:
        return bool(self._value)

    def __ceil__(self, /) -> Self:
        return self

    def __divmod__(self, other: Self | int, /) -> tuple[Self, Self]:
        if not isinstance(other, (Int, int)):
            return NotImplemented
        quotient, remainder = (
            divmod(self._value, other._value)
            if isinstance(other, Int)
            else divmod(self._value, other)
        )
        return Int(quotient), Int(remainder)

    @overload
    def __eq__(self, other: Self | int, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self._value == other._value
            if isinstance(other, Int)
            else (
                self._value == other
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __float__(self, /) -> float:
        return float(self._value)

    def __floor__(self, /) -> Self:
        return self

    def __floordiv__(self, other: Int | int, /) -> Self:
        return (
            Int(self._value // other._value)
            if isinstance(other, Int)
            else (
                Int(self._value // other)
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __ge__(self, other: Int | int, /) -> bool:
        return (
            self._value >= other._value
            if isinstance(other, Int)
            else (
                self._value >= other
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __reduce__(self, /) -> tuple[type[Self], tuple[int]]:
        return type(self), (self._value,)

    def __gt__(self, other: Self | int, /) -> bool:
        return (
            self._value > other._value
            if isinstance(other, Int)
            else (
                self._value > other
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __hash__(self, /) -> int:
        return hash(self._value)

    def __invert__(self, /) -> Self:
        return Int(~self._value)

    def __index__(self, /) -> int:
        return self._value

    __int__ = __index__

    def __le__(self, other: Self | int, /) -> bool:
        return (
            self._value <= other._value
            if isinstance(other, Int)
            else (
                self._value <= other
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __lshift__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value << other._value)
            if isinstance(other, Int)
            else (
                Int(self._value << other)
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __lt__(self, other: Int | int, /) -> bool:
        return (
            self._value < other._value
            if isinstance(other, Int)
            else (
                self._value < other
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __mod__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value % other._value)
            if isinstance(other, Int)
            else (
                Int(self._value % other)
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __mul__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value * other._value)
            if isinstance(other, Int)
            else self.__rmul__(other)
        )

    def __neg__(self, /) -> Self:
        return Int(-self._value)

    def __or__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value | other._value)
            if isinstance(other, Int)
            else self.__ror__(other)
        )

    def __pos__(self, /) -> Self:
        return self

    @overload
    def __pow__(
        self, exponent: Self | int, divisor: None, /
    ) -> Fraction | Self: ...

    @overload
    def __pow__(
        self, exponent: Self | int, divisor: Self | int = ..., /
    ) -> Self: ...

    def __pow__(
        self, exponent: Self | int, divisor: None | Self | int = None, /
    ) -> Fraction | Self:
        return (
            (
                (
                    Int(self._value ** int(exponent))
                    if exponent >= _ZERO
                    else Fraction(_ONE, self) ** -exponent
                )
                if divisor is None
                else (
                    Int(pow(self._value, int(exponent), int(divisor)))
                    if isinstance(divisor, (Int, int))
                    else NotImplemented
                )
            )
            if isinstance(exponent, (Int, int))
            else NotImplemented
        )

    def __radd__(self, other: int, /) -> Self:
        return (
            Int(self._value + other)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rand__(self, other: int, /) -> Self:
        return (
            Int(self._value & other)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rdivmod__(self, other: int, /) -> tuple[Self, Self]:
        if not isinstance(other, int):
            return NotImplemented
        quotient, remainder = divmod(other, self._value)
        return Int(quotient), Int(remainder)

    def __repr__(self, /) -> str:
        return f'{type(self).__qualname__}({self._value})'

    def __rfloordiv__(self, other: int, /) -> Self:
        return (
            Int(other // self._value)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rlshift__(self, other: int, /) -> Self:
        return (
            Int(other << self._value)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rmod__(self, other: int, /) -> Self:
        return (
            Int(other % self._value)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rmul__(self, other: int, /) -> Self:
        return (
            Int(self._value * other)
            if isinstance(other, int)
            else NotImplemented
        )

    def __ror__(self, other: int, /) -> Self:
        return (
            Int(self._value | other)
            if isinstance(other, int)
            else NotImplemented
        )

    def __round__(self, digits: int | None = None, /) -> Self:
        return Int(round(self._value, digits))

    def __rpow__(
        self, base: int, divisor: None | Self | int = None, /
    ) -> Fraction | Self:
        return (
            Int(base).__pow__(self, divisor)
            if isinstance(base, int)
            else NotImplemented
        )

    def __rrshift__(self, other: int, /) -> Self:
        return (
            Int(other >> self._value)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rshift__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value >> other._value)
            if isinstance(other, Int)
            else (
                Int(self._value >> other)
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __rsub__(self, other: int, /) -> Self:
        return (
            Int(other - self._value)
            if isinstance(other, int)
            else NotImplemented
        )

    def __rtruediv__(self, other: int, /) -> Fraction:
        return (
            Fraction(other, self) if isinstance(other, int) else NotImplemented
        )

    def __rxor__(self, other: int, /) -> Self:
        return (
            Int(self._value ^ other)
            if isinstance(other, int)
            else NotImplemented
        )

    def __str__(self, /) -> str:
        return str(self._value)

    def __sub__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value - other._value)
            if isinstance(other, Int)
            else (
                Int(self._value - other)
                if isinstance(other, int)
                else NotImplemented
            )
        )

    def __truediv__(self, other: Self | int, /) -> Fraction:
        return (
            Fraction(self, other)
            if isinstance(other, (Int, int))
            else NotImplemented
        )

    def __trunc__(self, /) -> Self:
        return self

    def __xor__(self, other: Self | int, /) -> Self:
        return (
            Int(self._value ^ other._value)
            if isinstance(other, Int)
            else self.__rxor__(other)
        )


def _to_bytes_count(value: int, /) -> int:
    return (8 + (value + (value < 0)).bit_length()) // 8


_ONE = Int(1)
_ZERO = Int()
_HASH_INF = Int(hash_info.inf)
_HASH_MODULUS = Int(hash_info.modulus)


@final
@Rational.register
class Fraction:
    @property
    def denominator(self) -> Int:
        return self._denominator

    @property
    def numerator(self) -> Int:
        return self._numerator

    def round(self, tie_breaking: _TieBreaking, /) -> Int:
        quotient, remainder = divmod(self.numerator, self.denominator)
        double_remainder = remainder * 2
        if double_remainder == self.denominator:
            if tie_breaking is _TieBreaking.AWAY_FROM_ZERO:
                return quotient + _ONE if quotient >= 0 else quotient
            elif tie_breaking is _TieBreaking.TO_EVEN:
                return quotient + _ONE if quotient % 2 else quotient
            elif tie_breaking is _TieBreaking.TO_ODD:
                return quotient + _ONE if not quotient % 2 else quotient
            else:
                assert tie_breaking is _TieBreaking.TOWARD_ZERO, tie_breaking
                return quotient + _ONE if quotient < 0 else quotient
        else:
            return (
                quotient + _ONE
                if double_remainder > self.denominator
                else quotient
            )

    _denominator: Int
    _numerator: Int

    __module__ = 'rithm.fraction'
    __slots__ = '_denominator', '_numerator'

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} ' 'is not an acceptable base type'
        )

    def __new__(
        cls,
        numerator: Int | int | float = _ZERO,
        denominator: Int | None | int = None,
        /,
        *,
        _normalize: bool = True,
    ) -> Self:
        self = super().__new__(cls)
        if denominator is None:
            if isinstance(numerator, Fraction):
                numerator, denominator = (
                    numerator.numerator,
                    numerator.denominator,
                )
            elif isinstance(numerator, float):
                raw_numerator, raw_denominator = numerator.as_integer_ratio()
                numerator, denominator = (
                    Int(raw_numerator),
                    Int(raw_denominator),
                )
            elif isinstance(numerator, (Int, int)):
                numerator, denominator = Int(numerator), _ONE
            elif isinstance(numerator, Rational):
                numerator, denominator = (
                    Int(numerator.numerator),
                    Int(numerator.denominator),
                )
            else:
                raise TypeError(
                    'First argument should be of '
                    f'type {Int}, {int} or {float}, '
                    f'but found: {type(numerator)}.'
                )
        elif isinstance(denominator, (Int, int)):
            if not isinstance(numerator, (Int, int)):
                raise TypeError(
                    f'Numerator should be '
                    f'of type {Int} or {int}, '
                    f'but found: {type(numerator)}.'
                )
            numerator, denominator = Int(numerator), Int(denominator)
        else:
            raise TypeError(
                f'Denominator should be '
                f'of type {Int} or {int}, '
                f'but found: {type(denominator)}.'
            )
        if not denominator:
            raise ZeroDivisionError('Denominator should not be zero.')
        if _normalize:
            numerator, denominator = _normalize_components_sign(
                *_normalize_components_moduli(numerator, denominator)
            )
        self._numerator, self._denominator = numerator, denominator
        return self

    def __abs__(self, /) -> Self:
        return Fraction(
            abs(self.numerator), self.denominator, _normalize=False
        )

    def __add__(self, other: Self | Int | int, /) -> Self:
        return (
            self._add_fraction(other)
            if isinstance(other, Fraction)
            else self.__radd__(other)
        )

    def __bool__(self, /) -> bool:
        return bool(self.numerator)

    def __ceil__(self, /) -> Int:
        return -(-self.numerator // self.denominator)

    def __divmod__(self, divisor: Self | Int | int, /) -> tuple[Int, Self]:
        return (
            _divmod_rationals(self, divisor)
            if isinstance(divisor, (Fraction, Int, int))
            else NotImplemented
        )

    @overload
    def __eq__(self, other: Int | Self | int, /) -> bool: ...

    @overload
    def __eq__(self, other: Any, /) -> Any: ...

    def __eq__(self, other: Any, /) -> Any:
        return (
            self.numerator == other.numerator
            and self.denominator == other.denominator
            if isinstance(other, Fraction)
            else (
                self.denominator == _ONE and self.numerator == other
                if isinstance(other, (Int, int))
                else NotImplemented
            )
        )

    def __float__(self, /) -> float:
        return int(self.numerator) / int(self.denominator)

    def __floor__(self, /) -> Int:
        return self.numerator // self.denominator

    def __floordiv__(self, divisor: Int | Self | int, /) -> Int:
        return (
            (self.numerator * divisor.denominator)
            // (self.denominator * divisor.numerator)
            if isinstance(divisor, Fraction)
            else (
                self.numerator // (self.denominator * divisor)
                if isinstance(divisor, (Int, int))
                else NotImplemented
            )
        )

    def __ge__(self, other: Int | Self | int, /) -> bool:
        return (
            self.numerator * other.denominator
            >= other.numerator * self.denominator
            if isinstance(other, Fraction)
            else (
                self.numerator >= other * self.denominator
                if isinstance(other, (Int, int))
                else NotImplemented
            )
        )

    def __gt__(self, other: Self | Int | int, /) -> bool:
        return (
            self.numerator * other.denominator
            > other.numerator * self.denominator
            if isinstance(other, Fraction)
            else (
                self.numerator > other * self.denominator
                if isinstance(other, (Int, int))
                else NotImplemented
            )
        )

    def __hash__(self, /) -> int:
        inverted_denominator = self._denominator.__pow__(
            _HASH_MODULUS - 2, _HASH_MODULUS
        )
        result = (
            ((abs(self._numerator) * inverted_denominator) % _HASH_MODULUS)
            if inverted_denominator
            else _HASH_INF
        )
        result = result if self >= 0 else -result
        return -2 if result == -1 else int(result)

    def __le__(self, other: Int | Self | int, /) -> bool:
        return (
            self.numerator * other.denominator
            <= other.numerator * self.denominator
            if isinstance(other, Fraction)
            else (
                self.numerator <= other * self.denominator
                if isinstance(other, (Int, int))
                else NotImplemented
            )
        )

    def __lt__(self, other: Int | Self | int, /) -> bool:
        return (
            self.numerator * other.denominator
            < other.numerator * self.denominator
            if isinstance(other, Fraction)
            else (
                self.numerator < other * self.denominator
                if isinstance(other, (Int, int))
                else NotImplemented
            )
        )

    def __mod__(self, divisor: Int | Self | int, /) -> Self:
        return (
            Fraction(
                (self.numerator * divisor.denominator)
                % (self.denominator * divisor.numerator),
                self.denominator * divisor.denominator,
            )
            if isinstance(divisor, Fraction)
            else (
                Fraction(
                    self.numerator % (self.denominator * divisor),
                    self.denominator,
                )
                if isinstance(divisor, (Int, int))
                else NotImplemented
            )
        )

    def __mul__(self, other: Int | Self | int, /) -> Self:
        return (
            self._mul_by_fraction(other)
            if isinstance(other, Fraction)
            else self.__rmul__(other)
        )

    def __neg__(self, /) -> Self:
        return Fraction(-self.numerator, self.denominator, _normalize=False)

    def __pos__(self, /) -> Self:
        return self

    def __pow__(self, exponent: Int | int, divisor: None = None, /) -> Self:
        return (
            (
                Fraction(
                    self.numerator**exponent,
                    self.denominator**exponent,
                    _normalize=False,
                )
                if exponent >= _ZERO
                else Fraction(
                    *_normalize_components_sign(
                        self.denominator**-exponent, self.numerator**-exponent
                    ),
                    _normalize=False,
                )
            )
            if isinstance(exponent, (Int, int)) and divisor is None
            else NotImplemented
        )

    def __radd__(self, other: Int | int, /) -> Self:
        return (
            self._add_int(Int(other))
            if isinstance(other, (Int, int))
            else NotImplemented
        )

    def __rdivmod__(self, dividend: Int | int, /) -> tuple[Int, Self]:
        return (
            _divmod_rationals(dividend, self)
            if isinstance(dividend, (Int, int))
            else NotImplemented
        )

    def __reduce__(self, /) -> tuple[type[Self], tuple[Int, Int]]:
        return type(self), (self._numerator, self._denominator)

    def __repr__(self, /) -> str:
        return (
            f'{type(self).__qualname__}'
            f'({self.numerator!r}, {self.denominator!r})'
        )

    def __rfloordiv__(self, dividend: Int | int, /) -> Int:
        return (
            (dividend * self.denominator) // self.numerator
            if isinstance(dividend, (Int, int))
            else NotImplemented
        )

    def __rmod__(self, dividend: Int | int, /) -> Self:
        return (
            Fraction(
                (dividend * self.denominator) % self.numerator,
                self.denominator,
            )
            if isinstance(dividend, (Int, int))
            else NotImplemented
        )

    def __rmul__(self, other: Int | int, /) -> Self:
        return (
            self._mul_by_int(Int(other))
            if isinstance(other, (Int, int))
            else NotImplemented
        )

    @overload
    def __round__(self, digits: None = ..., /) -> Int: ...

    @overload
    def __round__(self, digits: int, /) -> Self: ...

    def __round__(self, digits: int | None = None, /) -> Int | Self:
        if digits is None:
            return self.round(_TieBreaking.TO_EVEN)
        else:
            shift = 10 ** abs(digits)
            return (
                Fraction((self * shift).round(_TieBreaking.TO_EVEN), shift)
                if digits > 0
                else Fraction(
                    (self / shift).round(_TieBreaking.TO_EVEN) * shift
                )
            )

    def __rsub__(self, subtrahend: Int | int, /) -> Self:
        return (
            Fraction(
                *_normalize_components_moduli(
                    subtrahend * self.denominator - self.numerator,
                    self.denominator,
                ),
                _normalize=False,
            )
            if isinstance(subtrahend, (Int, int))
            else NotImplemented
        )

    def __str__(self, /) -> str:
        return (
            str(self.numerator)
            if self.denominator == _ONE
            else f'{self.numerator}/{self.denominator}'
        )

    def __sub__(self, minuend: Int | Self | int, /) -> Self:
        return (
            Fraction(
                *_normalize_components_moduli(
                    self.numerator * minuend.denominator
                    - minuend.numerator * self.denominator,
                    self.denominator * minuend.denominator,
                ),
                _normalize=False,
            )
            if isinstance(minuend, Fraction)
            else (
                Fraction(
                    *_normalize_components_moduli(
                        self.numerator - minuend * self.denominator,
                        self.denominator,
                    ),
                    _normalize=False,
                )
                if isinstance(minuend, (Int, int))
                else NotImplemented
            )
        )

    def __rtruediv__(self, dividend: Int | int, /) -> Self:
        return (
            self._rtruediv_by_int(Int(dividend))
            if isinstance(dividend, (Int, int))
            else NotImplemented
        )

    def __truediv__(self, divisor: Int | Self | int, /) -> Self:
        return (
            Fraction(
                *_normalize_components_sign(
                    *map(
                        mul,
                        _normalize_components_moduli(
                            self.numerator, divisor.numerator
                        ),
                        _normalize_components_moduli(
                            divisor.denominator, self.denominator
                        ),
                    )
                ),
                _normalize=False,
            )
            if isinstance(divisor, Fraction)
            else (
                self._truediv_by_int(Int(divisor))
                if isinstance(divisor, (Int, int))
                else NotImplemented
            )
        )

    def __trunc__(self, /) -> Int:
        return self.__ceil__() if self.numerator < _ZERO else self.__floor__()

    def _add_fraction(self, other: Self, /) -> Self:
        return Fraction(
            *_normalize_components_moduli(
                self.numerator * other.denominator
                + other.numerator * self.denominator,
                self.denominator * other.denominator,
            ),
            _normalize=False,
        )

    def _add_int(self, other: Int, /) -> Self:
        return Fraction(
            *_normalize_components_moduli(
                self.numerator + other * self.denominator, self.denominator
            ),
            _normalize=False,
        )

    def _mul_by_fraction(self, other: Self, /) -> Self:
        numerator, other_denominator = _normalize_components_moduli(
            self.numerator, other.denominator
        )
        other_numerator, denominator = _normalize_components_moduli(
            other.numerator, self.denominator
        )
        return Fraction(
            numerator * other_numerator,
            denominator * other_denominator,
            _normalize=False,
        )

    def _mul_by_int(self, other: Int, /) -> Self:
        other, denominator = _normalize_components_moduli(
            other, self.denominator
        )
        return Fraction(self.numerator * other, denominator, _normalize=False)

    def _rtruediv_by_int(self, dividend: Int, /) -> Self:
        dividend, numerator = _normalize_components_moduli(
            dividend, self.numerator
        )
        return Fraction(
            *_normalize_components_sign(
                dividend * self.denominator, numerator
            ),
            _normalize=False,
        )

    def _truediv_by_int(self, divisor: Int, /) -> Self:
        numerator, divisor = _normalize_components_moduli(
            self.numerator, divisor
        )
        return Fraction(
            *_normalize_components_sign(numerator, divisor * self.denominator),
            _normalize=False,
        )


def _divmod_rationals(
    dividend: Fraction | Int | int, divisor: Fraction | Int | int, /
) -> tuple[Int, Fraction]:
    quotient, remainder_numerator = divmod(
        dividend.numerator * divisor.denominator,
        dividend.denominator * divisor.numerator,
    )
    assert isinstance(quotient, Int)
    return quotient, Fraction(
        remainder_numerator, dividend.denominator * divisor.denominator
    )


def _normalize_components_moduli(
    numerator: Int, denominator: Int, /
) -> tuple[Int, Int]:
    gcd = numerator.gcd(denominator)
    return numerator // gcd, denominator // gcd


def _normalize_components_sign(
    numerator: Int, denominator: Int, /
) -> tuple[Int, Int]:
    return (
        (-numerator, -denominator)
        if denominator < _ZERO
        else (numerator, denominator)
    )
