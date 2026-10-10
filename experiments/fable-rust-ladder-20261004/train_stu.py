#!/usr/bin/env python3
"""DEPRECATED shim: train_stu.py was folded into train.py, whose defaults are train_stu.py's (same arguments, same
arithmetic; byte-identical exports on the MLX CPU device). This forwards every argument to train.py unchanged."""
import sys
import train
if __name__ == '__main__':
    print('train_stu.py is deprecated: forwarding to train.py with the same arguments', file=sys.stderr)
    train.main(sys.argv[1:])
