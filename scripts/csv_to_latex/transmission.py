import csv
import sys

file = csv.reader(open(sys.argv[1], "r"))
columns = next(file)


max_lines = 5
for _ in range(0, max_lines):
    line = next(file)

    def value(key):
        return line[columns.index(key)]

    elements = list()

    elements.append(value('crf')) # crf
    elements.append(f"{value('latency')}ms") # latency
    elements.append(f"{value('max_frame_delay')}ms") # max_frame_delay
    elements.append(f"{float(value('encoded_size'))/1024:.2f} KiB") # encoded_size
    elements.append(f"\\textasciitilde{int(float(value('drop_rate')) * 100.0)}\%") # drop_rate
    elements.append(f"{float(value('frame_delay')):.2f}ms") # frame_delay
        
    formatted_line = " & ".join(elements) + " \\\\"
    print(formatted_line)