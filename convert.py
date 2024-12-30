"""
Pixel-Clustering Image Format (PCF)




PCF is an image format that works by grouping pixels together in clusters that rely on simple additions and multiplications for compression, and grouping them by color.

It uses "layers" to store pixels, that is it completely disregards the pixels of the dominant color in the image, using it as a background. Then, it fills in the image using the other colors, thus allowing for a very efficient lossless compression.

However, it poorly compresses colors and their relation with pixels and as such, PCF works best with images that don't have a ton of colors and/or that have a high pixel/color ratio.

PCF supports transparency, and first reads colors in the RGBA format and then stores the colors in the hexadecimal format.




Usage:

python convert.py [filename] [-o] [-v] [-d]

positional arguments:
  filename

options:
  -o, --output    The .pcf output file
  -v, --verbose
  -d, --dev       Disable LZMA compression (makes the file human-readable)
"""
from multiprocessing import Pool, cpu_count
from os import getpid

from PIL import Image
import lzma
from statistics import mode
import argparse
from sys import getsizeof
from collections import defaultdict


Image.MAX_IMAGE_PIXELS = None


def compress(data):
    try:
        return lzma.compress(data.encode("utf-8"), check=lzma.CHECK_CRC64, preset=9)
    except:
        return lzma.compress(data, check=lzma.CHECK_CRC64, preset=9)


def process_sequence(sequence):
    nums = sequence.split("+")
    try:
        nums.remove("")
    except:
        pass
    nums.append("")
    new_sequence = ""
    current_num = ""
    current_num_count = 0
    for index, num in enumerate(nums):
        if index == 0:
            current_num = num
            current_num_count = 1
        else:
            if current_num == num:
                current_num_count += 1
            else:
                if current_num_count > 1:
                    new_sequence += "+" + current_num + "*" + str(current_num_count)
                else:
                    new_sequence += "+" + current_num
                current_num = num
                current_num_count = 1
    return new_sequence


def list_to_sequence(sequence):
    add_sequence = str(sequence[0])
    sequence_parts = [add_sequence]
    current_sum = sequence[0]
    for index, pixel in enumerate(sequence):
        if index != 0:
            diff = pixel - current_sum
            current_sum += diff
            sequence_parts.append(f"+{diff}")
    add_sequence = ''.join(sequence_parts)
    return process_sequence(add_sequence).lstrip("+")


def group_by_key(input_dict):
    compressed = {}
    for key in input_dict:
        if input_dict[key] not in compressed.keys():
            compressed[input_dict[key]] = []
        compressed[input_dict[key]].append(key)
    for key in list(compressed):
        if len(compressed[key]) == 1:
            compressed[key] = compressed[key][0]
        elif [x for x in compressed[key] if type(x) is str]:
            pass
        else:
            compressed[key] = list_to_sequence(compressed[key])
    for iteration in list(compressed):
        if type(compressed[iteration]) is list and "y" in str(compressed[iteration]):
            compressed["y" + str(iteration)] = list_to_sequence(
                [int(x.replace("y", "")) for x in compressed[iteration]])
            compressed.pop(iteration)
    return compressed


def shorthand_hex_color(color):
    compressed = ''
    color = color.lstrip("#")
    r = color[0:2]
    g = color[2:4]
    b = color[4:6]
    if r[0] == r[1]:
        compressed += r[0]
    else:
        compressed += r
    if g[0] == g[1]:
        compressed += g[0]
    else:
        compressed += g
    if b[0] == b[1]:
        compressed += b[0]
    else:
        compressed += b
    if len(compressed) not in [3, 6]:
        compressed = r + g + b

    if len(color) == 6:
        return f"#{compressed}"
    else:
        a = color[6:8]
        if a == "ff":
            a = ""
        return f"#{compressed}{a}"


# def calculate_potential_savings_by_vars(txt):
#     snake_pos = 0
#     hashes = []
#     seen = set()
#
#     print("Indexing all hashes...")
#     for i in txt:
#         for j in range(0, len(txt) - snake_pos):
#             substring = txt[snake_pos:snake_pos + j]
#             if substring not in seen:
#                 seen.add(substring)
#                 hashes.append((substring, snake_pos, snake_pos + j))
#         snake_pos += 1
#     print("Removed duplicate hashes")
#     occurrences = {}
#     hashes_length = len(hashes)
#
#     for i, (substring, start, end) in enumerate(hashes):
#         print(f"Processing hash {i + 1}/{hashes_length}", end="\r")
#         tot_count = txt.count(substring)
#
#         if tot_count > 1 and substring and len(substring) * tot_count > (
#                 len(f'v{i + 1}:{substring}') + len(f'v{i + 1}') * tot_count):
#             occurrences[substring] = (tot_count, start, end)
#
#     # sort occurences by length of key * occurences
#     occurrences = dict(sorted(occurrences.items(), key=lambda item: len(item[0]) * item[1][0], reverse=True))
#
#     # remove all keys that have overlapping start and end indexes
#     print("Removing overlapping keys...")
#     for key in list(occurrences.keys()):
#         print("Progress: ", round((list(occurrences.keys()).index(key) / len(occurrences.keys())) * 100, 2), "%",
#               end="\r")
#         start = occurrences[key][1]
#         end = occurrences[key][2]
#         for key2 in list(occurrences.keys()):
#             if key != key2:
#                 if occurrences[key2][1] <= start <= occurrences[key2][2]:
#                     del occurrences[key]
#                     break
#                 if occurrences[key2][1] <= end <= occurrences[key2][2]:
#                     del occurrences[key]
#                     break
#
#     space_savings = 0
#     i = 0
#     for key in occurrences:
#         i += 1
#         space_savings += len(key) * occurrences[key][0] - (
#                     len('v' + str(i) + ':' + str(occurrences[key])) + (len('v' + str(i)) * occurrences[key][0]))
#     print("Potential space savings: " + str(space_savings) + " bytes")


def list_to_chunks(array, chunk_size):
    for i in range(0, len(array), chunk_size):
        yield array[i:i + chunk_size]


def load_height_pixels(filename, width, height, verbose):
    # filename, width, height, verbose = args
    img = Image.open(filename).convert("RGBA")
    pixels = []
    for x in width:
        for y in range(height):
            px = img.getpixel((x, y))
            pixels.append((shorthand_hex_color('#%02x%02x%02x%02x' % px), x, y))
    if verbose:
        print("PROCESS N."+str(getpid())+" INDEXED CHUNK...")

    return pixels



def convert(filename, verbose, output, dev=None):
    if verbose:
        print("OPENING IMAGE AND CONVERTING TO RGBA...")
    img = Image.open(filename).convert("RGBA")
    width = img.size[0]
    height = img.size[1]


    if verbose:
        print("INDEXING PIXELS...")
    split_width = list_to_chunks(range(width), int(width / cpu_count()))
    if verbose:
        print("PROCESSING CHUNKS...")



    with Pool() as pool:
        pixels = [pixel for result in pool.starmap(load_height_pixels, [(filename, width, height, verbose) for width in split_width]) for pixel in result]


    if verbose:
        print("INDEXING COLORS...")

    px_colors = list(px[0] for px in pixels)
    colors = list(dict.fromkeys(px_colors))
    bg = mode(px_colors)
    colors.remove(bg)
    if verbose:
        print("GROUPING PIXELS...")

    grouped_pixels = defaultdict(list)
    for px in pixels:
        grouped_pixels[px[0]].append((px[1], px[2]))

    if verbose:
        print("COMPRESSING DATA...")
    outputf = str(width) + "-" + str(height)
    outputf += bg + "-*"
    i = 0
    for color in colors:
        i += 1
        # Get a list of tuples (x,y) containing every pixel of the color
        color_pixels = grouped_pixels[color]

        x_coords = {}
        y_coords = {}

        for pixel in color_pixels:
            # Groups pixels by their abscissa in the dict x_coords
            if pixel[0] not in x_coords.keys():
                x_coords[pixel[0]] = []
            x_coords[pixel[0]].append(pixel[1])

            # Groups pixels by their ordinate in the dict y_coords
            if "y" + str(pixel[1]) not in y_coords.keys():
                y_coords["y" + str(pixel[1])] = []
            y_coords["y" + str(pixel[1])].append(pixel[0])

        # If grouping pixels by their ordinate is more space efficient, then replace x_coords by y_coords
        if len(str(y_coords).replace(" ", "")) < len(str(x_coords).replace(" ", "")):
            x_coords = y_coords

        # If grouping pixels by their abscissa/ordinate is more space efficient, then proceed.
        if len(str(x_coords).replace(" ", "")) < len(str(color_pixels).replace(" ", "")):
            # Transform a sequence of integers (list of abscissas/ordinates, like [1,7,5,6,9,7]) into a string containing a mathematical operation (e.g 1+5+1+4+7+85+3+3)
            for coord in x_coords:
                # Get the list of integers in the sequence
                coord_pixels = x_coords[coord]
                # Get the first integer as a reference point
                add_sequence = str(coord_pixels[0])
                sequence_parts = [add_sequence]
                current_sum = coord_pixels[0]
                for index, pixel in enumerate(coord_pixels):
                    if index != 0:
                        diff = pixel - current_sum
                        current_sum += diff
                        sequence_parts.append(f"+{diff}")
                add_sequence = ''.join(sequence_parts)
                if len(add_sequence) < len(str(coord_pixels)):
                    x_coords[coord] = process_sequence(add_sequence).lstrip("+")

            pixels_out = str(group_by_key(x_coords)).replace(" ", "")
        # Else, store the pixels plainly
        else:
            if len(color_pixels) == 1:
                color_pixels = str(color_pixels[0]).replace("(", "").replace(")", "")
            else:
                color_pixels = str(color_pixels).replace("[", "").replace("]", "").replace("(", "").replace(")", "")
            pixels_out = str(color_pixels).replace(" ", "")

        outputf += (color + "-" + pixels_out.replace("'", ""))
        if verbose:
            print("PROCESSING... ", i, '/', len(colors), end="\r")
    outputf = outputf.strip("\n")

    # Not worth it with the current implementation
    # calculate_potential_savings_by_vars(outputf)

    output = open(output, "wb", buffering=131072)
    if len(str(outputf.encode("utf-8"))) > len(compress(outputf)) and not dev:
        compressed = compress(outputf)
        output.write(compressed)
        with open(filename, "rb") as f:
            print("\nSAVED TO " + str(output.name) + " -- " + str(
                round(getsizeof(compressed) * 100 / getsizeof(f.read()))) + "% OF ORIGINAL SIZE")
            return round(getsizeof(compressed) * 100 / getsizeof(f.read()))
    else:
        output.write(outputf.encode("utf-8"))
        with open(filename, "rb") as f:
            print("\nSAVED TO " + str(output.name) + " -- " + str(
                round(getsizeof(outputf.encode("utf-8")) * 100 / getsizeof(f.read()))) + "% OF ORIGINAL SIZE")
            return round(getsizeof(outputf.encode("utf-8")) * 100 / getsizeof(f.read()))


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Converts any image file to the pcf format')

    parser.add_argument("filename")
    parser.add_argument("-o", "--output", default="output.pcf", help="The .pcf output file")
    parser.add_argument("-v", "--verbose", action="store_true")
    parser.add_argument("-d", "--dev", action="store_true", help="")

    args = parser.parse_args()

    # yappi.set_clock_type("wall")
    # yappi.start()
    convert(args.filename, args.verbose, args.output, args.dev)
    # yappi.get_func_stats().print_all()
    # yappi.get_thread_stats().print_all()
