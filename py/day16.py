import re
from dataclasses import dataclass, replace
from typing import Self, cast

import networkx as nx

from py.lib import FromStr, read_input_lines_as

NODE_RE = re.compile(
    r"Valve (?P<name>[A-Z]+) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?P<tunnels>[A-Z ,]+)"  # noqa
)


@dataclass
class GraphNode(FromStr):
    name: str
    flow: int
    tunnels: list[str]

    def __hash__(self) -> int:
        return hash(self.name)

    @classmethod
    def from_str(cls, s: str) -> Self:
        match = NODE_RE.search(s)
        tunnels = list(match.group("tunnels").split(", "))
        return GraphNode(match.group("name"), int(match.group("flow")), tunnels)


def construct_graph() -> tuple[list[GraphNode], nx.Graph]:
    inputs = read_input_lines_as(16, GraphNode)
    graph = nx.Graph()
    for n in inputs:
        graph.add_node(n.name)

    for n in inputs:
        for tunnel in n.tunnels:
            graph.add_edge(n.name, tunnel)

    return inputs, graph


def search(
    loc: str,
    visited: list[str],
    score_so_far: int,
    time_so_far: int,
    to_visit: set[GraphNode],
    paths: dict,
) -> (list[str], int):
    scores = [(visited, score_so_far)]
    for item in to_visit:
        score = score_so_far
        path = paths[loc][item.name]
        remaining_time = 30 - time_so_far
        assert remaining_time >= 0
        if remaining_time == 0:
            continue
        walk_time = len(path) - 1
        open_time = 1

        for_next_valve = remaining_time - walk_time - open_time
        if for_next_valve > 0:
            score += item.flow * for_next_valve
            if to_visit:
                scores.append(
                    search(
                        item.name,
                        visited + [item.name],
                        score,
                        time_so_far + walk_time + open_time,
                        to_visit - {item},
                        paths,
                    ),
                )
            else:
                scores.append((visited + [item.name], score))

    return max(scores, key=lambda pair: pair[1])


@dataclass
class State:
    my_loc: str
    ele_loc: str
    visited: list[str]
    score_so_far: int
    my_time_so_far: int
    ele_time_so_far: int
    to_visit: set[str]


def search_pair(
    state: State, paths: dict, nodes: dict[str, GraphNode]
) -> (list[str], int):
    scores = [(state.visited, state.score_so_far)]
    my_remaining_time = 26 - state.my_time_so_far
    ele_remaining_time = 26 - state.ele_time_so_far
    my_choices = cast(list[str | None], [None]) + [
        path
        for path in state.to_visit
        if my_remaining_time > len(paths[state.my_loc][path])
    ]
    ele_choices = [
        path
        for path in state.to_visit
        if ele_remaining_time > len(paths[state.ele_loc][path])
    ]

    if my_remaining_time > 0 or ele_remaining_time > 0:
        for item in my_choices:
            for ele_item in ele_choices:
                if item == ele_item:
                    continue

                open_time = 1
                next_state = replace(
                    state,
                    visited=[v for v in state.visited],
                    to_visit=set(tv for tv in state.to_visit),
                )

                if my_remaining_time and item:
                    my_path = paths[state.my_loc][item]
                    my_walk_time = len(my_path) - 1
                    my_for_next_valve = my_remaining_time - my_walk_time - open_time
                    if my_for_next_valve > 0:
                        next_state.score_so_far += nodes[item].flow * my_for_next_valve
                        next_state.visited += [item]
                        next_state.to_visit -= {item}
                        next_state.my_time_so_far += my_walk_time + open_time
                        next_state.my_loc = item

                if ele_remaining_time:
                    ele_path = paths[state.ele_loc][ele_item]
                    ele_walk_time = len(ele_path) - 1
                    ele_for_next_valve = ele_remaining_time - ele_walk_time - open_time
                    if ele_for_next_valve > 0:
                        next_state.score_so_far += (
                            nodes[ele_item].flow * ele_for_next_valve
                        )

                        next_state.visited += [ele_item]
                        next_state.to_visit -= {ele_item}
                        next_state.ele_time_so_far += ele_walk_time + open_time
                        next_state.ele_loc = ele_item

                if next_state.to_visit and next_state.visited != state.visited:
                    scores.append(search_pair(next_state, paths, nodes))
                else:
                    scores.append((next_state.visited, next_state.score_so_far))

    return max(scores, key=lambda pair: pair[1])


def part1():
    inputs, graph = construct_graph()
    nodes_with_flow = [i for i in inputs if i.flow]
    paths = dict(nx.all_pairs_shortest_path(graph))
    best = search(
        "AA",
        ["AA"],
        0,
        0,
        set(nodes_with_flow),
        paths,
    )

    print(f"Part 1: {best}")


def part2():
    inputs, graph = construct_graph()
    nodes_with_flow = [i for i in inputs if i.flow]
    paths = dict(nx.all_pairs_shortest_path(graph))
    best = search_pair(
        State(
            my_loc="AA",
            ele_loc="AA",
            visited=["AA"],
            score_so_far=0,
            my_time_so_far=0,
            ele_time_so_far=0,
            to_visit=set(n.name for n in nodes_with_flow),
        ),
        paths,
        {node.name: node for node in inputs},
    )

    print(f"Part 2: {best}")


if __name__ == "__main__":
    part1()
    part2()
