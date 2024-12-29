from os import listdir, path
import convert

originals = []
transformed = []

folder = "../test-images/"

files = listdir(folder)
files.remove("credits.txt")
try:
    files.remove(".DS_Store")
except:
    pass

tot_sum = 0
for index, x in enumerate(files):
    originals.append(path.getsize(folder + x))
    convert.convert(folder + x, True, "output.pcf")
    transformed.append(path.getsize("output.pcf"))
    tot_sum += int(path.getsize("output.pcf") * 100 / path.getsize(folder + x))
    print(index+1, "/", len(files))
    print(transformed)

print("---\n", int(100 - (tot_sum / len(files))), "% SMALLER ON AVERAGE\n---")

print(originals)
print(transformed)
print(files)