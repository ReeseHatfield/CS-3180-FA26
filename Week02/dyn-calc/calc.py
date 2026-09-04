
def is_expression(data: str):
    return '+' in data

def read_file() -> list[str]:
    """
    Reads lines from input.txt
    """

    with open("input.txt", "r") as f:
        lines = f.readlines()

    # copy the data, instead of mutable reference
    # for line in lines:
    #     line = line.strip()

    lines: list[str] = [line.strip() for line in lines]

    return lines


    
data_table = {}

def dynamic_add(a, b):
    global data_table

    val_a = data_table[a]
    val_b = data_table[b]

    try:
        result = val_a + val_b
    except TypeError:
        result = f"{val_a}{val_b}"
    
    return result

def process_line(line: str):
    # two cases
    global data_table

    parts = line.split(":")
    variable = parts[0]
    data = parts[1]
    print(data)

    if is_expression(data):
        parts = data.split(" ")
        first_var = parts[0]
        plus = parts[1]
        second_var = parts[2]

        data_table[variable] = dynamic_add(first_var, second_var)
    else:
        # handling literal
        try:
            value = int(data)
            data_table[variable] = value
        except ValueError:
            value = data.strip("\'")
            data_table[variable] = value



def main():
    global data_table

    lines = read_file()

    for line in lines:
        process_line(line)

    print(data_table)




if __name__ == "__main__":
    main()