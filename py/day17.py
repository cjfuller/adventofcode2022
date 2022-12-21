import itertools
from dataclasses import dataclass
from typing import Any, Generator, Iterator, Protocol, Self

import numpy as np

from py.lib import read_input

WIDTH = 7
HEIGHT = 100000000


def prealloc_grid() -> np.ndarray:
    return np.zeros((HEIGHT, WIDTH), dtype=bool)


# row, column
Coord = tuple[int, int]


class Piece(Protocol):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        pass

    @classmethod
    def place_2(cls, top: int) -> list[Coord]:
        pass


class HorizLine(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 4, 3), (top - 4, 4), (top - 4, 5)]

    @classmethod
    def place_2(cls, top: int) -> list[Coord]:
        return [(top + 4, 2), (top + 4, 3), (top + 4, 4), (top + 4, 5)]


class VertLine(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 5, 2), (top - 6, 2), (top - 7, 2)]

    @classmethod
    def place_2(cls, top: int) -> list[Coord]:
        return [(top + 4, 2), (top + 5, 2), (top + 6, 2), (top + 7, 2)]


class Plus(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 3), (top - 5, 2), (top - 5, 3), (top - 5, 4), (top - 6, 3)]

    @classmethod
    def place_2(cls, top: int) -> list[Coord]:
        return [(top + 4, 3), (top + 5, 2), (top + 5, 3), (top + 5, 4), (top + 6, 3)]


class RevL(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 4, 3), (top - 4, 4), (top - 5, 4), (top - 6, 4)]

    @classmethod
    def place_2(cls, top: int) -> list[Coord]:
        return [(top + 4, 2), (top + 4, 3), (top + 4, 4), (top + 5, 4), (top + 6, 4)]


class Square(Piece):
    @classmethod
    def place(cls, top: int) -> list[Coord]:
        return [(top - 4, 2), (top - 4, 3), (top - 5, 2), (top - 5, 3)]

    @classmethod
    def place_2(cls, top: int) -> list[Coord]:
        return [(top + 4, 2), (top + 4, 3), (top + 5, 2), (top + 5, 3)]


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


def settle2(
    grid_tops: list[int],
    coords: list[Coord],
    jets_iter: Iterator[str],
):
    next_coords = coords
    jet = next(jets_iter)
    if jet == "<":
        if not any(c == 0 or grid_tops[c - 1] >= r for r, c in coords):
            next_coords = [(r, c - 1) for (r, c) in coords]
    elif jet == ">":
        if not any(c == WIDTH - 1 or grid_tops[c + 1] >= r for r, c in coords):
            next_coords = [(r, c + 1) for (r, c) in coords]

    next_down_coords = [(r - 1, c) for (r, c) in next_coords]

    if any(grid_tops[c] >= r for r, c in next_down_coords):
        for r, c in next_coords:
            if r > grid_tops[c]:
                grid_tops[c] = r
        return
    else:
        settle2(grid_tops, next_down_coords, jets_iter)


def calc_max_height(grid: np.ndarray, last_top: int) -> int:
    for i in range(last_top, 0, -1):
        if i == HEIGHT:
            continue
        if np.all(grid[i] == False):
            return i + 1

    assert False, "Didn't locate a max height"


def part1():
    jets = read_input(17).strip()
    # jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"  # test
    grid = prealloc_grid()
    curr_height = HEIGHT
    piece_counter = 0

    piece_iter = iter(piece_gen())
    jet_iter = itertools.cycle(jets)

    while piece_counter < 2022:
        curr = next(piece_iter)
        coords = curr.place(curr_height)
        settle(grid, coords, jet_iter)
        curr_height = calc_max_height(grid, curr_height)
        piece_counter += 1

    print(f"Part 1: {HEIGHT-curr_height}")


def part1_alt():
    jets = read_input(17).strip()
    # jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"  # test
    grid_tops = [0, 0, 0, 0, 0, 0, 0]
    piece_counter = 0
    curr_height = 0

    piece_iter = iter(piece_gen())
    jet_iter = itertools.cycle(jets)

    while piece_counter < 2022:
        curr = next(piece_iter)
        coords = curr.place_2(curr_height)
        settle2(grid_tops, coords, jet_iter)
        curr_height = max(grid_tops)
        piece_counter += 1

    print(f"Part 1: {curr_height}")


def print_grid(grid: np.ndarray):
    for row in grid:
        print("".join(map(lambda r: "#" if r else " ", row)))


@dataclass
class CycleIter:
    inner: Any
    index: int

    def __iter__(self) -> Self:
        return self

    def __next__(self) -> str:
        next_jet = self.inner[self.index]
        self.index = (self.index + 1) % len(self.inner)
        return next_jet


def part2():
    jets = read_input(17).strip()
    # jets = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"  # test
    grid = prealloc_grid()
    curr_height = HEIGHT
    piece_counter = 0

    piece_iter = CycleIter(piece_order, 0)
    jet_iter = CycleIter(jets, 0)

    target_num = 2022
    num_repeats = target_num // (len(jets) * len(piece_order))
    extra = target_num % (len(jets) * len(piece_order))
    height_from_extra = 0

    # print(f"nr: {num_repeats}, ex: {extra}")

    seen = set()
    orig = None
    orig_counter = None

    pre_height = 0
    heights_within_repeat = []

    start = 10000
    period = 1710
    # repeated_at = 3640 + (3640 - 1930) * 41

    while piece_counter < 1000000000000:
        curr = next(piece_iter)
        coords = curr.place(curr_height)
        settle(grid, coords, jet_iter)
        curr_height = calc_max_height(grid, curr_height)
        piece_counter += 1

        if piece_counter in range(start, start + period):
            heights_within_repeat.append(HEIGHT - curr_height)

        # if piece_iter.index == 4 and jet_iter.index == 292 and orig is None:
        #     orig = curr_height
        #     orig_counter = piece_counter

        if piece_counter > start + period:
            break

        # if (piece_iter.index, jet_iter.index) == (0, 1280) and piece_counter > start:
        #     repeated_at = piece_counter
        #     break

        if (piece_iter.index, jet_iter.index) in seen:
            print(
                f"Saw repeat indices at {piece_counter - start} pieces from state at {orig_counter}"
            )
            print((piece_iter.index, jet_iter.index))
            break
        elif piece_counter >= start:
            # print(f"Adding {(piece_iter.index, jet_iter.index)} to set")
            seen.add((piece_iter.index, jet_iter.index))

        # if np.all(grid[curr_height, :]):
        #     print(f"Detected flat after {piece_counter} pieces")
        #
        # if (piece_counter - 15) % 35 == 0:
        #     print(f"------- { piece_counter } -------")
        #     print_grid(grid[curr_height : curr_height + 20, :])
        #
        # if piece_counter > 400:
        #     break
        # if piece_counter == 1754 or (piece_counter - 1754) % 1700 == 0:
        #     print(f"----------- {piece_counter} ------------")
        #     print_grid(grid[curr_height : curr_height + 20, :])

    print(start)
    print(period)

    target = 1000000000000

    num_repeats = (target - start) // period
    leftover = (target - start) % period

    height_at_target = (
        heights_within_repeat[0]
        + num_repeats * (heights_within_repeat[-1] - heights_within_repeat[0])
        + (heights_within_repeat[leftover] - heights_within_repeat[0])
    )
    print(height_at_target)

    # # print_grid(grid[HEIGHT - 20 : HEIGHT, :])
    # # print("---")
    # print_grid(grid[curr_height : curr_height + 20, :])
    # print("---")
    # print_grid(grid[orig : orig + 20, :])


if __name__ == "__main__":
    # part1()
    # part1_alt()
    part2()
