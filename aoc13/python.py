

def remainer(n, base, target):
    copy = n
    print(f"base minus target is {base - target}")
    while target != n % base and base - target != n % base:
    # while target != n % base:
        n += copy
        print(n)

    return n


remainer(13, 7, 1)
