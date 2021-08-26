from pathlib import Path
from typing import (TYPE_CHECKING,
                    Iterator)

from setuptools import (find_packages,
                        setup)
from setuptools_rust import RustExtension

import rithm

if TYPE_CHECKING:
    from setuptools_rust import RustExtension


class LazyRustExtensions:
    def __iter__(self) -> Iterator['RustExtension']:
        from setuptools_rust import RustExtension
        yield RustExtension(rithm.__name__ + '._' + rithm.__name__)


project_base_url = 'https://github.com/lycantropos/rithm/'


def read_file(path_string: str) -> str:
    return Path(path_string).read_text(encoding='utf-8')


setup(name=rithm.__name__,
      packages=find_packages(exclude=('tests', 'tests.*')),
      rust_extensions=LazyRustExtensions(),
      version=rithm.__version__,
      description=rithm.__doc__,
      long_description=Path('README.md').read_text(encoding='utf-8'),
      long_description_content_type='text/markdown',
      author='Azat Ibrakov',
      author_email='azatibrakov@gmail.com',
      classifiers=[
          'License :: OSI Approved :: MIT License',
          'Programming Language :: Python :: 3.5',
          'Programming Language :: Python :: 3.6',
          'Programming Language :: Python :: 3.7',
          'Programming Language :: Python :: 3.8',
          'Programming Language :: Python :: 3.9',
          'Programming Language :: Python :: Implementation :: CPython',
          'Programming Language :: Python :: Implementation :: PyPy',
      ],
      license='MIT License',
      url=project_base_url,
      download_url=project_base_url + 'archive/master.zip',
      python_requires='>=3.5',
      setup_requires=read_file('requirements-setup.txt'),
      install_requires=read_file('requirements.txt'),
      include_package_data=True,
      zip_safe=False)
