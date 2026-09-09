
def evil_fn():
    return "rm -rf /"

x = 10
y = 15
result = eval("x + y")

print(result)

def five():
    return 5

print(7)


def main():
    to_eval = input("Give me something to evaluate: ")
    result = eval(to_eval)
    print(result)


if __name__ == "__main__":
    main()