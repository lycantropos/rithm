rithm
=====

[![](https://dev.azure.com/lycantropos/rithm/_apis/build/status/lycantropos.rithm?branchName=master)](https://dev.azure.com/lycantropos/rithm/_build/latest?definitionId=39&branchName=master "Azure Pipelines")
[![](https://codecov.io/gh/lycantropos/rithm/branch/master/graph/badge.svg)](https://codecov.io/gh/lycantropos/rithm "Codecov")
[![](https://img.shields.io/github/license/lycantropos/rithm.svg)](https://github.com/lycantropos/rithm/blob/master/LICENSE "License")
[![](https://badge.fury.io/py/rithm.svg)](https://badge.fury.io/py/rithm "PyPI")
[![](https://img.shields.io/crates/v/rithm.svg)](https://crates.io/crates/rithm "crates.io")

In what follows `python` is an alias for `python3.6` or `pypy3.6`
or any later version (`python3.7`, `pypy3.7` and so on).

Installation
------------

Install the latest `pip` & `setuptools` packages versions
```bash
python -m pip install --upgrade pip setuptools
```

### User

Download and install the latest stable version from `PyPI` repository
```bash
python -m pip install --upgrade rithm
```

### Developer

Download the latest version from `GitHub` repository
```bash
git clone https://github.com/lycantropos/rithm.git
cd rithm
```

Install dependencies
```bash
python -m pip install -r requirements.txt
```

Install
```bash
python setup.py install
```

Development
-----------

### Bumping version

#### Preparation

Install
[bump2version](https://github.com/c4urself/bump2version#installation).

#### Pre-release

Choose which version number category to bump following [semver
specification](http://semver.org/).

Test bumping version
```bash
bump2version --dry-run --verbose $CATEGORY
```

where `$CATEGORY` is the target version number category name, possible
values are `patch`/`minor`/`major`.

Bump version
```bash
bump2version --verbose $CATEGORY
```

This will set version to `major.minor.patch-alpha`. 

#### Release

Test bumping version
```bash
bump2version --dry-run --verbose release
```

Bump version
```bash
bump2version --verbose release
```

This will set version to `major.minor.patch`.

### Running tests

Install dependencies
```bash
python -m pip install -r requirements-tests.txt
```

Plain
```bash
pytest
```

Inside `Docker` container:
- with `CPython`
  ```bash
  docker-compose --file docker-compose.cpython.yml up
  ```
- with `PyPy`
  ```bash
  docker-compose --file docker-compose.pypy.yml up
  ```

`Bash` script:
- with `CPython`
  ```bash
  ./run-tests.sh
  ```
  or
  ```bash
  ./run-tests.sh cpython
  ```

- with `PyPy`
  ```bash
  ./run-tests.sh pypy
  ```

`PowerShell` script:
- with `CPython`
  ```powershell
  .\run-tests.ps1
  ```
  or
  ```powershell
  .\run-tests.ps1 cpython
  ```
- with `PyPy`
  ```powershell
  .\run-tests.ps1 pypy
  ```
