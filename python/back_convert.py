"""
Pixel-Clustering Image Format (PCF)




PCF is an image format that works by grouping pixels together in clusters that rely on simple additions and multiplications for compression, and grouping them by color.

It uses "layers" to store pixels, that is it completely disregards the pixels of the dominant color in the image, using it as a background. Then, it fills in the image using the other colors, thus allowing for a very efficient lossless compression.

However, it poorly compresses colors and their relation with pixels and as such, PCF works best with images that don't have a ton of colors and/or that have a high pixel/color ratio.

PCF supports transparency, and first reads colors in the RGBA format and then stores the colors in the hexadecimal format.




Usage:

python back_convert.py [filename]

positional arguments:
  filename
"""


from PIL import Image, ImageColor
import lzma
import re
from ast import literal_eval
import argparse


def back_quote(text):
    # very inefficient imo but it works -> match either 'yXXXXXX...'(3) or 'X+X*X+Y....'(2) or 'yXX+XX*X...'(1)
    pattern = r'(y([-+]?[0-9]*\.?[0-9]+[\+\*])+([-+]?[0-9]*\.?[0-9]+)|([-+]?[0-9]*\.?[0-9]+[\+\*])+([-+]?[0-9]*\.?[0-9]+)|y\d+)'

    def add_quotes(match):
        return f"'{match.group(0)}'"

    return re.sub(pattern, add_quotes, text)

def backprocess_sequence(new_sequence):
    if type(new_sequence) is int:
        return new_sequence
    parts = new_sequence.split("+")
    original_sequence = []

    for part in parts:
        if "*" in part:
            num, count = part.split("*")
            original_sequence.extend([num] * int(count))
        elif part:
            original_sequence.append(part)
    return "+".join(original_sequence)

def sequence_to_list(sequence):
    out_list = []
    sequence = backprocess_sequence(sequence)
    adds = sequence.split("+")
    base = int(adds[0])
    out_list.append(base)
    for index, diff in enumerate(adds):
        if index != 0:
            diff = int(diff)
            if diff >= 0:
                base += diff
            else:
                base -= abs(diff)
            out_list.append(base)
    return out_list


def degroup_by_key(input_dict):
    decompressed = {}
    for key in input_dict:
        if type(key) is tuple:
            return input_dict
        elif type(input_dict[key]) is int:
            decompressed[input_dict[key]] = key
        elif "y" in str(key) and type(key) is not tuple:
            for param in sequence_to_list(input_dict[key]):
                decompressed["y" + str(param)] = key.replace("y", "")
        elif '+' in input_dict[key] or '*' in input_dict[key]:
            for param in sequence_to_list(input_dict[key]):
                decompressed[param] = key
        elif type(input_dict[key]) is list:
            for param in input_dict[key]:
                decompressed[param] = key
        else:
            decompressed[input_dict[key]] = key
    return decompressed


def locate_indexes(string, character):
    indexes = []
    for i, char in enumerate(string):
        if char == character:
            indexes.append(i)
    return indexes



def back_convert(file_name):
    img_pixels = []
    bg = ""

    with open(file_name, "rb") as f:
        file_raw = f.read()
        try:
            content = lzma.decompress(file_raw).decode("utf-8")
        except:
            content = file_raw.decode("utf-8")
    content = back_quote(content).split("#")
    width = int(content[0].split("-")[0])
    height = int(content[0].split("-")[1])
    content.pop(0)
    for group in content:
        split = group.split("-")
        color = ImageColor.getcolor("#" + split[0], "RGBA")
        if split[1] == "*":
            bg = color
        else:
            pixels = literal_eval(split[1])
            if type(eval("("+split[1]+")")) is tuple and split[1].count(",") == 1:
                img_pixels.append((color, pixels[0], pixels[1]))
                continue
            if split[1].count(",") > 1 and "{" not in split[1]:
                new_split = []
                add_tuple = ""
                for x in split[1].split(","):
                    if add_tuple == "":
                        add_tuple = str(x)
                    else:
                        new_split.append((int(add_tuple), int(x)))
                        add_tuple = ""
                pixels = eval(str(new_split))

            pixels = degroup_by_key(pixels)
            for pixel_group in pixels:
                if type(pixel_group) is not tuple:
                    pixels[pixel_group] = backprocess_sequence(pixels[pixel_group])
                    if type(pixels[pixel_group]) is int:
                        adds = [pixels[pixel_group]]
                    else:
                        adds = pixels[pixel_group].split("+")
                    if type(pixel_group) is str:
                        base = int(adds[0])
                        pixel_group = int(pixel_group.replace("y",""))
                        img_pixels.append((color, base, int(pixel_group)))
                        for index, diff in enumerate(adds):
                            if index != 0:
                                diff = int(diff)
                                if diff >= 0:
                                    base += diff
                                else:
                                    base -= abs(diff)
                                img_pixels.append((color, base, pixel_group))
                    else:
                        base = int(adds[0])
                        img_pixels.append((color, int(pixel_group), base))
                        for index, diff in enumerate(adds):
                            if index != 0:
                                diff = int(diff)
                                if diff >= 0:
                                    base += diff
                                else:
                                    base -= abs(diff)
                                img_pixels.append((color, pixel_group, base))

                else:
                    img_pixels.append((color, pixel_group[0], pixel_group[1]))

    im = Image.new(mode="RGBA", size=(width, height), color=bg)
    for px in img_pixels:
        # print((px[1], px[2]))
        im.putpixel((px[1], px[2]), px[0])
    im.show()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description='Converts any image file from the PCF format to the PNG format')
    parser.add_argument("filename")

    args = parser.parse_args()

    back_convert(args.filename)
