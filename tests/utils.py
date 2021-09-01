from typing import Tuple

from rithm import Int


def equivalence(left: bool, right: bool) -> bool:
    return left is right


def is_equivalent_to_builtin_int(value: Int, builtin: int) -> bool:
    return str(value) == str(builtin)


def to_int_with_builtin_int(decimal_string: str) -> Tuple[Int, int]:
    return Int(decimal_string), int(decimal_string)
