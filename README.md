# PyRGTPSA

Python bindings to [`rgtpsa`](https://github.com/awegsche/rgtpsa), a Generalised Truncated Power Series Algebra package.

**Note**: this is still WIP.

## Introduction

TPSAs provide multivariate polynomials for calculating physical processes exact up to a certain order.
They are used among others in simulation codes for particle accelerators like PTC (link?) and 
[MAD-NG](https://github.com/MethodicalAcceleratorDesign/MAD).

## Installation

- First, you need to install `rustc` as `rgtpsa` is a rust package, follow the steps on the 
[rust website](https://www.rust-lang.org/tools/install).

- then, you need `maturin` to create the python bindings and install the package:

```bash
pip install maturin
```

- Finally, clone this repo:

```bash
git clone <repo-addr>
```

- Go into the cloned folder and build and install `pyrgtpsa`:

```bash
cd pyrgtpsa 
maturin develop
```

Now you should be good to go. Let's try it out with a small test:
create a new python file, say `test.py`

```python
from pyrgtpsa.pyrgtpsa import Tpsa6D

x = Tpsa6D([2.0])
print(f"x =\n{x}")

print(f"x^2 =\n{x*x}")
```

The output should be something like

```
x =
  I | coeff                | Exp               | Ord
----+----------------------+-------------------+----
  0 |    2.0000000000000e0 |  0  0  0  0  0  0 |  0

x^2 =
  I | coeff                | Exp               | Ord
----+----------------------+-------------------+----
  0 |    4.0000000000000e0 |  0  0  0  0  0  0 |  0
```

**Warning:** please note, that for now, only `Float`s are working in the constructor of `TpsaND`,
all other types (including integers) are truncated from the array, often resulting in quite a mess.
I will addresss this soon.

## The fun part

Now, calculating `2*2` is not that exciting, let's take a look at the `sin` function:

```python
phi = Tpsa6D([pi/2.0])

print(f"sin(pi/2) =\n{phi.sin()})
```

which should yield the well known

```
sin(pi/2) =
  I | coeff                | Exp               | Ord
----+----------------------+-------------------+----
  0 |    1.0000000000000e0 |  0  0  0  0  0  0 |  0
```

But now the magic of TPSA enters into the picture: the `sin` function is internally implemented using
only arithmetic operators (`+`, `-`, `*`, `/`).

Now, we can replicate this in python as well, if we define our own `sin` function:

```python
def my_sin(tpsa):
    result = tpsa.copy()
    factor = tpsa.copy()

    for k in range(1,10):
        factorial = -1.0 / (2*k * (2*k+1))
        factor *= factorial
        factor *= tpsa
        factor *= tpsa
        result += factor
    return result


print(f"sin(pi/2) =\n{phi.sin()}")
print(f"my_sin(pi/2) =\n{my_sin(phi)}")
```

(_note_ that the calculation order `range(1,10)` is arbitrary here and should be adapted according to the order of the Tpsa)

and, running it, should yield:

```
sin(pi/2) =
  I | coeff                | Exp               | Ord
----+----------------------+-------------------+----
  0 |    1.0000000000000e0 |  0  0  0  0  0  0 |  0

my_sin(pi/2) =
  I | coeff                | Exp               | Ord
----+----------------------+-------------------+----
  0 |    1.0000000000000e0 |  0  0  0  0  0  0 |  0
```
