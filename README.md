rithm
=====

[![](https://github.com/lycantropos/rithm/workflows/CI/badge.svg)](https://github.com/lycantropos/rithm/actions/workflows/ci.yml "Github Actions")
[![](https://codecov.io/gh/lycantropos/rithm/branch/master/graph/badge.svg)](https://codecov.io/gh/lycantropos/rithm "Codecov")
[![](https://img.shields.io/github/license/lycantropos/rithm.svg)](https://github.com/lycantropos/rithm/blob/master/LICENSE "License")
[![](https://badge.fury.io/py/rithm.svg)](https://badge.fury.io/py/rithm "PyPI")
[![](https://img.shields.io/crates/v/rithm.svg)](https://crates.io/crates/rithm "crates.io")

In what follows `python` is an alias for `python3.7` or `pypy3.7`
or any later version (`python3.8`, `pypy3.8` and so on).

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

Usage
-----

### Python

Arbitrary precision integer:
```python
>>> from rithm import Int
>>> Int()
rithm.Int(0)
>>> Int(16)
rithm.Int(16)
>>> Int("16")
rithm.Int(16)
>>> Int("0b10000", 2)
rithm.Int(16)
>>> Int("0o20", 8)
rithm.Int(16)
>>> Int("0x10", 16)
rithm.Int(16)
>>> Int("10000", 2)
rithm.Int(16)
>>> Int("20", 8)
rithm.Int(16)
>>> Int("10", 16)
rithm.Int(16)
>>> Int(16.99)
rithm.Int(16)
>>> Int(7) + Int(9)
rithm.Int(16)
>>> Int(25) - Int(9)
rithm.Int(16)
>>> Int(2) * Int(8)
rithm.Int(16)
>>> Int(32) / Int(2)
rithm.Fraction(rithm.Int(16), rithm.Int(1))
>>> Int(32) // Int(2)
rithm.Int(16)
>>> Int(33) % Int(17)
rithm.Int(16)
>>> Int(2) ** Int(4)
rithm.Int(16)

```

Exact fraction:
```python
>>> from rithm import Fraction
>>> Fraction()
rithm.Fraction(rithm.Int(0), rithm.Int(1))
>>> Fraction(1, 2)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(50, 100)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(0.5)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(1, 2) + 0
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> 0 + Fraction(1, 2)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(1, 3) + Fraction(1, 6)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(3, 2) - 1
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> 1 - Fraction(1, 2)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(1, 3) * Fraction(3, 2)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(1, 3) / Fraction(2, 3)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(1, 6) * 3
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> 1 / Fraction(2)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(3, 2) / 3
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(3, 2) // 1
rithm.Int(1)
>>> 2 // Fraction(3, 2)
rithm.Int(1)
>>> Fraction(3, 2) % 1
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> 2 % Fraction(3, 2)
rithm.Fraction(rithm.Int(1), rithm.Int(2))
>>> Fraction(1, 2) ** 2
rithm.Fraction(rithm.Int(1), rithm.Int(4))
>>> str(Fraction(1, 2))
'1/2'

```

### Rust

Arbitrary precision integer:
```rust
use std::convert::TryFrom;

use rithm::big_int;
use rithm::traits::{CheckedDivAsF32, CheckedDivAsF64, FromStrRadix, Pow, OppositionOf, Zeroable};

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;
const BINARY_SHIFT: usize = (OppositionOf::<Digit>::BITS - 2) as usize;
type BigInt = big_int::BigInt<Digit, '_', BINARY_SHIFT>;

assert_eq!(BigInt::zero(), 0);
assert_eq!(BigInt::from(16), 16);
assert_eq!(BigInt::try_from("16").unwrap(), 16);
assert_eq!(BigInt::try_from("0b10000").unwrap(), 16);
assert_eq!(BigInt::try_from("0o20").unwrap(), 16);
assert_eq!(BigInt::try_from("0x10").unwrap(), 16);
assert_eq!(BigInt::from_str_radix("10000", 2).unwrap(), 16);
assert_eq!(BigInt::from_str_radix("20", 8).unwrap(), 16);
assert_eq!(BigInt::from_str_radix("10", 16).unwrap(), 16);
assert_eq!(BigInt::try_from(16.99).unwrap(), 16);
assert_eq!(BigInt::from(7) + BigInt::from(9), 16);
assert_eq!(BigInt::from(25) - BigInt::from(9), 16);
assert_eq!(BigInt::from(2) * BigInt::from(8), 16);
assert_eq!(BigInt::from(32) / BigInt::from(2), 16);
assert_eq!(BigInt::from(33) % BigInt::from(17), 16);
#[cfg(target_arch = "x86")] // not supported by `u32` digits because conversion to `f32` is lossy
assert_eq!(BigInt::from(32).checked_div_as_f32(BigInt::from(2)), Ok(16.0));
assert_eq!(BigInt::from(32).checked_div_as_f64(BigInt::from(2)), Ok(16.0));
assert_eq!(BigInt::from(2).pow(BigInt::from(4)), 16);
```

Exact fraction:
```rust
use std::convert::TryFrom;
use rithm::fraction;
use rithm::traits::{CheckedDivEuclid, CheckedPow, CheckedRemEuclid, Zeroable};

type Fraction = fraction::Fraction<i8>;

assert_eq!(Fraction::zero(), 0);
assert_eq!(Fraction::new(1, 2), Some(Fraction::from(1) / 2));
assert_eq!(Fraction::new(50, 100), Fraction::new(1, 2));
assert_eq!(Fraction::try_from(0.5).unwrap(), Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 2).unwrap() + 0, Fraction::new(1, 2).unwrap());
assert_eq!(0 + Fraction::new(1, 2).unwrap(), Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 3).unwrap() + Fraction::new(1, 6).unwrap(),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap() - 1, Fraction::new(1, 2).unwrap());
assert_eq!(1 - Fraction::new(1, 2).unwrap(), Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 3).unwrap() * Fraction::new(3, 2).unwrap(),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 3).unwrap() / Fraction::new(2, 3).unwrap(),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 6).unwrap() * 3, Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap() / 3, Fraction::new(1, 2).unwrap());
assert_eq!(1 / Fraction::from(2), Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap().checked_div_euclid(1), Some(1));
assert_eq!(2.checked_div_euclid(Fraction::new(3, 2).unwrap()), Some(1));
assert_eq!(Fraction::new(3, 2).unwrap().checked_rem_euclid(1), Fraction::new(1, 2));
assert_eq!(2.checked_rem_euclid(Fraction::new(3, 2).unwrap()), Fraction::new(1, 2));
assert_eq!(Fraction::new(1, 2).unwrap().checked_pow(2), Fraction::new(1, 4));
assert_eq!(Fraction::new(1, 2).unwrap().to_string(), "1/2");
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
