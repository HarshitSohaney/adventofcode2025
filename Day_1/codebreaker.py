input_file_path = "input.txt"

class Action:
    def __init__(self, dir, num):
        self.dir = dir
        self.num = num

    def __repr__(self):
            return f"Action(dir='{self.dir}', num={self.num})"

def process_file():
    actions = []
    try:
        with open(input_file_path, 'r') as file:
            lines = file.readlines()

            for line in lines:
                actions.append(Action(line[:1], int(line[1:])))
    except FileNotFoundError:
        print(f"Error: The file '{file_path}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

    return actions

def solve():
    actions = process_file()

    count = 0

    at_num = 50
    max_num = 100
    min_num = 0

    for action in actions:
        print(action.dir, action.num)
        val = action.num
        amnt = 0
        if action.num >= max_num:
            val = action.num % 100
            amnt = abs(action.num // 100)

        match action.dir:
            case 'R':
                at_num += val
                print("at:", at_num, " count:", count, " any loops:", amnt)

                # we need to correct - we moved past 99
                if at_num >= max_num:
                    at_num = at_num - max_num
                if amnt > 0:
                    count += amnt

            case 'L':
                at_num -= val
                print("at:", at_num, " count:", count, " any loops:", amnt)

                # we need to correct - we moved past 0
                if at_num < min_num:
                    at_num += max_num
                if amnt > 0:
                    count += amnt

        print("FINAL AT NUM", at_num)

        if at_num == 0:
            count += 1

            if amnt > 0:
                count -= 1
            print("^^ SAW A 0, ", count)
        print("\n")

    print(count)
    
if __name__ == "__main__":
    solve()