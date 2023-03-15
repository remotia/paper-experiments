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

    elements.append(value('crf'))
    elements.append(f"{value('latency')}ms")
    elements.append(f"{value('max_frame_delay')}ms")

    elements.append(f"{float(value('psnr_hvs')):.2f} dB")
    elements.append(f"{float(value('float_ssim')):.2f}")
    elements.append(f"{float(value('vmaf')):.2f}")
    elements.append(f"{float(value('score')):.2f}")
        
    formatted_line = " & ".join(elements) + " \\\\"
    print(formatted_line)