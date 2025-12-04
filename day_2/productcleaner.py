input_file_path = "input.txt"

class Range:
    def __init__(self, min, max):
        self.min = min
        self.max = max

    def __repr__(self):
        return f"Range({self.min} -- {self.max})"

def validate_pattern(number):
        strnum = str(number)

        # ex. 1, 2, 3
        if len(strnum) == 1:
            return False

        # ex. 22222, 333, 444444
        if len(set(strnum)) == 1:
            return True

        divisible_list = [1]

        for i in range(2, min(6, len(strnum))):
            if len(strnum)%i == 0:
                divisible_list.append(i)

        # go through divisors and see if any work!!
        for divisor in divisible_list:
            l = 0
            r = divisor

            while r < len(strnum):
                if strnum[l] != strnum[r]:
                    break
                l+=1
                r+=1
            
            if r == len(strnum):
                return True

        return False

def process_file():
    ranges = []

    try:
        with open(input_file_path, 'r') as file:
            lines = file.readlines()

            for range in lines[0].split(","):
                curr_min = int(range.split("-")[0])
                curr_max = int(range.split("-")[1])

                ranges.append(Range(curr_min, curr_max))
            
    except FileNotFoundError:
        print(f"Error: The file '{file_path}' was not found.")
    except Exception as e:
        print(f"An error occurred: {e}")

    return ranges

def solve():
    ranges = process_file()
    count = 0

    for minmax in ranges:
        for num in range(minmax.min, minmax.max + 1):
            if validate_pattern(num):
                count+= num
                print("FOUND AN ANOMALY", num)

    return count

if __name__ == "__main__":
    print(solve())