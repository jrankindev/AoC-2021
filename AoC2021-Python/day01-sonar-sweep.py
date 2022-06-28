### ADVENT OF CODE 2021 DAY 1 SONAR SWEEP ###


import helper.helper as helper


## * puzzle part 1
def puzzle_part_1(puzzle_input):
    # create a variable to count depth increases
    depth_increases = 0

    # loop through every depth measurement in puzzle input
    for i, depth in enumerate(puzzle_input):

        # if not the first puzzle
        if i > 0:

            # if depth is greater than previous puzzle input then increase counter
            if depth > puzzle_input[i - 1]:
                depth_increases += 1

    # return total count of depth increases
    return depth_increases


## * puzzle part 2
def puzzle_part_2(puzzle_input):
    # create a variable to count depth increases
    depth_increases = 0

    # length of each measurement window
    window_length = 3

    # calculate first window sum
    window_sum = sum(puzzle_input[:window_length])

    # implement window sliding technique by looping through range of length of puzzle input minus the window length
    for i in range(len(puzzle_input) - window_length):
        previous_window_sum = window_sum
        window_sum = window_sum - puzzle_input[i] + puzzle_input[i + window_length]
        
        # if current window sum is greater than previous, then increase counter
        if window_sum > previous_window_sum:
            depth_increases += 1

    # retur total count of depth increases
    return depth_increases


## * main function
def main():
    # print title
    helper.print_title("2021", "1", "Sonar Sweep")

    # get puzzle input
    puzzle_input = helper.load_puzzle_input("inputs/day01/pi.txt", 1, 0)

    # solve part 1 and print QA
    answer = puzzle_part_1(puzzle_input)
    helper.print_question_answer(
        "How many measurements are larger than the previous measurement? ", answer,
    )

    # solve part 2 and print QA
    answer = puzzle_part_2(puzzle_input)
    helper.print_question_answer(
        "Consider sums of a three-measurement sliding window. How many sums are larger than the previous sum? ",
        answer,
    )


if __name__ == "__main__":
    main()
