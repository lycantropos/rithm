import re
import string
import typing as t
from functools import partial

from hypothesis import strategies

compile_ = t.cast(
    t.Callable[[str], t.Pattern[str]], partial(re.compile, flags=re.ASCII)
)
_whitespaces = r'[\f\n\r\t\v ]'
decimal_int_strings_with_leading_zeros = strategies.from_regex(
    compile_(rf'\A{_whitespaces}*[+-]?\d(\d+)*{_whitespaces}*\Z')
)
decimal_int_strings_without_leading_zeros = strategies.from_regex(
    compile_(rf'\A{_whitespaces}*[+-]?(\d|[1-9](\d+)+){_whitespaces}*\Z')
)
prefixed_binary_int_strings = strategies.from_regex(
    compile_(rf'\A{_whitespaces}*[+-]?0b([0-1]+)+{_whitespaces}*\Z')
)
prefixed_octal_int_strings = strategies.from_regex(
    compile_(rf'\A{_whitespaces}*[+-]?0o([0-7]+)+{_whitespaces}*\Z')
)
prefixed_hexadecimal_int_strings = strategies.from_regex(
    compile_(rf'\A{_whitespaces}*[+-]?0x([\da-f]+)+{_whitespaces}*\Z')
)
int_strings_with_bases = (
    strategies.tuples(
        strategies.from_regex(
            compile_(rf'\A{_whitespaces}*[+-]?0(0+)*{_whitespaces}*\Z')
        ),
        strategies.sampled_from(range(2, 37)),
    )
    | strategies.tuples(
        decimal_int_strings_with_leading_zeros, strategies.just(10)
    )
    | strategies.tuples(
        decimal_int_strings_without_leading_zeros,
        strategies.sampled_from([0, 10]),
    )
    | strategies.one_of(
        [
            strategies.tuples(
                strategies.from_regex(
                    compile_(
                        r'\A{whitespaces}*[+-]?{digits}'
                        r'({digits}+)*{whitespaces}*\Z'.format(
                            digits=f'[0-{max_digit}]', whitespaces=_whitespaces
                        )
                    )
                ),
                strategies.just(max_digit + 1),
            )
            for max_digit in range(1, 10)
        ]
    )
    | strategies.one_of(
        [
            strategies.tuples(
                strategies.from_regex(
                    compile_(
                        r'\A{whitespaces}*[+-]?{digits}({digits}+)*'
                        r'{whitespaces}*\Z'.format(
                            digits=(
                                f'[0-9a-{max_lower}' f'A-{max_lower.upper()}]'
                            ),
                            whitespaces=_whitespaces,
                        )
                    )
                ),
                strategies.just(base),
            )
            for base, max_lower in enumerate(string.ascii_lowercase, start=11)
        ]
    )
    | strategies.tuples(
        prefixed_binary_int_strings, strategies.sampled_from([0, 2])
    )
    | strategies.tuples(
        prefixed_hexadecimal_int_strings, strategies.sampled_from([0, 16])
    )
    | strategies.tuples(
        prefixed_octal_int_strings, strategies.sampled_from([0, 8])
    )
)
