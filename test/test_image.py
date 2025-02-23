import requests
import ril
from ril import Rgba

PIXELS = [
    Rgba(255, 0, 0, 255),
    Rgba(255, 128, 0, 255),
    Rgba(255, 255, 0, 255),
    Rgba(128, 255, 0, 255),
    Rgba(0, 255, 0, 255),
    Rgba(0, 255, 128, 255),
    Rgba(0, 255, 255, 255),
    Rgba(0, 128, 255, 255),
    Rgba(0, 0, 255, 255),
    Rgba(128, 0, 255, 255),
    Rgba(255, 0, 255, 255),
    Rgba(255, 0, 128, 255),
]

def test_create_image() -> None:
    image = ril.Image.new(1, 1, ril.Pixel.from_rgb(255, 255, 255))
    
    assert image.height == 1
    assert image.width == 1
    assert image.dimensions == (1, 1)

def test_image_pixels() -> None:
    image = ril.Image.new(1, 1, ril.Pixel.from_rgb(255, 255, 255))

    image.pixels()

def test_gif_decode(fetch_file) -> None:
    for i, frame in enumerate(ril.ImageSequence.from_bytes(fetch_file('sample_rgba.gif'))):
        frame = frame.image

        assert frame.dimensions == (256, 256)
        assert frame.get_pixel(0, 0) == PIXELS[i]
