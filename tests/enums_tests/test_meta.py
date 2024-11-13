import importlib
import inspect
import sys
import typing
from types import ModuleType
from typing import Any, ClassVar, get_args, get_type_hints

import pytest
from _pytest.monkeypatch import MonkeyPatch
from typing_extensions import Self

from tests.meta_utils import (
    do_signatures_have_same_api,
    escape_if_built_in_name,
    is_class_final,
    is_descriptor,
    is_private_object_name,
    natural_sorting_key,
    to_base_annotation,
    to_class_signature,
    to_lists_different_elements,
    to_self_referential_class_fields,
    to_sorted_lists_diff,
    to_unsorted_list_elements,
)


@pytest.fixture(scope='module')
def module() -> ModuleType:
    from rithm import enums

    return enums


@pytest.fixture(scope='module')
def module_namespace(module: ModuleType) -> dict[str, Any]:
    return vars(module)


@pytest.fixture
def type_annotated_module(
    module: ModuleType, monkeypatch: MonkeyPatch
) -> ModuleType:
    module_name = module.__name__
    rest_modules = sys.modules.copy()
    del rest_modules[module_name]
    with monkeypatch.context() as patch:
        patch.setattr(sys, 'modules', rest_modules)
        patch.setattr(typing, 'TYPE_CHECKING', True)
        return importlib.import_module(module_name)


@pytest.fixture
def type_annotated_module_namespace(
    type_annotated_module: ModuleType,
) -> dict[str, Any]:
    return vars(type_annotated_module)


def test_type_annotations(
    module_namespace: dict[str, Any],
    type_annotated_module_namespace: dict[str, Any],
) -> None:
    assert type_annotated_module_namespace.keys() == module_namespace.keys()
    assert [
        name
        for name in (
            type_annotated_module_namespace.keys() & module_namespace.keys()
        )
        if (
            type(type_annotated_module_namespace[name])
            is not type(module_namespace[name])
        )
        and (
            inspect.isfunction(type_annotated_module_namespace[name])
            is not inspect.isbuiltin(module_namespace[name])
        )
    ] == []


def test_member_module_names(
    module: ModuleType, module_namespace: dict[str, Any]
) -> None:
    assert {
        name: value
        for name, value in module_namespace.items()
        if (
            not is_private_object_name(name)
            and (
                inspect.isfunction(value)
                or inspect.isbuiltin(value)
                or inspect.isclass(value)
            )
            and getattr(value, '__module__', None) != module.__name__
        )
    } == {}


def test_enum_like_classes(
    module_namespace: dict[str, Any],
    type_annotated_module_namespace: dict[str, Any],
) -> None:
    classes = {
        name: value
        for name, value in module_namespace.items()
        if (
            inspect.isclass(value)
            and value.__module__ == module_namespace['__name__']
        )
    }
    type_annotated_classes = {
        name: value
        for name, value in type_annotated_module_namespace.items()
        if (
            inspect.isclass(value)
            and value.__module__ == type_annotated_module_namespace['__name__']
        )
    }
    enum_like_classes_field_names = {
        cls: field_names
        for cls in classes.values()
        if (
            is_class_final(cls)
            and (
                len(field_names := list(to_self_referential_class_fields(cls)))
                != 0
            )
        )
    }
    type_annotated_enum_like_classes_field_names = {
        cls: field_names
        for cls in type_annotated_classes.values()
        if (
            is_class_final(cls)
            and len(
                field_names := [
                    field_name
                    for field_name, field_annotation in get_type_hints(
                        cls, type_annotated_module_namespace
                    ).items()
                    if (
                        to_base_annotation(field_annotation) is ClassVar
                        and get_args(field_annotation)[0] is Self
                    )
                ]
            )
            != 0
        )
    }

    assert {cls.__qualname__ for cls in enum_like_classes_field_names} == {
        cls.__qualname__
        for cls in type_annotated_enum_like_classes_field_names
    }
    assert {
        cls: unsorted_field_names
        for cls, field_names in enum_like_classes_field_names.items()
        if (
            len(
                unsorted_field_names := to_unsorted_list_elements(
                    field_names, key=natural_sorting_key
                )
            )
            != 0
        )
    } == {}
    assert {
        cls: unsorted_field_names
        for cls, field_names in (
            type_annotated_enum_like_classes_field_names.items()
        )
        if (
            len(
                unsorted_field_names := to_unsorted_list_elements(
                    field_names, key=natural_sorting_key
                )
            )
            != 0
        )
    } == {}
    assert {
        cls: to_sorted_lists_diff(field_names, type_annotated_field_names)
        for cls, field_names in enum_like_classes_field_names.items()
        if (
            field_names
            != (
                type_annotated_field_names := (
                    type_annotated_enum_like_classes_field_names[
                        type_annotated_classes[cls.__qualname__]
                    ]
                )
            )
        )
    } == {}


def test_constructible_classes(
    module_namespace: dict[str, Any],
    type_annotated_module_namespace: dict[str, Any],
) -> None:
    classes = {
        name: value
        for name, value in module_namespace.items()
        if (
            inspect.isclass(value)
            and value.__module__ == module_namespace['__name__']
        )
    }
    type_annotated_classes = {
        name: value
        for name, value in type_annotated_module_namespace.items()
        if (
            inspect.isclass(value)
            and value.__module__ == type_annotated_module_namespace['__name__']
        )
    }
    constructible_class_signatures = {
        cls: signature
        for cls in classes.values()
        if (signature := to_class_signature(cls)) is not None
    }
    type_annotated_class_signatures = {
        cls: signature
        for cls in type_annotated_classes.values()
        if (signature := to_class_signature(cls)) is not None
    }

    assert constructible_class_signatures
    assert classes.keys() == type_annotated_classes.keys()
    assert {cls.__qualname__ for cls in constructible_class_signatures} - {
        cls.__qualname__ for cls in type_annotated_class_signatures
    } == set()
    assert [
        class_name
        for class_name in (
            {cls.__qualname__ for cls in type_annotated_class_signatures}
            - {cls.__qualname__ for cls in constructible_class_signatures}
        )
        if (
            not is_class_final(module_namespace[class_name])
            and not inspect.isabstract(
                type_annotated_module_namespace[class_name]
            )
        )
    ] == []
    assert [
        cls
        for cls in constructible_class_signatures
        if (
            is_class_final(cls)
            is not is_class_final(
                type_annotated_module_namespace[cls.__qualname__]
            )
        )
    ] == []
    assert {
        cls: to_sorted_lists_diff(
            sorted_descriptor_names, sorted_parameter_names
        )
        for cls, signature in constructible_class_signatures.items()
        if (
            (
                sorted_parameter_names := sorted(
                    parameter_name
                    for parameter_name in signature.parameters
                    if not is_private_object_name(parameter_name)
                )
            )
            != (
                sorted_descriptor_names := sorted(
                    escape_if_built_in_name(field_name)
                    for field_name, field in vars(cls).items()
                    if is_descriptor(field)
                )
            )
        )
    } == {}
    assert {
        cls: (
            type_annotated_cls
            if type_annotated_cls is None
            else to_lists_different_elements(parameter_names, descriptors)
        )
        for cls, signature in constructible_class_signatures.items()
        if (
            (
                (
                    type_annotated_cls := type_annotated_module_namespace.get(
                        cls.__qualname__
                    )
                )
                is None
            )
            or (
                (
                    parameter_names := [
                        parameter_name
                        for parameter_name in signature.parameters
                        if not is_private_object_name(parameter_name)
                    ]
                )
                != (
                    descriptors := [
                        escape_if_built_in_name(field_name)
                        for field_name, field in vars(
                            type_annotated_cls
                        ).items()
                        if isinstance(field, property)
                    ]
                )
            )
        )
    } == {}
    assert [
        cls
        for cls, signature in constructible_class_signatures.items()
        if (
            (
                type_annotated_cls := type_annotated_module_namespace.get(
                    cls.__qualname__
                )
            )
            is None
            or (
                (
                    type_annotated_signature := to_class_signature(
                        type_annotated_cls
                    )
                )
                is None
            )
            or not do_signatures_have_same_api(
                signature, type_annotated_signature
            )
        )
    ] == []
