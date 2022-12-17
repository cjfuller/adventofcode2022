import os
from typing import Protocol, Self, TypeVar


class FromStr(Protocol):
    @classmethod
    def from_str(cls, s: str) -> Self:
        ...


S = TypeVar("S", bound=FromStr)


def read_input(day: int) -> str:
    if day == -1:
        return """\
Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II"""

    with open(
        os.path.join(
            os.path.dirname(os.path.dirname(__file__)), "inputs", f"day{day:02}.txt"
        )
    ) as f:
        return f.read()


def read_input_lines(day: int, include_empty: bool) -> list[str]:
    return [
        line.strip() for line in read_input(day).split("\n") if line or include_empty
    ]


def read_input_lines_as(day: int, t: type[S]) -> list[S]:
    return [t.from_str(s) for s in read_input_lines(day, False)]


def read_blank_line_delimited_blocks_as(day, t: type[S]) -> list[S]:
    return [t.from_str(s) for s in read_input(day).split("\n\n")]
