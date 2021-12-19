import json
import math
from functools import reduce
from typing import List, Tuple, Union

def explode(pairs):
    pairs = pairs.copy()
    i = 0
    while i < (len(pairs) - 1):
        (a, la), (b, lb) = pairs[i], pairs[i+1]
        if la == lb and la >= 5:
            del pairs[i]
            del pairs[i]
            pairs.insert(i, (0, la-1))
            if (i - 1) >= 0:
                (x, l) = pairs[i-1]
                pairs[i - 1] = (x + a, l)
            if (i + 1) < len(pairs):
                (x, l) = pairs[i+1]
                pairs[i+1] = (x + b, l)
            break
        i += 1
    return pairs

def split(pairs):
    pairs = pairs.copy()
    for i in range(len(pairs)):
        (v, l) = pairs[i]
        if v >= 10:
            return pairs[:i] + [(int(math.floor(v / 2)), l+1), (int(math.ceil(v / 2)), l+1)] + pairs[i+1:]
    return pairs

def reduce_pair(pair):
    last_pair = pair
    while True:
        last_pair = explode(pair)
        if last_pair != pair:
            pair = last_pair
            continue
        last_pair = split(pair)
        if last_pair != pair:
            pair = last_pair
            continue
        break
    return last_pair


def add_pairs(x, y):
    z = x + y
    z = [(v, l+1) for (v, l) in z]
    return reduce_pair(z)

def sum_pairs(pairs):
    return reduce(lambda x, y: add_pairs(x, y), pairs)

def magnitude(pairs : List[Tuple[int, int]]) -> int:
    pairs = pairs.copy()
    while True:
        i = 0
        while i < (len(pairs) - 1):
            (a, la), (b, lb) = pairs[i], pairs[i+1]
            if la == lb:
                del pairs[i]
                del pairs[i]
                pairs.insert(i, (3*a + 2*b, la-1))
                break
            i += 1
        if len(pairs) == 1:
            return pairs[0][0]

def part2(pairs):
    n = len(pairs)
    max = None
    for i in range(0, n):
        for j in range(0, n):
            if i == j: continue
            m = magnitude(sum_pairs([pairs[i], pairs[j]]))
            if max is None:
                max = m
            elif m > max:
                max = m
    return max
            

def part1(pairs):
    return magnitude(sum_pairs(pairs))

def transform(data : Union[list, int], level : int, acc : List[Tuple[int, int]]):
    if isinstance(data, int):
        acc.append((data, level))
        return
    transform(data[0], level+1, acc)
    transform(data[1], level+1, acc)

def parse_line(line : str) -> List[Tuple[int, int]]:
    data = json.loads(line)
    acc = []
    transform(data, 0, acc)
    return acc

def parse_input(s : str) -> List[List[Tuple[int, int]]]:
    return [parse_line(line) for line in s.strip().splitlines()]

def main():
    with open("assets/input.txt") as f:
        pairs = parse_input(f.read())
    print(part1(pairs))
    print(part2(pairs))

def test():
    test_parse_input()
    test_explode()
    test_split()
    test_sum_pairs()
    test_magnitude()
    test_part2()

def test_parse_input():
    count = 1
    def test_(input, expected):
        nonlocal count
        ans = parse_line(input)
        assert expected == ans, f"Bad test {count}\nexpecting {expected}\nfound     {ans}"
        count += 1
    test_("[[[[[9,8],1],2],3],4]", [(9,5),(8,5),(1,4),(2,3),(3,2),(4,1)])
    test_("[7,[6,[5,[4,[3,2]]]]]", [(7,1),(6,2),(5,3),(4,4),(3,5),(2,5)])
    test_("[[6,[5,[4,[3,2]]]],1]", [(6,2),(5,3),(4,4),(3,5),(2,5),(1,1)])
    test_("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]", [(3,2),(2,3),(1,4),(7,5),(3,5),(6,2),(5,3),(4,4),(3,5),(2,5)])
    test_("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]", [(3,2),(2,3),(8,4),(0,4),(9,2),(5,3),(4,4),(3,5),(2,5)])

    print("All test passed for parse input")

def test_explode():
    count = 1
    def test_(input, expected):
        nonlocal count
        ans = explode(input)
        assert expected == ans, f"Bad test {count}\nexpecting {expected}\nfound     {ans}"
        count += 1
    
    test_([(9,5),(8,5),(1,4),(2,3),(3,2),(4,1)],
          [(0,4),(9,4),(2,3),(3,2),(4,1)])
    test_([(7,1),(6,2),(5,3),(4,4),(3,5),(2,5)],
          [(7,1),(6,2),(5,3),(7,4),(0,4)])
    test_([(6,2),(5,3),(4,4),(3,5),(2,5),(1,1)],
          [(6,2),(5,3),(7,4),(0,4),(3,1)])  
    test_([(3,2),(2,3),(1,4),(7,5),(3,5),(6,2),(5,3),(4,4),(3,5),(2,5)],
          [(3,2),(2,3),(8,4),(0,4),(9,2),(5,3),(4,4),(3,5),(2,5)])
    test_([(3,2),(2,3),(8,4),(0,4),(9,2),(5,3),(4,4),(3,5),(2,5)],
          [(3,2),(2,3),(8,4),(0,4),(9,2),(5,3),(7,4),(0,4)])
    print("All test passed for explode")

def test_split():
    count = 1
    def test_(input, expected):
        nonlocal count
        ans = split(parse_line(input))
        assert parse_line(expected) == ans, f"Bad test {count}\nexpecting {expected}\nfound     {ans}"
        count += 1
    
    test_("[[[[0,7],4],[15,[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")
    test_("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]", "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]")
    print("All test passed for split")

def test_sum_pairs():
    count = 1
    def test_(input, expected):
        nonlocal count
        ans = sum_pairs(parse_input(input))
        assert parse_line(expected) == ans, f"Bad test {count}\nexpecting {parse_line(expected)}\nfound     {ans}"
        count += 1
    test_("[[[[4,3],4],4],[7,[[8,4],9]]]\n[1,1]", "[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")
    test_("[1,1]\n[2,2]\n[3,3]\n[4,4]", "[[[[1,1],[2,2]],[3,3]],[4,4]]")
    test_("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]", "[[[[3,0],[5,3]],[4,4]],[5,5]]")
    test_("[1,1]\n[2,2]\n[3,3]\n[4,4]\n[5,5]\n[6,6]", "[[[[5,0],[7,4]],[5,5]],[6,6]]")
    test_("[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]\n[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]", "[[[[4,0],[5,4]],[[7,7],[6,0]]],[[8,[7,7]],[[7,9],[5,0]]]]")
    s = """[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]"""
    test_(s, "[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")
    s = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""
    test_(s, "[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]")
    print("All test passed for sum pairs")

def test_magnitude():
    count = 1
    def test_(input, expected):
        nonlocal count
        ans = magnitude(parse_line(input))
        assert expected == ans, f"Bad test {count}\nexpecting {expected}\nfound     {ans}"
        count += 1
    test_("[[1,2],[[3,4],5]]", 143)
    test_("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]", 1384)
    test_("[[[[1,1],[2,2]],[3,3]],[4,4]]", 445)
    test_("[[[[3,0],[5,3]],[4,4]],[5,5]]", 791)
    test_("[[[[5,0],[7,4]],[5,5]],[6,6]]", 1137)
    test_("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]", 3488)
    test_("[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]", 4140)
    
    print("All test passed for magnitude")


def test_part2():
    count = 1
    def test_(input, expected):
        nonlocal count
        ans = part2(parse_input(input))
        assert expected == ans, f"Bad test {count}\nexpecting {expected}\nfound     {ans}"
        count += 1
    
    s = """[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"""
    test_(s, 3993)
    
    print("All test passed for part2")

if __name__ == "__main__":
    test()
    main()