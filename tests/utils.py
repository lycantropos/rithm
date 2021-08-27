from rithm import Int


def is_equivalent_to_builtin_int(value: Int, builtin: int) -> bool:
    return str(value) == str(builtin)
