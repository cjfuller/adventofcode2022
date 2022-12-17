import itertools
from typing import Generator, Iterator, Protocol

import numpy as np

from py.lib import read_input

WIDTH = 7
HEIGHT = 100000


def prealloc_grid() -> np.ndarray:
    return np.zeros((HEIGHT, WIDTH), dtype=bool)


# row, column
Coord = tuple[int, int]


class Piece(Protocol):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        pass


class HorizLine(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 4, 3), (top - 4, 4), (top - 4, 5)]


class VertLine(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 5, 2), (top - 6, 2), (top - 7, 2)]


class Plus(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 3), (top - 5, 2), (top - 5, 3), (top - 5, 4), (top - 6, 3)]


class RevL(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 4, 3), (top - 4, 4), (top - 5, 4), (top - 6, 4)]


class Square(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 4, 3), (top - 5, 2), (top - 5, 3)]


piece_order: list[Piece] = [HorizLine, Plus, RevL, VertLine, Square]


def piece_gen() -> Generator[Piece, None, None]:
    i = 0
    while True:
        yield piece_order[i % len(piece_order)]
        i += 1


def settle(grid: np.ndarray, coords: list[Coord], jets_iter: Iterator[str]):
    next_coords = coords
    jet = next(jets_iter)
    if jet == "<":
        if not any(c == 0 or grid[r, c - 1] for (r, c) in coords):
            next_coords = [(r, c - 1) for (r, c) in coords]
    elif jet == ">":
        if not any(c == WIDTH - 1 or grid[r, c + 1] for (r, c) in coords):
            next_coords = [(r, c + 1) for (r, c) in coords]

    next_down_coords = [(r + 1, c) for (r, c) in next_coords]

    if any(r == HEIGHT for r, _ in next_down_coords) or any(
        grid[r, c] for r, c in next_down_coords
    ):
        for r, c in next_coords:
            grid[r, c] = True
        return
    else:
        settle(grid, next_down_coords, jets_iter)


def calc_max_height(grid: np.ndarray, last_top: int) -> int:
    for i in range(last_top, 0, -1):
        if i == HEIGHT:
            continue
        if np.all(grid[i] == False):
            return i + 1

    assert False, "Didn't locate a max height"


def part1():
    jets = read_input(17).strip()
    # test: jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"
    grid = prealloc_grid()
    curr_height = HEIGHT
    piece_counter = 0

    piece_iter = iter(piece_gen())
    jet_iter = itertools.cycle(jets)

    while piece_counter < 2022:
        curr = next(piece_iter)
        coords = curr.place(curr_height)
        settle(grid, coords, jet_iter)
        print_grid(grid[HEIGHT - 10 : HEIGHT, :])
        curr_height = calc_max_height(grid, curr_height)
        print(curr_height)
        piece_counter += 1

    print(f"Part 1: {HEIGHT-curr_height}")


def print_grid(grid: np.ndarray):
    for row in grid:
        print("".join(map(lambda r: "#" if r else " ", row)))


def part2():
    pass


if __name__ == "__main__":
    part1()
    part2()
