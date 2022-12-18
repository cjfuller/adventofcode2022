from dataclasses import dataclass
from typing import Self

import numpy as np

from py.lib import read_input_lines_as


@dataclass
class Coord:
    x: int
    y: int
    z: int

    @classmethod
    def from_str(cls, s: str) -> Self:
        return Coord(*[int(x.strip()) for x in s.split(",")])


def count_surface_area(inputs: list[Coord], grid: np.ndarray) -> int:
    face_count = 0
    for i in inputs:
        face_count += grid[i.x - 1, i.y, i.z]
        face_count += grid[i.x + 1, i.y, i.z]
        face_count += grid[i.x, i.y - 1, i.z]
        face_count += grid[i.x, i.y + 1, i.z]
        face_count += grid[i.x, i.y, i.z - 1]
        face_count += grid[i.x, i.y, i.z + 1]

    return face_count


def part1():
    input = read_input_lines_as(18, Coord)
    x_max = max(i.x for i in input)
    y_max = max(i.y for i in input)
    z_max = max(i.z for i in input)
    grid = np.ones((x_max + 2, y_max + 2, z_max + 2), dtype=int)

    for i in input:
        grid[i.x, i.y, i.z] = 0

    face_count = count_surface_area(input, grid)
    print(f"Part 1: {face_count}")


def part2():
    input = read_input_lines_as(18, Coord)
    x_max = max(i.x for i in input)
    y_max = max(i.y for i in input)
    z_max = max(i.z for i in input)
    grid = np.ones((x_max + 2, y_max + 2, z_max + 2), dtype=int)

    for i in input:
        grid[i.x, i.y, i.z] = 0

    watershed_grid = np.zeros_like(grid)
    watershed_grid[0, 0, 0] = 1
    changed = True

    while changed:
        changed = False
        for x in range(0, watershed_grid.shape[0]):
            for y in range(0, watershed_grid.shape[1]):
                for z in range(0, watershed_grid.shape[2]):
                    if watershed_grid[x, y, z] or not grid[x, y, z]:
                        continue

                    if x > 0:
                        if watershed_grid[x - 1, y, z] > 0 and grid[x - 1, y, z] > 0:
                            watershed_grid[x, y, z] = 1
                            changed = True
                            continue
                    if x < watershed_grid.shape[0] - 1:
                        if watershed_grid[x + 1, y, z] > 0 and grid[x + 1, y, z] > 0:
                            watershed_grid[x, y, z] = 1
                            changed = True
                            continue

                    if y > 0:
                        if watershed_grid[x, y - 1, z] > 0 and grid[x, y - 1, z] > 0:
                            watershed_grid[x, y, z] = 1
                            changed = True
                            continue
                    if y < watershed_grid.shape[1] - 1:
                        if watershed_grid[x, y + 1, z] > 0 and grid[x, y + 1, z] > 0:
                            watershed_grid[x, y, z] = 1
                            changed = True
                            continue

                    if z > 0:
                        if watershed_grid[x, y, z - 1] > 0 and grid[x, y, z - 1] > 0:
                            watershed_grid[x, y, z] = 1
                            changed = True
                            continue
                    if z < watershed_grid.shape[2] - 1:
                        if watershed_grid[x, y, z + 1] > 0 and grid[x, y, z + 1] > 0:
                            watershed_grid[x, y, z] = 1
                            changed = True
                            continue

    grid = np.multiply(grid, watershed_grid)
    count_faces = count_surface_area(input, grid)
    print(f"Part 2: {count_faces}")


if __name__ == "__main__":
    part1()
    part2()
