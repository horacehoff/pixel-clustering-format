names = ["among_us_pixel_art.png", "astro_logo.png", "9_squares_solidcolor_pixel_art.png", "black.png", "blue.png", "blue_ball_pixel_art.png", "boat_pixel_art.png", "boat_drawing.png", "buffalo_university_logo.png", "computer_blue_screen_screenshot.png", "chatgpt_logo.png", "cube_pixel_art.png", "cat_pixel_art.png", "curve_graph.png", "castle_pixel_art_isometric.png", "fig1.png", "fig2.png", "fig3.png", "fig4.png", "fig5.png", "fig6.png", "fig7.png", "green_character_pixel_art.png", "grass_pixel_art.png", "github_actions_ui.png", "graph.png", "google_logo.png", "handshake_banner.png", "intel_logo.png", "lightning_bolt_drawing.png", "library_of_congress_logo.png", "musescore_logo.png", "meta_logo.png", "mountains_logo.png", "mario-pixel-art.png", "mda_logo.png", "mtv_logo.png", "metal_texture_pixel_art.png", "npm_logo.png", "nyt_logo.png", "nitrogen_cycle_diagram.png", "openalex_logo.png", "pixel_art_isometric_basic_shapes.png", "pixel_Art_Isometric_Example_3_Bigx4.png", "pplx-default-preview.png", "pixel_art_comparison.png", "resistor_schematic.png", "red.png", "rgb_color_palette.png", "skulls_pixel_art.png", "simple_shapes_rendering.png", "simple_box_model.png", "sword_render.png", "spotify_logo.png", "skull_and_sword_pixel_art_by_dulcahn_dfn2ikx-pre.png", "shop_logo.png", "simpletv_logo.png", "tark-pixel-art.png", "table_drawing_schematic.png", "tv_pixel_art.png", "tree_drawing.png", "tree_pixel_art.png", "threads_logo.png", "texture_pixel_art.png", "us_flag_no_stars.png", "wordpress_logo.png", "wikipedia_logo.png", "wood_wall_texture_pixel_art.png", "white.png", "yellow_black.png"]
originals_ = [200156, 84137, 28968, 31484, 484, 69602, 24405, 122049, 40134, 17598, 40808, 15909, 3709, 6581, 27255, 411014, 380288, 365610, 392956, 361872, 390800, 267916, 17411, 275919, 88773, 90801, 221554, 75210, 175186, 5042, 39994, 127280, 111916, 85339, 7150, 42950, 44940, 1647, 1759, 8302, 80025, 15927, 48821, 1666, 221337, 1727, 6476, 31484, 70622, 8955, 18330, 29530, 325313, 7089, 140291, 26292, 24253, 4405, 206909, 3713, 104991, 4129, 9700, 1858, 1485, 52214, 9862, 27325, 31484, 21581]
transformed = [150258, 49029, 1845, 19, 18, 598, 15514, 138883, 20103, 10210, 58285, 263, 1159, 5015, 14539, 137626, 127795, 121999, 129246, 122993, 139042, 70262, 275, 102319, 129960, 32712, 156338, 70139, 38373, 1250, 12301, 67068, 147368, 49823, 655, 17942, 87719, 246, 344, 12543, 55302, 12206, 1239, 168, 163565, 231, 2797, 21, 44501, 1233, 15358, 15813, 138625, 10615, 227448, 11738, 19801, 4892, 158418, 3588, 80335, 1282, 16816, 1078, 73, 66592, 8301, 7918, 20, 1981]
compressed_lossy = [72400, 30228, 433, 19, 18, 589, 14700, 68769, 7735, 8249, 20129, 258, 1145, 3027, 13178, 54908, 52232, 50201, 52621, 51131, 54604, 29363, 270, 88595, 84660, 13855, 73066, 34471, 11302, 1242, 4722, 23492, 80877, 20327, 651, 8186, 72211, 236, 199, 4180, 34149, 5863, 1227, 165, 97064, 229, 1424, 21, 20793, 1218, 14642, 8305, 105139, 4045, 114201, 4916, 5403, 4642, 64118, 2926, 31779, 1284, 5321, 1072, 72, 19890, 8286, 5624, 20, 1101]

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

    plt.bar(png_bar_pos, png_bar, width=barWidth, color='r', label='PNG')
    plt.bar(pcf_bar_pos, pcf_bar, width=barWidth, color='b', label='PCF')
    plt.bar(pcf_lossy_bar_pos, pcf_lossy_bar, width=barWidth, color='g', label='PCF --lossy')

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
        plt.text(x=all_bar_pos[i] + 0.5, y=label[i] + 0.1, s=label[i], size=5)

    plt.savefig(f"fig{chunkindex}.png", dpi=1000, pad_inches=0, bbox_inches="tight")
    plt.clf()