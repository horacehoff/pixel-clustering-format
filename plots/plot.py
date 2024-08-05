originals_ = [8955,127280, 18330, 111916, 17411, 52214, 48821, 6476, 29530, 325313, 17598, 484, 40808, 275919, 4405, 12811, 7089, 24405, 206909, 15909, 1759, 31484, 9862, 3713, 1666, 8302, 1485, 85339, 122049, 175186, 88773, 90801, 140291, 104991, 26292, 4129, 28968, 3709, 6581, 221337, 221554, 7150, 27255, 9700, 1858, 42950, 5042, 39994, 15927, 80025, 69602, 21581, 75210, 44940, 1647, 27325, 1727, 84137, 31484, 200156, 31484, 24253, 70622, 40134]
transformed_ = [1844, 55224, 12700, 158004, 380, 67748, 1624, 3184, 17988, 131568, 11044, 18, 48616, 115416, 5612, 30312, 12424, 19308, 170016, 404, 460, 17, 12572, 4808, 248, 12324, 136, 54444, 155988, 42700, 138260, 37940, 237664, 85616, 10904, 1952, 2352, 1696, 4516, 159080, 173392, 904, 19712, 16744, 1520, 19688, 1424, 15256, 15168, 55008, 804, 1700, 82404, 68060, 352, 9852, 316, 61704, 17, 176988, 17, 21428, 46952, 23488]
names_ = ['skulls_pixel_art.png', 'musescore_logo.png', 'simple_shapes_rendering.png', 'meta_logo.png', 'green_character_pixel_art.png', 'wordpress_logo.png', 'pixel_art_isometric_basic_shapes.png', 'resistor_schematic.png', 'simple_box_model.png', 'sword_render.png', 'computer_blue_screen_screenshot.png', 'blue.png', 'chatgpt_logo.png', 'grass_pixel_art.png', 'tark-pixel-art.png', 'bloomberg_logo.png', 'spotify_logo.png', 'boat_pixel_art.png', 'table_drawing_schematic.png', 'cube_pixel_art.png', 'npm_logo.png', 'black.png', 'wikipedia_logo.png', 'tv_pixel_art.png', 'Pixel_Art_Isometric_Example_3_Bigx4.png', 'nyt_logo.png', 'us_flag_no_stars.png', 'mountains_logo.png', 'boat_drawing.png', 'intel_logo.png', 'github_actions_ui.png', 'graph.png', 'skull_and_sword_pixel_art_by_dulcahn_dfn2ikx-pre.png', 'tree_drawing.png', 'shop_logo.png', 'tree_pixel_art.png', '9_squares_solidcolor_pixel_art.png', 'cat_pixel_art.png', 'curve_graph.png', 'pplx-default-preview.png', 'google_logo.png', 'mario-pixel-art.png', 'castle_pixel_art_isometric.png', 'threads_logo.png', 'texture_pixel_art.png', 'mda_logo.png', 'lightning_bolt_drawing.png', 'library_of_congress_logo.png', 'openalex_logo.png', 'nitrogen_cycle_diagram.png', 'blue_ball_pixel_art.png', 'yellow_black.png', 'handshake_banner.png', 'mtv_logo.png', 'metal_texture_pixel_art.png', 'wood_wall_texture_pixel_art.png', 'pixel_art_comparison.png', 'astro_logo.png', 'white.png', 'among_us_pixel_art.png', 'red.png', 'simpletv_logo.png', 'rgb_color_palette.png', 'buffalo_university_logo.png']


# Inspired by https://python-graph-gallery.com/10-barplot-with-number-of-observation/
import matplotlib.pyplot as plt


def split_list(to_split, n):
    length = len(to_split)
    for indx in range(0, length, n):
        end = min(indx + n, length)
        yield to_split[indx:end]


originals_split = split_list(originals_, 10)
transformed_split = split_list(transformed_, 10)
names_split = split_list(names_, 10)

chunkindex = 0
for (originals, transformed, names) in zip(originals_split, transformed_split, names_split):
    chunkindex += 1
    barWidth = 1
    png_bar = []
    lpi_bar = []
    png_lpi_bar = []
    for i, x in enumerate(originals):
        png_bar.append(x / 1000)
        lpi_bar.append(transformed[i] / 1000)
        png_lpi_bar.append(x / 1000)
        png_lpi_bar.append(transformed[i] / 1000)

    png_bar_pos = []
    lpi_bar_pos = []
    for i in range(len(originals+transformed)):
        if (i % 2) == 0:
            png_bar_pos.append(i+1)
        else:
            lpi_bar_pos.append(i+1)

    all_bar_pos = range(len(originals+transformed))

    plt.bar(png_bar_pos, png_bar, width=barWidth, color='r', label='.PNG')
    plt.bar(lpi_bar_pos, lpi_bar, width=barWidth, color='b', label='.LPI')

    plt.legend()

    bar_names = []
    for i in names:
        bar_names.append(i)
        bar_names.append("")

    plt.xticks([r + barWidth for r in range(len(all_bar_pos))],
               bar_names, rotation=90, size=3)


    plt.ylabel("Image size (kB)")

    label = []
    for x, i in enumerate(originals):
        if i / 1000 > 1:
            label.append(round(i / 1000))
        else:
            label.append(i / 1000)
        if transformed[x] / 1000 > 1:
            label.append(round(transformed[x] / 1000))
        else:
            label.append(transformed[x] / 1000)

    for i in range(len(all_bar_pos)):
        plt.text(x=all_bar_pos[i] + 0.5, y=png_lpi_bar[i] + 0.1, s=label[i], size=6)

    plt.savefig("fig"+str(chunkindex)+".png", dpi=1000, pad_inches=0, bbox_inches="tight")
    plt.clf()
