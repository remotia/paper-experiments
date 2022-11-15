import csv
import sys

file = csv.reader(open(sys.argv[1], "r"))
first = True
for line in file:
    if first:
        first = False
        continue

    elements = list()

    elements.append(line[0]) # crf
    elements.append(f"{float(line[1])/1024:.2f} KiB") # encoded_size
    elements.append(f"{float(line[2]):.2f}ms") # frame_delay
    elements.append(f"\\textasciitilde{int(float(line[3]) * 100.0)}\%") # drop_rate
    elements.append(f"{float(line[4]):.2f}dB") # psnr_hvs
    elements.append(f"{float(line[5]):.2f}") # ssim
    elements.append(f"{float(line[6]):.2f}") # vmaf
    elements.append(f"{float(line[7]):.2f}") # score
        
    formatted_line = " & ".join(elements) + " \\\\"
    print(formatted_line)