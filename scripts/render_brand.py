#!/usr/bin/env python3
"""Rasterize Castle brand assets from geometry + system type.

Hero and social preview must get the wordmark and tagline exactly right,
so this is drawn with Pillow rather than an image model.
"""

from __future__ import annotations

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
INK = (14, 17, 22, 255)  # #0E1116
STONE = (201, 205, 212, 255)  # #C9CDD4
MORTAR = (139, 145, 154, 255)  # #8B919A
WORD = (236, 238, 241, 255)
ACCENT = (76, 141, 222, 255)  # #4C8DDE
ARCH = (14, 17, 22, 255)


def font(path: str, size: int, index: int = 0) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype(path, size=size, index=index)


def draw_keep(draw: ImageDraw.ImageDraw, cx: float, base_y: float, scale: float) -> None:
    """Flat keep: merlons, arch, quiet pennant. No shadows, no gradient."""
    s = scale
    body_w, body_h = 92 * s, 78 * s
    left = cx - body_w / 2
    top = base_y - body_h
    merlon_w, merlon_h, gap = 18 * s, 14 * s, 8 * s

    # Three merlons, centered on the body.
    merlon_span = 3 * merlon_w + 2 * gap
    mx0 = cx - merlon_span / 2
    for i in range(3):
        x = mx0 + i * (merlon_w + gap)
        draw.rectangle([x, top - merlon_h, x + merlon_w, top + 1], fill=STONE)

    draw.rectangle([left, top, left + body_w, base_y], fill=STONE)

    # Arch: cut a dark doorway out of the body.
    arch_w, arch_h = 30 * s, 38 * s
    ax0 = cx - arch_w / 2
    ay1 = base_y
    ay0 = ay1 - arch_h
    draw.rectangle([ax0, ay0 + arch_w / 2, ax0 + arch_w, ay1], fill=ARCH)
    draw.ellipse([ax0, ay0, ax0 + arch_w, ay0 + arch_w], fill=ARCH)

    # Pole + small pennant from the center merlon.
    pole_x = cx
    pole_top = top - merlon_h - 28 * s
    draw.rectangle([pole_x - 1.1 * s, pole_top, pole_x + 1.1 * s, top - merlon_h], fill=MORTAR)
    flag = [
        (pole_x + 1.1 * s, pole_top + 2 * s),
        (pole_x + 20 * s, pole_top + 9 * s),
        (pole_x + 1.1 * s, pole_top + 16 * s),
    ]
    draw.polygon(flag, fill=ACCENT)


def render_banner(path: Path, width: int, height: int, word_size: int, tag_size: int) -> None:
    img = Image.new("RGBA", (width, height), INK)
    draw = ImageDraw.Draw(img)
    word = font("/System/Library/Fonts/HelveticaNeue.ttc", word_size, index=0)
    tag = font("/System/Library/Fonts/HelveticaNeue.ttc", tag_size, index=0)
    wordmark = "Castle"
    tagline = "Cargo for small C++ projects."

    # Vertical stack: keep, word, tagline — optically centered.
    keep_scale = height / 300
    keep_base = height * 0.46
    draw_keep(draw, width / 2, keep_base, keep_scale)

    wb = draw.textbbox((0, 0), wordmark, font=word)
    tb = draw.textbbox((0, 0), tagline, font=tag)
    word_y = keep_base + height * 0.07
    tag_y = word_y + (wb[3] - wb[1]) + height * 0.035
    draw.text(((width - (wb[2] - wb[0])) / 2, word_y), wordmark, font=word, fill=WORD)
    draw.text(((width - (tb[2] - tb[0])) / 2, tag_y), tagline, font=tag, fill=MORTAR)
    img.save(path, "PNG", optimize=True)


def main() -> None:
    render_banner(ROOT / "hero.png", 1600, 600, 92, 28)
    render_banner(ROOT / ".github" / "social-preview.png", 1280, 640, 88, 26)
    print("wrote hero.png and .github/social-preview.png")


if __name__ == "__main__":
    main()
