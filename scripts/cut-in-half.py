import imageio
import os

for file in os.listdir("../data"):
    img = imageio.imread("../data/"+file)
    height, width = img.shape[:2]
    width_cutoff = width // 2
    s1 = img[:, :width_cutoff]
    s2 = img[:, width_cutoff:]
    imageio.imsave("../data/preview-" + file, s2)
