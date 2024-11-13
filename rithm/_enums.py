from __future__ import annotations

from typing import Any, ClassVar, NoReturn, get_type_hints

from typing_extensions import Self, final


@final
class Endianness:
    BIG: ClassVar[Self]
    LITTLE: ClassVar[Self]

    @property
    def value(self) -> int:
        return self._value

    _cache: ClassVar[dict[int, Self]] = {}
    __module__ = 'rithm.enums'
    __slots__ = ('_value',)
    _value: int

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, value: int, /) -> Self:
        if not (
            isinstance(value, int)
            and 0 <= value < len(get_type_hints(cls, globals()))
        ):
            raise ValueError(f'{value} is not a valid {cls.__qualname__}')
        try:
            return cls._cache[value]
        except KeyError:
            self = super().__new__(cls)
            self._value = value
            cls._cache[value] = self
            return self

    def __getnewargs__(self) -> tuple[int]:
        return (self._value,)

    def __repr__(self) -> str:
        name = next(
            name for name, field in vars(type(self)).items() if field is self
        )
        return f'{type(self).__qualname__}.{name}'


Endianness.BIG = Endianness(0)
Endianness.LITTLE = Endianness(1)


@final
class TieBreaking:
    AWAY_FROM_ZERO: ClassVar[Self]
    TOWARD_ZERO: ClassVar[Self]
    TO_EVEN: ClassVar[Self]
    TO_ODD: ClassVar[Self]

    @property
    def value(self) -> int:
        return self._value

    _cache: ClassVar[dict[int, Self]] = {}
    __module__ = 'rithm.enums'
    __slots__ = ('_value',)
    _value: int

    def __init_subclass__(cls, /, **_kwargs: Any) -> NoReturn:
        raise TypeError(
            f'type {cls.__qualname__!r} is not an acceptable base type'
        )

    def __new__(cls, value: int, /) -> Self:
        if not (
            isinstance(value, int)
            and 0 <= value < len(get_type_hints(cls, globals()))
        ):
            raise ValueError(f'{value} is not a valid {cls.__qualname__}')
        try:
            return cls._cache[value]
        except KeyError:
            self = super().__new__(cls)
            self._value = value
            cls._cache[value] = self
            return self

    def __getnewargs__(self) -> tuple[int]:
        return (self._value,)

    def __repr__(self) -> str:
        name = next(
            name for name, field in vars(type(self)).items() if field is self
        )
        return f'{type(self).__qualname__}.{name}'


TieBreaking.AWAY_FROM_ZERO = TieBreaking(0)
TieBreaking.TOWARD_ZERO = TieBreaking(1)
TieBreaking.TO_EVEN = TieBreaking(2)
TieBreaking.TO_ODD = TieBreaking(3)
