#!/usr/bin/env python3

import re


def main():
    with open("input", 'r') as f:
        text = f.read()

    passport_strs = text.split("\n\n")
    key_value_pattern = re.compile(r"(?P<key>[^:]*):(?P<val>\S*)[\s]")

    required_fields = {
        "byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid",
        # "cid",
    }

    num_invalid = 0
    num_invalid_full = 0
    for pp_str in passport_strs:
        # print(pp_str)
        pp = dict(re.findall(key_value_pattern, pp_str + '\n'))
        missing_fields = required_fields - set(pp.keys())
        if len(missing_fields) > 0:
            num_invalid += 1
            num_invalid_full += 1
            print(f"invalid, missing {missing_fields}: {pp}")
        else:
            has_invalid_item = False
            for k, v in pp.items():
                if not is_valid(k, v):
                    num_invalid_full += 1
                    has_invalid_item = True
                    print(f"invalid {k=}, {v=}: {pp}")
                    break
            if not has_invalid_item:
                print(f"valid: {pp}")
    print(
        f"valid but unchecked: {len(passport_strs) - num_invalid}, "
        f"fully valid: {len(passport_strs) - num_invalid_full}, "
        f"invalid b/c missing fields: {num_invalid}, "
        f"total invalid: {num_invalid_full}"
    )


def is_valid(key, value):
    if key == "byr":
        return re.match(r"^\d{4}$", value) and 1920 <= int(value) <= 2002
    if key == "iyr":
        return re.match(r"^\d{4}$", value) and 2010 <= int(value) <= 2020
    if key == "eyr":
        return re.match(r"^\d{4}$", value) and 2020 <= int(value) <= 2030
    if key == "hgt":
        if m := re.match(r"^(?P<hgt>\d+)(?P<unit>in|cm)$", value):
            hgt = int(m.group("hgt"))
            unit = m.group("unit")
            return (
                (unit == "cm" and 150 <= hgt <= 193)
                or (unit == "in" and 59 <= hgt <= 76)
            )
        return False
    if key == "hcl":
        return re.match(r"^#[a-f0-9]{6}$", value) is not None
    if key == "ecl":
        return value in {"amb", "blu", "brn", "gry", "grn", "hzl", "oth"}
    if key == "pid":
        return re.match(r"^\d{9}$", value) is not None
    if key == "cid":
        return True
    raise ValueError(f"Unknown {key=}")


if __name__ == '__main__':
    main()
