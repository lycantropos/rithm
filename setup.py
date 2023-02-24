import platform
from pathlib import Path

from setuptools import (find_packages,
                        setup)

import rithm

project_base_url = 'https://github.com/lycantropos/rithm/'


def read_file(path_string: str) -> str:
    return Path(path_string).read_text(encoding='utf-8')


parameters = dict(
        name=rithm.__name__,
        packages=find_packages(exclude=('tests', 'tests.*')),
        version=rithm.__version__,
        description=rithm.__doc__,
        long_description=read_file('README.md'),
        long_description_content_type='text/markdown',
        author='Azat Ibrakov',
        author_email='azatibrakov@gmail.com',
        classifiers=[
            'License :: OSI Approved :: MIT License',
            'Programming Language :: Python :: 3.7',
            'Programming Language :: Python :: 3.8',
            'Programming Language :: Python :: 3.9',
            'Programming Language :: Python :: 3.10',
            'Programming Language :: Python :: 3.11',
            'Programming Language :: Python :: Implementation :: CPython',
            'Programming Language :: Python :: Implementation :: PyPy',
        ],
        license='MIT License',
        url=project_base_url,
        download_url=project_base_url + 'archive/master.zip',
        python_requires='>=3.7',
        install_requires=read_file('requirements.txt'),
        setup_requires=read_file('requirements-setup.txt'),
)
if platform.python_implementation() == 'CPython':
    from typing import (TYPE_CHECKING,
                        Iterator)

    if TYPE_CHECKING:
        from setuptools_rust import RustExtension


    class LazyRustExtensions(list):
        def __iter__(self) -> Iterator['RustExtension']:
            from setuptools_rust import RustExtension
            yield RustExtension(rithm.__name__ + '._c' + rithm.__name__)

        def __len__(self) -> int:
            return 1


    parameters.update(rust_extensions=LazyRustExtensions(),
                      include_package_data=True,
                      zip_safe=False)
setup(**parameters)
