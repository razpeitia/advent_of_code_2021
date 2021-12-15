from math import ceil
import time

def parse(line):
    arr = line.split(" -> ")
    k = arr[0]
    v = arr[1]
    return (k, [k[0] + v, v + k[1]])

def solve(template, instructions, steps):
    d = {}
    for (a, b) in zip(template[0:-1], template[1:]):
        pair = a + b
        if pair not in d:
            d[pair] = 0
        d[pair] += 1
    for step in range(steps):
        dt = {}
        for k in d:
            for pair in instructions[k]:
                if pair not in dt:
                    dt[pair] = 0
                dt[pair] += d[k]
        d = dt
    letters = {}
    for (pair, v) in d.items():
        for c in pair:
            if c not in letters:
                letters[c] = 0
            letters[c] += v
    max_v = max(letters.values()) // 2
    min_v = min(letters.values()) // 2
    ans = max_v - min_v + 1
    return ans

def parse_input(s):
    arr = s.split("\n\n")
    template = arr[0]
    instructions = dict(parse(line) for line in arr[1].split("\n"))
    return template, instructions

def main():
    with open("assets/input.txt") as f:
        s = f.read().strip()
        template, instructions = parse_input(s)
    
    start = time.time()
    ans = solve(template, instructions, 10)
    elapsed = time.time() - start
    print(ans)
    print(f"Part 1 took {elapsed} seconds")

    start = time.time()
    ans = solve(template, instructions, 40)
    elapsed = time.time() - start
    print(ans)
    print(f"Part 2 took {elapsed} seconds")

def test():
    s = """
NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C""".strip()
    print("=" * 80)
    print("TEST")
    template, instructions = parse_input(s)
    assert 1588 == solve(template, instructions, 10), "Basic test part 1 failed"
    assert 2188189693529 == solve(template, instructions, 40), "Basic test part 2 failed"
    print("=" * 80)


if __name__ == '__main__':
    test()
    main()