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
git clone https://github.com/just-a-mango/pixel-clustering-image-format
```

## Usage

```sh
cargo run --release -- [filename] [--verbose] [--decode]
// OR
./pcf [filename] [--verbose] [--decode]
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
1920%1080%#0000FE%
```
18 bytes (~27 times smaller)

### Blue ball pixel art
#### PNG:
![Blue ball pixel art](test-images/blue_ball_pixel_art.png)
70 kB
#### PCF:

595 bytes (~120 times smaller)
<details>
<summary>Uncompressed</summary>

```
1408%1402%#6E92A2%#F1F2D4{306+1*219:966+1*87,218+1*87:614a3,218+1*263:746+1*131,394+1*87:1098a3,218+1*219:702a3,350+1*175:1054a3,262+1*263:878+1*87,218+1*175:658a3}#1E3147{306a3:307a3,965+1*87:1142a3,1097a3:1054a3,1141a3:966+1*87,262a3:351a3,1185a3:439+1*87+352+1*87,394+1*87:219a3,482+1*87:175a3,658+1*218:1230a3,877+1*87:1186a3,350a3:263a3,570+1*306:131a3,1053a3:1098a3,1229a3:527+1*350}#385165{746+1*306:1098a3,1053+1*175:834a3,1141a3:483a3,746+1*130:1186a3,350a3+748a3:351a3,702a3+264+1*175:922a3,350+1*131:307a3,746+1*218:1142a3,262a3+836a3:395a3,702+1*87+176+1*175:966a3,702+1*87+132+1*219:1010a3,1009+1*175:878a3,1141+1*87:527+1*306,394+1*87:263a3,746+1*87+44+1*219:1054a3}#242424{0+1*1401:1406+1}#85B0B5{350a3+265+1*87:1098a3,658a3:1010a3,174a3+353+1*87:834a3,262a3+485+1*350:439a3,482+1*87+45+1*131:1186a3,174a3+265+1*87:658a3,702+1*306:351a3,174a3+221+1*87:570a3,658+1*87:1054a3,746+1*306:395a3,174a3+265+1*131:702+1*87,570+1*87:878a3,614+1*87:922+1*87,658+1*306:307a3,174a3+309+1*87:790a3,174+1*87+133+1*87+353+1*130:527a2,174a3+265a3:614a3,262a3+485+1*262:483a3,658+1*262:219+1*87,658+1*218:175a3,394+1*87+177+1*87:1142a3}#587084{790+1*174+221+1*87:966a3,658+1*350+221+1*87:878a3,658+1*350+89a3+133a3:746a3,658+1*394+221a3:834a3,702+1*262+133a3+133a3:702a3,526+1*219+528a3:527a2,306a3+45+1*87+704+1*131:351a3,702+1*218+177a3+133a3:658a3,1185a3:43a3,482a3+572a3+45+1*131:307a3,218a3+177+1*131+572a3+45+1*87:439a3,746+1*262+221+1*87:922a3,570a3+264+1*87+221+1*87:175a3,570+1*87:1230a3,482a3+528a3+89+1*131:263a3,482+1*87+396+1*87+133+1*87:219a3,306a3+45+1*131+660+1*131:395a3,834a2+309a3:1054a3,570+1*219+484a3:570a3,658+1*218+397a3:614a3,790+1*130+265+1*87:1010a3,1185+1*87:87+1*87,218a3+221+1*175+572+1*87:483a3,614a38+45a3+133a3:790a3}#BAD4CF{262a3+221+1*131:1010a3,306a3:439a3,218a3+265a3:878a3,921a3:219a3,218+1*219:570a3,262a3+221+1*87:966a3,921+1*131:263a3,438a3:702a3,482a3:790a3,262+1*131:527a2,1053+1*87:395a3,306a3+177+1*131:1054a3,482+1*87:834a3,394+1*87:658a3,1009+1*131:351a3,218a3+265+1*87:922a3,306+1*175:614a3,306+1*87:483a3,965+1*131:307a3,570a3:1186a3,482+1*175:1098+1*87}_$+1*4$a
```

</details>

### Google logo
#### PNG:
![Google logo](test-images/google_logo.png)
222 kB
### PCF:

156 kB (30% smaller)


## License

[MIT License](LICENSE)
