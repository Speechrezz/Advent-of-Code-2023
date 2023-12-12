from typing import List

def extractNumbers(line: str):
    allNumbers: List[int] = []

    numbers = "0123456789"
    for char in line:
        if char in numbers:
            allNumbers.append(char)

    return allNumbers[0] + allNumbers[-1]

output: List[int] = []

with open('input.txt') as file:
    lines = file.readlines()
    
    for line in lines:
        output.append(extractNumbers(line))

with open('python_output.txt', 'w') as file:
    file.write("\n".join(output))