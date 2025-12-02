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
    max_num = 99
    min_num = 0

    for action in actions:
        print(action.dir, action.num)
        val = action.num
        amnt = 0

        # are we going be looping?
        if action.num > max_num:
            # what's our actual value going to be without those loops
            val = action.num % (max_num + 1)

            # how many times did we loop around (100 clicks)?
            amnt = abs(action.num // (max_num + 1))

        were_we_at_zero = at_num == 0

        match action.dir:
            case 'R':
                at_num += val

                # we need to correct - we moved past 99
                if at_num > max_num:
                    at_num -= max_num + 1

                    # if we did cross the threshold, we should add
                    # we will account for full 100 click loops later
                    if not were_we_at_zero and at_num != 0:
                        count += 1

            case 'L':
                at_num -= val

                # we need to correct - we moved past 0
                if at_num < min_num:
                    at_num += max_num + 1

                    # if we did cross the threshold, we should add
                    # we will account for full 100 click loops later
                    if not were_we_at_zero and at_num != 0:
                        count += 1

        # did we loop around?
        if amnt > 0:
            count += amnt

        # what if we landed on a 0
        if at_num == 0:
            count += 1

            # Woops if we ended up at zero, we might have overcounted amnt 
            # if we started at zero too
            if were_we_at_zero:
                count -= 1

        print(f" --- FINAL COUNT {count} --- ENDED UP AT {at_num} \n")

    print(count)
    
if __name__ == "__main__":
    solve()