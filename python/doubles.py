import re
import string
import random
import sys
import fun_motifs_plugin as fmp



def count_doubles(val):
    """Count repeated pair of chars in a string"""
    total = 0
    for c1, c2 in zip(val, val[1:]):
        if c1 == c2:
            total += 1
    return total


val = ''.join(random.choice(string.ascii_letters) for i in range(1000000))


def count_doubles_regex(val):
    return len(double_re.findall(val))

double_re = re.compile(r'(?=(.)\1)')

def test_pure_python(benchmark):
    benchmark(count_doubles, val)


def test_regex(benchmark):
    benchmark(count_doubles_regex, val)


def test_rust(benchmark, val):  # <-- Benchmark the Rust version
    benchmark(fmp.count_doubles, val)

