import os
import requests
from dotenv import load_dotenv

def get_puzzle_input(session_token):
    inputs = list(map(int,requests.get(
        "https://adventofcode.com/2019/day/4/input",
        cookies={"session": session_token}
    ).text.strip().split('-')))
    return inputs[0], inputs[1]

def has_adjacent_double(x):
    c = str(x)
    for i in range(10):
        if "%d%d"%(i,i) in c:
            return True
    return False

def non_decreasing(x):
    s = str(x)
    p = 0
    for c in s:
        if int(c) < p:
            return False
        p = int(c)
    return True

def partOne(l, u):
    return len([x for x in range(l,u) if has_adjacent_double(x) and non_decreasing(x)])

if __name__ == "__main__":
    load_dotenv()
    lower, upper = get_puzzle_input(os.getenv("session"))
    print(partOne(lower, upper))
