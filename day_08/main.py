s = """
be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
""".strip()

def part1(lines):
    ans = 0
    for (_a, b) in lines:
        ans += sum([1 for i in b if len(i) in (2, 4, 3, 7)])
    return ans

def solve(a, b):
    mapping = {i: None for i in range(10)}
    # Get 1
    mapping[1] = set([x for x in a if len(x) == 2][0])
    # Get 4
    mapping[4] = set([x for x in a if len(x) == 4][0])
    # Get 7
    mapping[7] = set([x for x in a if len(x) == 3][0])
    # Get 8
    mapping[8] = set([x for x in a if len(x) == 7][0])
    # 1 in len(x) == 5 -> 3
    mapping[3] = set([x for x in a if len(x) == 5 and mapping[1].issubset(set(x))][0])
    
    # 4 in len(x) == 6 -> 9
    mapping[9] = set([x for x in a if len(x) == 6 and mapping[4].issubset(set(x))][0])
    # 7 not in len(x) == 6 -> 6
    mapping[6] = set([x for x in a if len(x) == 6 and not mapping[7].issubset(set(x))][0])
    # rest -> 0
    mapping[0] = set([x for x in a if len(x) == 6 and set(x) != mapping[6] and set(x) != mapping[9]][0])
    
    # 6 in len(x) == 5 -> 5
    mapping[5] = set([x for x in a if len(x) == 5 and set(x).issubset(mapping[6])][0])

    # rest -> 2
    mapping[2] = set([x for x in a if len(x) == 5 and set(x) != mapping[3] and set(x) != mapping[5]][0])

    def lookup(v1):
        v1 = set(v1)
        for k, v in mapping.items():
            if v == v1:
                return str(k)
    return int(''.join([lookup(x) for x in b]))

def part2(lines):
    ans = 0
    for (a, b) in lines:
        ans += solve(a, b)
    return ans

def main():
    lines = s.strip().split("\n")
    lines = [line.split(" | ") for line in lines]
    lines = [(a.split(), b.split()) for (a, b) in lines]
    ans = part1(lines)
    print(f"test {ans}")
    assert part1(lines) == 26
    assert part2(lines) == 61229

    with open("assets/input.txt") as f:
        lines = f.read().strip().split("\n")
        lines = [line.split("|") for line in lines]
        lines = [(a.split(), b.split()) for (a, b) in lines]
        print(part1(lines))
        print(part2(lines))

if __name__ == '__main__':
    main()
