# Pixel-Clustering Image Format (.PCF)
### About
PCF is an image format that works by grouping pixels together in clusters that rely on simple additions and multiplications for compression, and grouping them by color.

### How
It uses "layers" to store pixels, that is it completely disregards the pixels of the dominant color in the image, using it as a "background". Then, it fills in the image using the other colors, thus allowing for a very efficient lossless compression. 

1. For example, imagine a black 1*5 image. You could theoretically represent all the pixels inside of this image as a list of tuples (x,y) : `[(0,0), (0,1), (0,2), (0,3), (0,4)]`. But then, this isn't very efficient: we just wrote the same abscissa five times in a row. What if we could save space by factoring them by their abscissa (or ordinate) ? That's exactly what LPI does, by representing the same pixels like this: `0:[0,1,2,3,4]`. Now that's more compact. 
2. But what if our image is bigger ? For example, let's say our image is 5x5:\
`
[(0,0), (0,1), (0,2), (0,3), (0,4),
(1,0), (1,1), (1,2), (1,3), (1,4),
(2,0), (2,1), (2,2), (2,3), (2,4),
(3,0), (3,1), (3,2), (3,3), (3,4),
(4,0), (4,1), (4,2), (4,3), (4,4)]
`\
In that case, our previous method gives us this:\
`0:[0,1,2,3,4], 1:[0,1,2,3,4], 2:[0,1,2,3,4], 3:[0,1,2[README.md](README.md),3,4], 4:[0,1,2,3,4]`\
Which isn't great space-wise. However, there's a way to make it more compact. We have the same list five times in row, thus we basically re-use the previous method, which returns this:
`[0,1,2,3,4]:[0,1,2,3,4]`\
That's better!
3. But then, same as last time, this becomes very inefficient for bigger images. Let's consider a black image of size (x,x) with x>10. With our previous method, we get this:
`[0,1,2,3,4,...,x-1]:[0,1,2,3,4,...,x-1]`. The bigger x gets, the more wasteful this is. With x=10000, the end of each list would look like this:`..., 9998, 9999`.\
The answer lies in sums. Instead of writing all those big numbers, we can write, "0+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1+1......", each addition representing a number. Of course, this works even if the numbers are not evenly spaced apart (e.g. you could have "0+1+10+2+4+5+1+99").
4. However, that's not super compact. Why write the same addition 9999 times in a row ? This is the last step. LPI simply writes "0+1\*9999". This also works with different numbers, like: "0+1\*932+2\*34+1+7+4+56+3\*23"

This works great, but less so for images which contain thousands and thousands of colors and not many pixels per color, as PCF's method is given less "space" to work properly.

### More info
On images where it works best, PCF offers a great compression level, sometimes being as much as 99% smaller than the original PNG image. However, it poorly compresses colors and their relation with pixels and as such, works best with images that don't have a ton of colors and/or that have a high pixel/color ratio. Examples below (the images in the .PCF format are 47% to 62% smaller than their heavily optimized PNG original counterparts).

PCF supports transparency, and first reads colors in the RGBA format and then stores them in the hexadecimal format.

## Installation

```bash
git clone https://github.com/just-a-mango//pixel-clustering-image-format
```

## Usage

```sh
python convert.py [filename] [-o] [-v] [-d]

python back_convert.py [filename]

positional arguments:
  filename

options:
  -o, --output    The .pcf output file
  -v, --verbose
  -d, --dev       Disable LZMA compression (makes the file readable)
```

## Comparisons with PNG
![Plot 1](plots/fig1.png)
![Plot 2](plots/fig2.png)
![Plot 3](plots/fig3.png)
![Plot 4](plots/fig4.png)
![Plot 5](plots/fig5.png)
![Plot 6](plots/fig6.png)
![Plot 7](plots/fig7.png)

## Examples
### Blue image
#### PNG:
![PNG Blue image](test-images/blue.png)
484 bytes
#### LPI:
```
1920-1080#0000fe-*
```
18 bytes (~27 times smaller)

### Blue ball pixel art
#### PNG:
![Blue ball pixel art](test-images/blue_ball_pixel_art.png)
70 kB
#### PCF:
804 bytes (~86 times smaller)
<details>
<summary>Uncompressed</summary>

```
1408-1402#6e92a2-*#587084-{y43+1*395+528+1*131:1185+1*43,y87+1*439+352+1*175:1229+1*43,y175+1*43+265+1*130+617+1*43:570+1*43,y175+1*43+440+1*395:877+1*43,y175+1*43+484+1*307:921+1*43,y219+1*131+45+1*131:482+1*43,y219+1*43+177+1*130:526+1*43,y219+1*43+484+1*219:965+1*43,y219+1*43+528+1*87:1009+1*43,y263+1*43:1053+1*43,y263+1*702:1273+1*43,y307+1*43+308+1*175:1097+1*43,y351+1*87:306+1*43+45+1*43,y351+1*131:438+1*43,y439+1*87:218+1*43,y439+1*43:1141+1*43,y483+1*130+177+1*43+397+1*43:614+1*43,y527+1*130+89+1*175:658+1*43,y527+1*394:702+1*43,y570+1*395:746+1*43,y614+1*439:790+1*43,y614+1*483:834+1*42}#1e3147-{y131+1*43:570+1*87,y131+1*43+1056+1*43:658+1*218,y175+1*43:482+1*87,y219+1*43:394+1*87,y263+1*43:350+1*43,y307+1*43:306+1*43,y351+1*43:262+1*43,y439+1*87+352+1*87:1185+1*43,y527+1*350:1229+1*43,y966+1*87:1141+1*43,y1054+1*43:1097+1*43,y1098+1*43:1053+1*43,y1142+1*43:965+1*87,y1186+1*43:877+1*87}#85b0b5-{658+1*218:175+1*43,658+1*262:219+1*87,658+1*306:307+1*43,702+1*306:351+1*43,746+1*306:395+1*43,262+1*43+485+1*350:439+1*43,262+1*43+485+1*262:483+1*43,174+1*87+133+1*87+353+1*130:527+1*42,174+1*43+221+1*87:570+1*43,174+1*43+265+1*43:614+1*43,174+1*43+265+1*87:658+1*43,174+1*43+265+1*131:702+1*87,174+1*43+309+1*87:790+1*43,174+1*43+353+1*87:834+1*43,570+1*87:878+1*43,614+1*87:922+1*87,658+1*43:1010+1*43,658+1*87:1054+1*43,350+1*43+265+1*87:1098+1*43,394+1*87+177+1*87:1142+1*43,482+1*87+45+1*131:1186+1*43}#bad4cf-{921+1*43:219+1*43,921+1*131:263+1*43,965+1*131:307+1*43,1009+1*131:351+1*43,1053+1*87:395+1*43,306+1*43:439+1*43,306+1*87:483+1*43,262+1*131:527+1*42,218+1*219:570+1*43,306+1*175:614+1*43,394+1*87:658+1*43,438+1*43:702+1*43,482+1*43:790+1*43,482+1*87:834+1*43,218+1*43+265+1*43:878+1*43,218+1*43+265+1*87:922+1*43,262+1*43+221+1*87:966+1*43,262+1*43+221+1*131:1010+1*43,306+1*43+177+1*131:1054+1*43,482+1*175:1098+1*87,570+1*43:1186+1*43}#385165-{y263+1*87:394+1*87,y307+1*87:350+1*43,y351+1*87+45+1*482:1141+1*43,y395+1*43:262+1*43,y527+1*350:1185+1*43,y834+1*263:1053+1*43,y834+1*219:1097+1*43,y878+1*263:1009+1*43,y922+1*131:702+1*43,y966+1*263:746+1*43,y966+1*175:965+1*43,y1010+1*175:921+1*43,y1054+1*175:790+1*43,y1054+1*131:877+1*43,y1098+1*131:834+1*42}#f1f2d4-{218+1*87:614+1*43,218+1*175:658+1*43,218+1*219:702+1*43,218+1*263:746+1*131,262+1*263:878+1*87,306+1*219:966+1*87,350+1*175:1054+1*43,394+1*87:1098+1*43}#242424-{0+1*1401:1406+1}
```

</details>

### Google logo
#### PNG:
![Google logo](test-images/google_logo.png)
222 kB
### PCF:
173 kB (22% smaller)


## Contributing

Feel free to submit pull requests. If you're considering significant changes, please open an issue first to discuss your ideas.

## License

[MIT License](LICENSE)
