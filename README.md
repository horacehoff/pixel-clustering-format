# Lossless Pixel-Clustering Image Format (LPI)

LPI is an image format that works by grouping pixels together in clusters that rely on simple additions and multiplications for compression, and grouping them by color.

It uses "layers" to store pixels, that is it completely disregards the pixels of the dominant color in the image, using it as a background. Then, it fills in the image using the other colors, thus allowing for a very efficient lossless compression.

On images where it works best, LPI offers a great compression level, sometimes being as much as 99% smaller than the original PNG image. However, it poorly compresses colors and their relation with pixels and as such, LPI works best with images that don't have a ton of colors and/or that have a high pixel/color ratio. Examples below. 

LPI supports transparency, and first reads colors in the RGBA format and then stores them in the hexadecimal format.

## Installation

```bash
git clone https://github.com/just-a-mango/lpi
```

## Usage

```sh
python convert.py [filename] [-o] [-v] [-d]

python back_convert.py [filename]

positional arguments:
  filename

options:
  -o, --output    The .lpi output file
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
#### LPI:
804 bytes (~86 times smaller)

### Google logo
#### PNG:
![Google logo](test-images/google_logo.png)
222 kB
### LPI:
173 kB (22% smaller)


## Contributing

Feel free to submit pull requests. If you're considering significant changes, please open an issue first to discuss your ideas.

## License

[MIT License](LICENSE)
