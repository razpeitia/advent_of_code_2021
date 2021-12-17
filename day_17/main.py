def parse_input(s):
    s = s.strip()
    prefix = "target area: "
    s = s[len(prefix):]
    x, y = s.split(", ")
    x = x[2:]
    y = y[2:]
    x = list(map(int, x.split("..")))
    y = list(map(int, y.split("..")))
    return (x, y)

def generate_path_y(vy, ty):
    path = []
    y = 0
    while y > min(ty):
        y += vy
        path.append(y)
        vy -= 1
    return path

def hits_targety(path, ty):
    a, b = ty
    for p in path:
        if a <= p <= b:
            return True
    return False

def solve(target):
    ty = target[1]

    highest_point = None
    for vy in range(1, max(map(abs, ty))):
        path = generate_path_y(vy, ty)
        if hits_targety(path, ty):
            max_point = max(path)
            if highest_point is None:
                highest_point = max_point
            elif max_point > highest_point:
                highest_point = max_point
    return highest_point


def generate_path(vx):
    path = []
    x = 0
    while vx:
        x += vx
        path.append(x)
        vx += -1 if vx > 0 else 1
    return path

def hits_targetx(path, t):
    a, b = t
    for p in path:
        if a <= p <= b:
            return True
    return False


def is_solution(path_x, path_y, tx, ty):
    if len(path_x) < len(path_y):
        diff = len(path_y) - len(path_x)
        padding = [path_x[-1]] * diff
        path_x = path_x + padding
    ax, bx = tx
    ay, by = ty
    for x, y in zip(path_x, path_y):
        if (ax <= x <= bx) and (ay <= y <= by):
            return True
    return False

def solve2(target):
    (tx, ty) = target

    possible_solutions_y = []
    limit = max(map(abs, ty))
    for vy in range(-limit, limit+1):
        path = generate_path_y(vy, ty)
        if hits_targety(path, ty):
            possible_solutions_y.append((vy, path))

    possible_solutions_x = []
    for vx in range(1, max(tx)+1):
        path = generate_path(vx)
        if hits_targetx(path, tx):
            possible_solutions_x.append((vx, path))
    
    solutions = set()
    for (vx, path_x) in possible_solutions_x:
        for (vy, path_y) in possible_solutions_y:
            if (vx, vy) in solutions:
                continue
            if is_solution(path_x, path_y, tx, ty):
                solutions.add((vx, vy))
    return len(solutions)


def main():
    with open("assets/input.txt") as f:
        s = f.read()
        target = parse_input(s)
    print(solve(target))
    print(solve2(target))

def test():
    s = "target area: x=20..30, y=-10..-5"
    target = parse_input(s)
    assert 45 == solve(target)
    assert 112 == solve2(target)

if __name__ == '__main__':
    test()
    main()
