#!/usr/bin/env python3

import numpy as np

draws = list()
boards = [[]]
row = list()
row_count = 0
board = list()
with open("input/4.txt") as f:
    draws, *boards = f.read().rstrip().split('\n\n')

draws = [int(x) for x in draws.split(',')]


print(draws)
