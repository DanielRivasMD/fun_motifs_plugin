from datetime import datetime
import argparse

def parse_args():
    parser = argparse.ArgumentParser(description='stop time for counting lines')
    parser.add_argument('--file', default="")
    args, unknown = parser.parse_known_args()
    return args

def count_lines(infile):
    with open(infile, 'r') as f:
        count = 0
        for line in f:
            count += 1
    return count


if __name__ == '__main__':
    args = parse_args()
    start_time = datetime.now()
    print(start_time)
    x = count_lines(args.file)
    print('Lines:', x)
    end_time = datetime.now()
    print(f'Duration: {end_time - start_time}')
