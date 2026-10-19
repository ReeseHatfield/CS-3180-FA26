
def with_logging(func):

    def wrapper(*args, **kwargs):
        print("Started execution")
        res = func(*args, **kwargs)
        print("Terminated execution")
        return res


    return wrapper


@with_logging
def greet():
    print("hello world")


def my_sum(*args):
    total = 0
    for value in args:
        total += value
    
    return total

def display_profile(**kwargs):
    for k, v in kwargs.items():
        print(f"{k}: {v}")

# display_profile(
#     user_name="Reese",
#     uid=123456
# )

# print(my_sum(1, 2, 3, 4, 5))

# greet() # wrapper(greet)

@with_logging
def foo(a, b):
    print(f"a={a}, b={b}")
    return a + b

# print(foo(1, 2))



cache = {}

def memoizer(func):
    global cache

    def memoized_fn(value):
        try:
            result = cache[value]
        except KeyError:
            result = func(value)
            cache[value] = result
        return result

    return memoized_fn

@memoizer
def fibonacci(value):
    if value <= 1:
        return 1
    
    return fibonacci(value - 1) + fibonacci(value - 2)

for i in range(120):
    print(fibonacci(i))
