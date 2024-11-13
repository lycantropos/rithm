from __future__ import annotations

import platform
from typing import Any

from setuptools import find_packages, setup

project_base_url = 'https://github.com/lycantropos/rithm/'
parameters: dict[str, Any] = {
    'packages': find_packages(exclude=('tests', 'tests.*')),
    'url': project_base_url,
    'download_url': project_base_url + 'archive/master.zip',
}
if platform.python_implementation() == 'CPython':
    from setuptools_rust import RustExtension

    parameters.update(
        rust_extensions=[
            RustExtension('rithm._crithm', features=['python_binding'])
        ],
        zip_safe=False,
    )
setup(**parameters)
