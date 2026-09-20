class Student:
    def __init__(self, name, courses):
        self.name = name
        self.course = courses

    def go_to_class(self):
        print(self.course)


print(Student.__dict__)
print()
s = Student("Bob", ["CS-1181", "CS-3180"])

print(s.__dict__)

# print(s.name)

s.__dict__["secret_key"] = "hello world"

Student.__dict__["go_to_class"](s)

print(s.secret_key)


print()
print()
print()
print()


def add(a, b):
    return a + b


def mult(a, b):
    return a * b
	 

def perform_operation(fn, a, b):
    return fn(a, b)


result = perform_operation(mult, 5, 6)

print(result)
