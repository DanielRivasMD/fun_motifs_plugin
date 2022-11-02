from datetime import datetime


def count_lines(infile):
    with open(infile, 'r') as f:
        count = 0
        for line in f:
            count += 1
    return count


if __name__ == '__main__':
    start_time = datetime.now()
    print(start_time)
    file = 'D:\CRC-SW.Ensemble.1063_DNBSEQ.20210706.lite.maf'
    x = count_lines(file)
    print(x)
    end_time = datetime.now()
    print(f'Duration: {end_time - start_time}')
