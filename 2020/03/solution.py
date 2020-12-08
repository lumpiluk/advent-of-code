#!/usr/bin/env python3


def main():
    with open("input", 'r') as f:
        tree_map = f.readlines()

    slopes = [
        (1, 1),
        (3, 1),
        (5, 1),
        (7, 1),
        (1, 2),
    ]
    tree_product = 1
    for right, down in slopes:
        num_trees = get_num_trees(tree_map, right, down)
        print(f"{right=}, {down=}, {num_trees=}")
        tree_product *= num_trees
    print(f"{tree_product=}")


def get_num_trees(tree_map, right, down):
    width = len(tree_map[0].strip())
    height = len(tree_map)

    num_trees = 0
    x = 0
    for y in range(0, height, down):
        num_trees += (
            1 if tree_map[y][x % width] == '#' else 0
        )
        # print(f"{tree_map[y]}, {y=}, {x=} ({x % width}), {num_trees=}")
        x += right
    return num_trees


if __name__ == '__main__':
    main()
