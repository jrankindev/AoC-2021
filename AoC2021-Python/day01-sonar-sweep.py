### ADVENT OF CODE 2021 DAY 1 SONAR SWEEP ###


import helper.helper as helper


## * puzzle part 1
def puzzle_part_1(puzzle_input):
    for line in puzzle_input:
        print(line)

    return 0


## * puzzle part 2
def puzzle_part_2(puzzle_input):
    return 0


## * main function
def main():
    # print title
    helper.print_title("2021", "1", "Sonar Sweep")

    # get puzzle input
    puzzle_input = helper.load_puzzle_input("inputs/day01/pie.txt", 1, 0)

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
