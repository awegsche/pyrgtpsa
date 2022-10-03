from pyrgtpsa.pyrgtpsa import Tpsa6D4 as Tpsa
from numpy import pi

t1 = Tpsa([pi/6, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0])

print("ini")
print(t1)

t2 = t1.sin()

print("sin")
print(t2)

# asin not supported yet
# t2 = t2.asin()

# print("ssin")
# print(t2)

