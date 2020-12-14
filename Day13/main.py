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
    a = [abs(ni - ai) for (ai, ni) in ids]
    print(chinese_remainder(n, a))


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
        ids2.sort(key=lambda x: x[1])
    part1(earliest, ids)
    part2(ids2)


if __name__ == '__main__':
    main()