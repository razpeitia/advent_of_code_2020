from functools import reduce

def chinese_remainder(n, a):
    sum = 0
    prod = reduce(lambda a, b: a*b, n)
    for n_i, a_i in zip(n, a):
        p = prod // n_i
        sum += a_i * mul_inv(p, n_i) * p
    return sum % prod

def mul_inv(a, b):
    b0 = b
    x0, x1 = 0, 1
    if b == 1: return 1
    while a > 1:
        q = a // b
        a, b = b, a%b
        x0, x1 = x1 - q * x0, x0
    if x1 < 0: x1 += b0
    return x1

def part2(ids):
    n = [ni for (_, ni) in ids]
    a = [(-ai) % ni for (ai, ni) in ids]
    ans = chinese_remainder(n, a)
    return ans

def part1(earliest, ids):
    m_id = None
    m_wait = None
    for id_ in ids:
        s = 0 if earliest % id_ == 0 else 1
        x = ((earliest // id_) + s) * id_
        wait = x - earliest
        if m_id is None:
            m_id = id_
            m_wait = wait
        elif m_wait > wait:
            m_id = id_
            m_wait = wait
    print(m_id * m_wait)



def main():
    with open("assets/input.txt") as f:
        line = next(f)
        earliest = int(line.strip())
        line = next(f).strip().split(",")
        ids = [int(i) for i in line if i != 'x']
        ids2 = [(i, int(id_)) for i, id_ in enumerate(line) if id_ != 'x']
    part1(earliest, ids)
    print("Tests part 2")
    assert 3417 == part2([(0, 17), (2, 13), (3, 19)])
    assert 754018 == part2([(0, 67), (1, 7), (2, 59), (3, 61)])
    assert 779210 == part2([(0, 67), (2, 7), (3, 59), (4, 61)])
    assert 1261476 == part2([(0, 67), (1, 7), (3, 59), (4, 61)])
    assert 1202161486 == part2([(0, 1789), (1, 37), (2, 47), (3, 1889)])
    t = part2(ids2)
    print(part2(ids2))


if __name__ == '__main__':
    main()
