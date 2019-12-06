import os
import requests
from dotenv import load_dotenv
from typing import Tuple, Callable, List

def get_puzzle_input(session_token: str) -> Tuple[int,int]:
    inputs = list(map(int,requests.get(
        "https://adventofcode.com/2019/day/4/input",
        cookies={"session": session_token}
    ).text.strip().split('-')))
    return inputs[0], inputs[1]

def has_adjacent_double(x: int) -> bool:
    c = str(x)
    for i in range(10):
        if "%d%d"%(i,i) in c:
            return True
    return False

def has_adjacent_double_isolated(x: int) -> bool:
    c = str(x)
    for i in range(10):
        if "%d%d"%(i,i) in c and "%d%d%d"%(i,i,i) not in c:
            return True
    return False

def non_decreasing(x: int) -> bool:
    s = str(x)
    p = 0
    for c in s:
        if int(c) < p:
            return False
        p = int(c)
    return True

def amt_in_range_meeting_constraints(
        l: int,
        u: int,
        constraints: List[Callable[[int],bool]] = []
    ) -> int:
    return len([x for x in range(l,u) if False not in map(lambda f: f(x), constraints)])

def partOne(l: int, u: int) -> int:
    return amt_in_range_meeting_constraints(l,u,[
        has_adjacent_double,
        non_decreasing
    ])

def partTwo(l: int, u: int) -> int:
    return amt_in_range_meeting_constraints(l,u,[
        has_adjacent_double_isolated,
        non_decreasing
    ])

if __name__ == "__main__":
    load_dotenv()
    lower, upper = get_puzzle_input(os.getenv("session"))
    print(partOne(lower, upper))
    print(partTwo(lower, upper))
