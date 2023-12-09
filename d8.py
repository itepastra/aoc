from itertools import cycle
from math import lcm
import re

def read_input_file():
    with open("data/day8.txt", "r") as input_file:
        lines = input_file.read().splitlines()
        instructions = [0 if instruction == "L" else 1 for instruction in lines[0]]
        nodes = {label: (left, right) for line in lines[2:]
                 for label, left, right in [tuple(re.findall(r"\w+", line))]}
        return instructions, nodes

def part_one(instructions, nodes):
    label = "AAA"
    steps = 0
    for instruction in cycle(instructions):
        if label == "ZZZ":
            break
        label = nodes[label][instruction]
        steps += 1
    return steps

def part_two(instructions, nodes):
    starting_labels = [label for label in nodes.keys() if label.endswith("A")]
    steps_list = []
    for label in starting_labels:
        steps = 0
        for instruction in cycle(instructions):
            if label.endswith("Z"):
                print(f"{label} ended after {steps} steps")
                steps_list.append(steps)
                break
            label = nodes[label][instruction]
            steps += 1
    return lcm(*steps_list)

(instructions, nodes) = read_input_file()
print(part_one(instructions, nodes))
print(part_two(instructions, nodes))
