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

#### Arbitrary precision integer

With setup
```python
>>> from rithm.integer import Int

```
we can:
- construct
  ```python
  >>> Int()
  Int(0)
  >>> Int(9)
  Int(9)
  >>> Int('9')
  Int(9)
  >>> Int('0b1001', 2)
  Int(9)
  >>> Int('0o11', 8)
  Int(9)
  >>> Int('0x9', 16)
  Int(9)
  >>> Int('1001', 2)
  Int(9)
  >>> Int('0o11', 8)
  Int(9)
  >>> Int('9', 16)
  Int(9)
  >>> Int(9.99)
  Int(9)

  ```
- compare
  ```python
  >>> Int(9) == Int(9)
  True
  >>> Int(9) >= Int(9)
  True
  >>> Int(9) > Int(8)
  True
  >>> Int(9) <= Int(9)
  True
  >>> Int(9) < Int(10)
  True
  
  ```
- calculate
  ```python
  >>> abs(Int(-9))
  Int(9)
  >>> Int(4) + Int(5)
  Int(9)
  >>> Int(9) & Int(11)
  Int(9)
  >>> Int(19) // Int(2)
  Int(9)
  >>> ~Int(-10)
  Int(9)
  >>> Int(19) % Int(10)
  Int(9)
  >>> Int(3) * Int(3)
  Int(9)
  >>> -Int(-9)
  Int(9)
  >>> Int(1) | Int(8)
  Int(9)
  >>> Int(3) ** Int(2)
  Int(9)
  >>> Int(5) << Int(1)
  Int(10)
  >>> Int(5) >> Int(1)
  Int(2)
  >>> Int(25) - Int(16)
  Int(9)
  >>> Int(18) / Int(2)
  Fraction(Int(9), Int(1))
  >>> Int(2) ^ Int(11)
  Int(9)
  
  ```

#### Exact fraction

With setup
```python
>>> from rithm.fraction import Fraction

```
we can:
- construct
  ```python
  >>> Fraction()
  Fraction(Int(0), Int(1))
  >>> Fraction(1)
  Fraction(Int(1), Int(1))
  >>> Fraction(1, 2)
  Fraction(Int(1), Int(2))
  >>> Fraction(50, 100)
  Fraction(Int(1), Int(2))
  >>> Fraction(0.5)
  Fraction(Int(1), Int(2))

  ```
- compare
  ```python
  >>> Fraction(1, 2) == Fraction(1, 2)
  True
  >>> Fraction(1, 2) >= Fraction(1, 2)
  True
  >>> Fraction(1, 2) > Fraction(1, 3)
  True
  >>> Fraction(1, 2) < Fraction(2, 3)
  True
  >>> Fraction(1, 2) != Fraction(1, 3)
  True

  ```
- calculate
  ```python
  >>> abs(Fraction(-1, 2))
  Fraction(Int(1), Int(2))
  >>> Fraction(1, 3) + Fraction(1, 6)
  Fraction(Int(1), Int(2))
  >>> Fraction(3, 2) // Fraction(1)
  Int(1)
  >>> Fraction(3, 2) % Fraction(1)
  Fraction(Int(1), Int(2))
  >>> Fraction(1, 3) * Fraction(3, 2)
  Fraction(Int(1), Int(2))
  >>> -Fraction(-1, 2)
  Fraction(Int(1), Int(2))
  >>> Fraction(1, 2) ** 2
  Fraction(Int(1), Int(4))
  >>> Fraction(3, 2) - Fraction(1)
  Fraction(Int(1), Int(2))
  >>> Fraction(1, 3) / Fraction(2, 3)
  Fraction(Int(1), Int(2))

  ```

### Rust

#### Arbitrary precision integer

```rust
/// With setup
use std::convert::TryFrom;
use traiter::numbers::{
    Abs, DivEuclid, FromStrRadix, Pow, RemEuclid, Zeroable
};
use rithm::big_int;

#[cfg(target_arch = "x86")]
type Digit = u16;
#[cfg(not(target_arch = "x86"))]
type Digit = u32;
const DIGIT_BITNESS: usize = (Digit::BITS - 1) as usize;
const _: () = assert!(big_int::is_valid_digit_bitness::<Digit, DIGIT_BITNESS>());
type BigInt = big_int::BigInt<Digit, DIGIT_BITNESS>;
/// we can:
/// - construct
assert_eq!(BigInt::zero(), 0);
assert_eq!(BigInt::from(9), 9);
assert_eq!(BigInt::try_from("9").unwrap(), 9);
assert_eq!(BigInt::try_from("0b1001").unwrap(), 9);
assert_eq!(BigInt::try_from("0o11").unwrap(), 9);
assert_eq!(BigInt::try_from("0x9").unwrap(), 9);
assert_eq!(BigInt::from_str_radix("1001", 2).unwrap(), 9);
assert_eq!(BigInt::from_str_radix("11", 8).unwrap(), 9);
assert_eq!(BigInt::from_str_radix("9", 16).unwrap(), 9);
assert_eq!(BigInt::try_from(9.99).unwrap(), 9);
/// - compare
assert!(BigInt::from(9) == BigInt::from(9));
assert!(BigInt::from(9) >= BigInt::from(9));
assert!(BigInt::from(9) > BigInt::from(8));
assert!(BigInt::from(9) <= BigInt::from(9));
assert!(BigInt::from(9) < BigInt::from(10));
/// - calculate
assert_eq!(BigInt::from(-9).abs(), 9);
assert_eq!(BigInt::from(4) + BigInt::from(5), 9);
assert_eq!(BigInt::from(9) & BigInt::from(11), 9);
assert_eq!(BigInt::from(1) | BigInt::from(8), 9);
assert_eq!(BigInt::from(2) ^ BigInt::from(11), 9);
assert_eq!(BigInt::from(19) / BigInt::from(2), 9);
assert_eq!(BigInt::from(19).div_euclid(BigInt::from(2)), 9);
assert_eq!(BigInt::from(3) * BigInt::from(3), 9);
assert_eq!(-BigInt::from(-9), 9);
assert_eq!(!BigInt::from(-10), 9);
assert_eq!(BigInt::from(3).pow(BigInt::from(2)), 9);
assert_eq!(BigInt::from(19) % BigInt::from(10), 9);
assert_eq!(BigInt::from(19).rem_euclid(BigInt::from(10)), 9);
assert_eq!(BigInt::from(5) << 1, 10);
assert_eq!(BigInt::from(5) >> 1, 2);
assert_eq!(BigInt::from(25) - BigInt::from(16), 9);
```

#### Exact fraction

```rust
/// With setup
use std::convert::TryFrom;
use traiter::numbers::{Abs, DivEuclid, Pow, RemEuclid, Unitary, Zeroable};
use rithm::fraction;

type Fraction = fraction::Fraction<i8>;
/// we can:
/// - construct
assert_eq!(Fraction::zero(), 0);
assert_eq!(Fraction::one(), 1);
assert_eq!(Fraction::new(1, 2), Some(Fraction::from(1) / 2));
assert_eq!(Fraction::new(50, 100), Fraction::new(1, 2));
assert_eq!(Fraction::try_from(0.5).unwrap(), Fraction::new(1, 2).unwrap());
/// - compare
assert!(Fraction::new(1, 2).unwrap() == Fraction::new(1, 2).unwrap());
assert!(Fraction::new(1, 2).unwrap() >= Fraction::new(1, 2).unwrap());
assert!(Fraction::new(1, 2).unwrap() > Fraction::new(1, 3).unwrap());
assert!(Fraction::new(1, 2).unwrap() <= Fraction::new(1, 2).unwrap());
assert!(Fraction::new(1, 2).unwrap() < Fraction::new(2, 3).unwrap());
assert!(Fraction::new(1, 2).unwrap() != Fraction::new(1, 3).unwrap());
/// - calculate
assert_eq!(Fraction::new(-1, 2).unwrap().abs(), Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 3).unwrap() + Fraction::new(1, 6).unwrap(),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 3).unwrap() / Fraction::new(2, 3).unwrap(),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap().div_euclid(Fraction::from(1)), 1);
assert_eq!(Fraction::new(1, 3).unwrap() * Fraction::new(3, 2).unwrap(),
           Fraction::new(1, 2).unwrap());
assert_eq!(-Fraction::new(-1, 2).unwrap(), Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(1, 2).unwrap().pow(2), Fraction::new(1, 4).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap() % Fraction::from(1),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap().rem_euclid(Fraction::from(1)),
           Fraction::new(1, 2).unwrap());
assert_eq!(Fraction::new(3, 2).unwrap() - Fraction::from(1),
           Fraction::new(1, 2).unwrap());
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
