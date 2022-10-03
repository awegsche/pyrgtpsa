from pyrgtpsa.pyrgtpsa import Tpsa6D4 as Tpsa
from numpy import pi

# constructing a `Tpsa`
# the constructor takes a list of floats
x = Tpsa([2.0])

# calculating x^2 and printing the result
print(f"x =\n{x}")
print(f"x^2 =\n{x*x}")


# an angle
phi = Tpsa([pi/2.0])

# printing sin(phi)
print(f"sin(pi/2) =\n{phi.sin()}")

# defining our own `sin` function, using the power of TPSA
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


# calculating and printing sin with our new function
print(f"my_sin(pi/2) =\n{my_sin(phi)}")

