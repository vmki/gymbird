import imageio
import os

for file in os.listdir("assets"):
    img = imageio.imread("assets/"+file)
    height, width = img.shape[:2]
    width_cutoff = width // 2
    s1 = img[:, :width_cutoff]
    s2 = img[:, width_cutoff:]
    imageio.imsave("assets/preview-" + file, s2)
