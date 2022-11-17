import csv
import sys

file = csv.reader(open(sys.argv[1], "r"))
next(file)
max_lines = 5
for _ in range(0, max_lines):
    line = next(file)
    elements = list()

    elements.append(line[0]) # crf
    elements.append(f"{line[1]}ms") # latency
    elements.append(f"{line[2]}ms") # max_frame_delay
    elements.append(f"{float(line[3])/1024:.2f} KiB") # encoded_size
    elements.append(f"\\textasciitilde{int(float(line[4]) * 100.0)}\%") # drop_rate
    elements.append(f"{float(line[5]):.2f}ms") # frame_delay
    elements.append(f"{float(line[6]):.2f}") # score
        
    formatted_line = " & ".join(elements) + " \\\\"
    print(formatted_line)