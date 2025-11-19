from __future__ import annotations

import builtins
import inspect
import pickle
from collections.abc import Callable
from itertools import zip_longest
from platform import python_implementation
from typing import Annotated, Any, Protocol, TypeVar, get_args, get_origin

from typing_extensions import Self


def do_signatures_have_same_api(
    left: inspect.Signature, right: inspect.Signature
) -> bool:
    left_parameters, right_parameters = left.parameters, right.parameters
    return (
        all(
            (
                to_typeless_parameter(left_parameters[parameter_name])
                == to_typeless_parameter(right_parameters[parameter_name])
            )
            for parameter_name in (
                left_parameters.keys() & right_parameters.keys()
            )
        )
        and all(
            is_private_optional_parameter(left_parameters[parameter_name])
            for parameter_name in (
                left_parameters.keys() - right_parameters.keys()
            )
        )
        and all(
            is_private_optional_parameter(right_parameters[parameter_name])
            for parameter_name in (
                right_parameters.keys() - left_parameters.keys()
            )
        )
    )


def is_private_optional_parameter(parameter: inspect.Parameter) -> bool:
    return is_private_object_name(parameter.name) and (
        parameter.kind is inspect.Parameter.KEYWORD_ONLY
    )


def escape_if_built_in_name(name: str, /) -> str:
    return f'{name}_' if name in vars(builtins) else name


def is_class_final(cls: type[Any], /) -> bool:
    return getattr(cls, '__final__', False)


if python_implementation() == 'CPython':

    def is_descriptor(value: Any, /) -> bool:
        return inspect.isgetsetdescriptor(value)

else:

    def is_descriptor(value: Any, /) -> bool:
        return isinstance(value, property)


def is_private_object_name(value: str, /) -> bool:
    return value.startswith('_')


def natural_sorting_key(value: str, /) -> list[int | str]:
    result: list[int | str] = []
    part_index = 0
    while part_index < len(value):
        part_start_index = part_index
        is_integer_part = value[part_index].isdigit()
        while (part_index := part_index + 1) < len(value) and value[
            part_index
        ].isdigit() is is_integer_part:
            pass
        part = value[part_start_index:part_index]
        result.append(int(part) if is_integer_part else part)
    return result


def pickling_round_trip(value: Any, /) -> Any:
    return pickle.loads(pickle.dumps(value))


def to_base_annotation(annotation: Any, /) -> Any | None:
    result = get_origin(annotation)
    if result is Annotated:
        result = get_origin(get_args(annotation)[0])
    return result


def to_class_signature(cls: type[Any], /) -> inspect.Signature | None:
    try:
        return inspect.signature(cls)
    except (TypeError, ValueError):
        return None


def to_lists_different_elements(
    left: list[_T], right: list[_T], /
) -> list[tuple[_T | None, _T | None]]:
    return [
        (left_element, right_element)
        for left_element, right_element in zip_longest(left, right)
        if left_element != right_element
    ]


_T = TypeVar('_T')


def to_self_referential_class_fields(cls: type[_T], /) -> dict[str, _T]:
    return {
        name: value
        for name, value in vars(cls).items()
        if isinstance(value, cls)
    }


class _Sortable(Protocol):
    def __lt__(self, other: Self, /) -> bool: ...


_SortableT = TypeVar('_SortableT', bound=_Sortable)


def to_sorted_lists_diff(
    left: list[_SortableT], right: list[_SortableT], /
) -> list[tuple[_SortableT | None, _SortableT | None]]:
    result: list[tuple[_SortableT | None, _SortableT | None]] = []
    left_elements, right_elements = iter(left), iter(right)
    try:
        left_element = next(left_elements)
    except StopIteration:
        result.extend(
            (None, right_element) for right_element in right_elements
        )
    else:
        while True:
            try:
                while (right_element := next(right_elements)) < left_element:
                    result.append((None, right_element))
            except StopIteration:  # noqa: PERF203
                result.append((left_element, None))
                result.extend(
                    (left_element, None) for left_element in left_elements
                )
                break
            else:
                if right_element == left_element:
                    try:
                        left_element = next(left_elements)
                    except StopIteration:
                        result.extend(
                            (None, right_element)
                            for right_element in right_elements
                        )
                        break
                    else:
                        continue
                result.append((left_element, None))
                assert left_element < right_element, (
                    left_element,
                    right_element,
                )
                try:
                    while (
                        left_element := next(left_elements)
                    ) < right_element:
                        result.append((left_element, None))
                except StopIteration:
                    result.append((None, right_element))
                    result.extend(
                        (None, right_element)
                        for right_element in right_elements
                    )
                    break
                else:
                    if left_element == right_element:
                        try:
                            left_element = next(left_elements)
                        except StopIteration:
                            result.extend(
                                (None, right_element)
                                for right_element in right_elements
                            )
                            break
                        else:
                            continue
                    result.append((None, right_element))
                    assert right_element < left_element, (
                        left_element,
                        right_element,
                    )
    return result


def to_typeless_parameter(parameter: inspect.Parameter) -> inspect.Parameter:
    result = parameter.replace(annotation=inspect.Parameter.empty)
    return (
        result
        if result.default is inspect.Parameter.empty
        else result.replace(default=None)
    )


def to_unsorted_list_elements(
    value: list[_T], /, *, key: Callable[[_T], _SortableT]
) -> list[tuple[_T, _T]]:
    return [
        (element, sorted_element)
        for element, sorted_element in zip(
            value, sorted(value, key=key), strict=True
        )
        if element != sorted_element
    ]
