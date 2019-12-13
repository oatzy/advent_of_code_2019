"""
The idea here is that the axes are independent
i.e. the x positions don't dependend on the y conditions, etc.

So we try to find a loop for each of the axes,
assuming these will be much shorter than the loop for the whole state.

Then to get the loop for the whole state, we want to find the point at which the loops coincide.
This first occurs at the lowest common multiple of the individual loops.
"""
from __future__ import print_function

import re
from functools import reduce  # py3


def gcd(a, b):
    while b != 0:
        a, b = b, a % b
    return a


def lcm(a, b):
    return a * b / gcd(a, b)


def find_loop(positions):
    seen = set()
    velocities = (0, 0, 0, 0)

    t = 0

    while (positions, velocities) not in seen:
        seen.add((positions, velocities))

        # py3 doesn't have cmp, google suggests (x > y) - (x < y) instead
        # (we actually want -cmp(x, y) here)
        gravity = [sum((x < y) - (x > y) for y in positions) for x in positions]

        # need to be tuples (immutable/hashable) to add them to sets
        velocities = tuple(a + b for a, b in zip(gravity, velocities))
        positions = tuple(a + b for a, b in zip(velocities, positions))

        t += 1

    return t


def main():
    with open('/home/chris/advent_of_code/2019/inputs/day12.txt', 'r') as f:
        moons = re.findall("<x=(\\-?\\d+), y=(\\-?\\d+), z=(\\-?\\d+)>", f.read())

    positions = zip(*moons)  # group coords by axis
    loops = [find_loop(tuple(map(int, p))) for p in positions]

    # lcm(a, b, c) = lcm(lcm(a, b), c) = reduce(lcm, [a, b, c])
    print(int(reduce(lcm, loops)))


if __name__ == '__main__':
    main()
