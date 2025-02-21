originals_ = [28968, 200156, 84137, 31484, 484, 69602, 122049, 24405, 40134, 27255, 3709, 40808, 17598, 15909, 6581, 411014, 380288, 365610, 392956, 361872, 390800, 267916, 88773, 221554, 90801, 275919, 17411, 75210, 175186, 39994, 5042, 7150, 42950, 1647, 111916, 85339, 44940, 127280, 80025, 1759, 8302, 15927, 1727, 48821, 1666, 221337, 31484, 6476, 70622, 26292, 24253, 29530, 18330, 8955, 140291, 7089, 325313, 206909, 4405, 1858, 9700, 104991, 4129, 3713, 1485, 31484, 9862, 27325, 52214, 21581]
transformed = [1835, 150247, 49101, 19, 18, 592, 138754, 15451, 20154, 14531, 1152, 57940, 10217, 257, 5004, 137650, 127981, 121892, 129424, 122879, 139094, 70449, 130215, 156277, 32781, 102804, 274, 70141, 38325, 12316, 1249, 658, 17939, 241, 147577, 49773, 87911, 67028, 55255, 342, 12541, 12229, 235, 1243, 170, 163317, 21, 2804, 44526, 11737, 19780, 15769, 15382, 1222, 227504, 10638, 137950, 158356, 4877, 1090, 16839, 80389, 1299, 3582, 72, 20, 8287, 7950, 66643, 1965]
compressed_lossy = [1835, 150247, 49101, 19, 18, 592, 138754, 15451, 20154, 14531, 1152, 57940, 10217, 257, 5004, 137650, 127981, 121892, 129424, 122879, 139094, 70449, 130215, 156277, 32781, 102804, 274, 70141, 38325, 12316, 1249, 658, 17939, 241, 147577, 49773, 87911, 67028, 55255, 342, 12541, 12229, 235, 1243, 170, 163317, 21, 2804, 44526, 11737, 19780, 15769, 15382, 1222, 227504, 10638, 137950, 158356, 4877, 1090, 16839, 80389, 1299, 3582, 72, 20, 8287, 7950, 66643, 1965]
names = ["9_squares_solidcolor_pixel_art.png", "among_us_pixel_art.png", "astro_logo.png", "black.png", "blue.png", "blue_ball_pixel_art.png", "boat_drawing.png", "boat_pixel_art.png", "buffalo_university_logo.png", "castle_pixel_art_isometric.png", "cat_pixel_art.png", "chatgpt_logo.png", "computer_blue_screen_screenshot.png", "cube_pixel_art.png", "curve_graph.png", "fig1.png", "fig2.png", "fig3.png", "fig4.png", "fig5.png", "fig6.png", "fig7.png", "github_actions_ui.png", "google_logo.png", "graph.png", "grass_pixel_art.png", "green_character_pixel_art.png", "handshake_banner.png", "intel_logo.png", "library_of_congress_logo.png", "lightning_bolt_drawing.png", "mario-pixel-art.png", "mda_logo.png", "metal_texture_pixel_art.png", "meta_logo.png", "mountains_logo.png", "mtv_logo.png", "musescore_logo.png", "nitrogen_cycle_diagram.png", "npm_logo.png", "nyt_logo.png", "openalex_logo.png", "pixel_art_comparison.png", "pixel_art_isometric_basic_shapes.png", "Pixel_Art_Isometric_Example_3_Bigx4.png", "pplx-default-preview.png", "red.png", "resistor_schematic.png", "rgb_color_palette.png", "shop_logo.png", "simpletv_logo.png", "simple_box_model.png", "simple_shapes_rendering.png", "skulls_pixel_art.png", "skull_and_sword_pixel_art_by_dulcahn_dfn2ikx-pre.png", "spotify_logo.png", "sword_render.png", "table_drawing_schematic.png", "tark-pixel-art.png", "texture_pixel_art.png", "threads_logo.png", "tree_drawing.png", "tree_pixel_art.png", "tv_pixel_art.png", "us_flag_no_stars.png", "white.png", "wikipedia_logo.png", "wood_wall_texture_pixel_art.png", "wordpress_logo.png", "yellow_black.png"]

# Inspired by https://python-graph-gallery.com/10-barplot-with-number-of-observation/
import matplotlib.pyplot as plt


def split_list(to_split, n):
    length = len(to_split)
    for indx in range(0, length, n):
        end = min(indx + n, length)
        yield to_split[indx:end]


originals_split = split_list(originals_, 10)
transformed_split = split_list(transformed, 10)
compressed_lossy_split = split_list(compressed_lossy, 10)
names_split = split_list(names, 10)

chunkindex = 0
for (originals, transformed, compressed_lossy, names) in zip(originals_split, transformed_split, compressed_lossy_split, names_split):
    chunkindex += 1
    barWidth = 1

    png_bar = [x / 1000 for x in originals]
    pcf_bar = [x / 1000 for x in transformed]
    pcf_lossy_bar = [x / 1000 for x in compressed_lossy]

    png_bar_pos = []
    pcf_bar_pos = []
    pcf_lossy_bar_pos = []

    for i in range(len(originals * 3)):
        if i % 3 == 0:
            png_bar_pos.append(i + 1)
        elif i % 3 == 1:
            pcf_bar_pos.append(i + 1)
        else:
            pcf_lossy_bar_pos.append(i + 1)

    all_bar_pos = range(len(originals) * 3)

    plt.bar(png_bar_pos, png_bar, width=barWidth, color='r', label='.PNG')
    plt.bar(pcf_bar_pos, pcf_bar, width=barWidth, color='b', label='.PCF')
    plt.bar(pcf_lossy_bar_pos, pcf_lossy_bar, width=barWidth, color='g', label='.PCF lossy')

    plt.legend()

    bar_names = []
    for i in names:
        bar_names.append(i)
        bar_names.append("")
        bar_names.append("")

    plt.xticks([r + barWidth for r in range(len(all_bar_pos))], bar_names, rotation=90, size=3)

    plt.ylabel("Image size (kB)")

    label = []
    for x, i in enumerate(originals):
        label.append(round(i / 1000, 2))
        label.append(round(transformed[x] / 1000, 2))
        label.append(round(compressed_lossy[x] / 1000, 2))

    for i in range(len(all_bar_pos)):
        plt.text(x=all_bar_pos[i] + 0.5, y=label[i] + 0.1, s=label[i], size=6)

    plt.savefig(f"fig{chunkindex}.png", dpi=1000, pad_inches=0, bbox_inches="tight")
    plt.clf()