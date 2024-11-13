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
    is_class_final,
    is_private_object_name,
    to_base_annotation,
    to_class_signature,
    to_self_referential_class_fields,
)


@pytest.fixture(scope='module')
def module() -> ModuleType:
    from rithm import integer

    return integer


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
    enum_like_classes = [
        cls
        for cls in classes.values()
        if (
            is_class_final(cls)
            and len(list(to_self_referential_class_fields(cls))) != 0
        )
    ]
    type_annotated_enum_like_classes = [
        cls
        for cls in type_annotated_classes.values()
        if (
            is_class_final(cls)
            and len(
                [
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
    ]

    assert enum_like_classes == []
    assert type_annotated_enum_like_classes == []


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
