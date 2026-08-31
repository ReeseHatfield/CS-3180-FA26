def calculate_total(price, quantity):
    return price * quantity


print(calculate_total(4.99, 3))


try:
    print(calculate_total(4.99, "three"))
except TypeError as e:
    print(f"oopies: {e}")