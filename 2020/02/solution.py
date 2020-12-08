#!/usr/bin/env python3

import re


def main():
    with open("input", 'r') as f:
        text = f.readlines()

    p = re.compile(
        r"(?P<l_from>\d+)-(?P<l_to>\d+) (?P<letter>[a-z]): (?P<pw>.*)"
    )
    num_pws = 0
    num_corrupt_rule0 = 0
    num_corrupt_rule1 = 0
    for line in text:
        if m := p.match(line):
            l_from = int(m.group('l_from'))
            l_to = int(m.group('l_to'))
            letter = m.group('letter')
            pw = m.group('pw')
            letter_count = pw.count(letter)
            print(f"{l_from=}, {l_to=}, {letter=}, {pw=}, {letter_count=}")
            if letter_count < l_from or letter_count > l_to:
                num_corrupt_rule0 += 1
            times_letter_is_at_either_pos = 0
            times_letter_is_at_either_pos += pw[l_from-1] == letter
            times_letter_is_at_either_pos += pw[l_to-1] == letter
            if times_letter_is_at_either_pos != 1:
                num_corrupt_rule1 += 1
            num_pws += 1
        else:
            print('nomatch')
    print(
        f"{num_pws=}, {num_corrupt_rule0=}, {num_corrupt_rule1=}, "
        f"num_valid_rule0={num_pws-num_corrupt_rule0}, "
        f"num_valid_rule1={num_pws-num_corrupt_rule1}"
    )


if __name__ == '__main__':
    main()
