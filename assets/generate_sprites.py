#!/usr/bin/env python3
"""Generate 32x32 pixel art sprites for the dungeon game."""

from PIL import Image, ImageDraw

SIZE = 32


def create_sprite(draw_func, filename):
    """Create a 32x32 sprite with the given drawing function."""
    img = Image.new("RGBA", (SIZE, SIZE), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)
    draw_func(draw)
    img.save(f"sprites/{filename}")
    print(f"Created sprites/{filename}")


# ============================================================================
# Terrain Tiles
# ============================================================================


def draw_floor(draw):
    """Stone floor tile with subtle texture."""
    # Base color
    draw.rectangle([0, 0, SIZE - 1, SIZE - 1], fill=(70, 70, 80, 255))
    # Stone texture lines
    for i in range(0, SIZE, 8):
        draw.line([(i, 0), (i, SIZE - 1)], fill=(60, 60, 70, 255), width=1)
        draw.line([(0, i), (SIZE - 1, i)], fill=(60, 60, 70, 255), width=1)
    # Add some dots for texture
    for x in range(4, SIZE, 8):
        for y in range(4, SIZE, 8):
            draw.point((x, y), fill=(80, 80, 90, 255))


def draw_wall(draw):
    """Stone wall with brick pattern."""
    # Base color
    draw.rectangle([0, 0, SIZE - 1, SIZE - 1], fill=(100, 80, 60, 255))
    # Brick pattern
    brick_h = 8
    for row in range(4):
        y = row * brick_h
        offset = (row % 2) * (SIZE // 2)
        for col in range(-1, 3):
            x = col * SIZE // 2 + offset
            # Brick outline
            draw.rectangle(
                [x + 1, y + 1, x + SIZE // 2 - 2, y + brick_h - 2],
                fill=(110, 90, 70, 255),
                outline=(80, 60, 45, 255),
            )
    # Highlight top
    draw.line([(0, 0), (SIZE - 1, 0)], fill=(130, 110, 90, 255), width=1)


def draw_void(draw):
    """Dark void/abyss."""
    draw.rectangle([0, 0, SIZE - 1, SIZE - 1], fill=(15, 15, 25, 255))
    # Add subtle stars/specks
    import random

    random.seed(42)  # Deterministic
    for _ in range(8):
        x, y = random.randint(0, SIZE - 1), random.randint(0, SIZE - 1)
        draw.point((x, y), fill=(40, 40, 60, 255))


def draw_water(draw):
    """Animated water tile (static frame)."""
    # Base water color
    draw.rectangle([0, 0, SIZE - 1, SIZE - 1], fill=(30, 80, 150, 255))
    # Wave pattern
    for y in range(0, SIZE, 6):
        for x in range(SIZE):
            offset = (x // 4) % 2
            if (y + offset * 3) % 6 < 3:
                draw.point((x, y), fill=(50, 100, 180, 255))
                draw.point((x, y + 1), fill=(40, 90, 160, 255))
    # Highlights
    draw.line([(4, 8), (12, 8)], fill=(80, 130, 200, 255), width=1)
    draw.line([(18, 20), (28, 20)], fill=(80, 130, 200, 255), width=1)


def draw_custom(draw):
    """Custom terrain (purple mystery tile)."""
    draw.rectangle([0, 0, SIZE - 1, SIZE - 1], fill=(100, 40, 120, 255))
    # Diamond pattern
    center = SIZE // 2
    draw.polygon(
        [(center, 4), (SIZE - 4, center), (center, SIZE - 4), (4, center)],
        fill=(130, 60, 150, 255),
        outline=(80, 30, 100, 255),
    )
    # Center dot
    draw.ellipse([center - 3, center - 3, center + 3, center + 3], fill=(180, 100, 200, 255))


# ============================================================================
# Actors
# ============================================================================


def draw_player(draw):
    """Player character - heroic figure."""
    # Body (green tunic)
    draw.rectangle([10, 12, 21, 24], fill=(40, 160, 60, 255))
    # Head
    draw.ellipse([11, 4, 20, 13], fill=(220, 180, 140, 255))
    # Eyes
    draw.point((14, 8), fill=(40, 40, 40, 255))
    draw.point((17, 8), fill=(40, 40, 40, 255))
    # Hair
    draw.arc([11, 2, 20, 10], 0, 180, fill=(100, 70, 40, 255), width=2)
    # Arms
    draw.rectangle([6, 14, 10, 20], fill=(220, 180, 140, 255))
    draw.rectangle([21, 14, 25, 20], fill=(220, 180, 140, 255))
    # Legs
    draw.rectangle([11, 24, 15, 30], fill=(60, 50, 40, 255))
    draw.rectangle([16, 24, 20, 30], fill=(60, 50, 40, 255))
    # Sword (right hand)
    draw.rectangle([24, 8, 26, 20], fill=(180, 180, 190, 255))
    draw.rectangle([23, 18, 27, 20], fill=(140, 100, 60, 255))


def draw_enemy_goblin(draw):
    """Goblin enemy - small green creature."""
    # Body (greenish)
    draw.ellipse([8, 12, 23, 26], fill=(80, 130, 70, 255))
    # Head
    draw.ellipse([10, 4, 21, 15], fill=(90, 140, 80, 255))
    # Big ears
    draw.polygon([(8, 8), (4, 2), (10, 10)], fill=(90, 140, 80, 255))
    draw.polygon([(23, 8), (27, 2), (21, 10)], fill=(90, 140, 80, 255))
    # Eyes (angry red)
    draw.ellipse([12, 7, 15, 11], fill=(200, 50, 50, 255))
    draw.ellipse([16, 7, 19, 11], fill=(200, 50, 50, 255))
    draw.point((13, 8), fill=(255, 255, 255, 255))
    draw.point((17, 8), fill=(255, 255, 255, 255))
    # Mouth
    draw.arc([13, 10, 18, 14], 0, 180, fill=(40, 40, 40, 255), width=1)
    # Legs
    draw.rectangle([10, 25, 14, 30], fill=(80, 130, 70, 255))
    draw.rectangle([17, 25, 21, 30], fill=(80, 130, 70, 255))


def draw_enemy_skeleton(draw):
    """Skeleton enemy - undead warrior."""
    # Skull
    draw.ellipse([10, 3, 21, 14], fill=(240, 240, 230, 255))
    # Eye sockets
    draw.ellipse([12, 6, 15, 10], fill=(20, 20, 20, 255))
    draw.ellipse([16, 6, 19, 10], fill=(20, 20, 20, 255))
    # Nose hole
    draw.polygon([(15, 10), (16, 10), (15.5, 12)], fill=(20, 20, 20, 255))
    # Teeth
    draw.rectangle([12, 12, 19, 14], fill=(240, 240, 230, 255))
    draw.line([(14, 12), (14, 14)], fill=(20, 20, 20, 255), width=1)
    draw.line([(17, 12), (17, 14)], fill=(20, 20, 20, 255), width=1)
    # Ribcage
    draw.rectangle([11, 15, 20, 24], fill=(230, 230, 220, 255))
    for y in range(16, 24, 2):
        draw.line([(12, y), (19, y)], fill=(200, 200, 190, 255), width=1)
    # Arms (bones)
    draw.line([(8, 16), (8, 22)], fill=(240, 240, 230, 255), width=2)
    draw.line([(23, 16), (23, 22)], fill=(240, 240, 230, 255), width=2)
    # Legs (bones)
    draw.line([(13, 24), (13, 30)], fill=(240, 240, 230, 255), width=2)
    draw.line([(18, 24), (18, 30)], fill=(240, 240, 230, 255), width=2)


def draw_enemy_slime(draw):
    """Slime enemy - blob creature."""
    # Main body (jelly)
    draw.ellipse([4, 10, 27, 28], fill=(100, 180, 100, 180))
    # Highlight
    draw.ellipse([6, 12, 14, 18], fill=(140, 220, 140, 150))
    # Eyes
    draw.ellipse([10, 16, 14, 20], fill=(255, 255, 255, 255))
    draw.ellipse([17, 16, 21, 20], fill=(255, 255, 255, 255))
    draw.ellipse([11, 17, 13, 19], fill=(20, 20, 20, 255))
    draw.ellipse([18, 17, 20, 19], fill=(20, 20, 20, 255))
    # Happy mouth
    draw.arc([12, 20, 19, 25], 0, 180, fill=(60, 120, 60, 255), width=2)


# ============================================================================
# Props
# ============================================================================


def draw_door_closed(draw):
    """Closed wooden door."""
    # Door frame
    draw.rectangle([4, 2, 27, 29], fill=(80, 60, 40, 255))
    # Door panels
    draw.rectangle([6, 4, 25, 27], fill=(120, 90, 60, 255))
    # Planks
    draw.line([(6, 10), (25, 10)], fill=(100, 75, 50, 255), width=1)
    draw.line([(6, 18), (25, 18)], fill=(100, 75, 50, 255), width=1)
    # Handle
    draw.ellipse([20, 14, 24, 18], fill=(200, 180, 100, 255))


def draw_door_open(draw):
    """Open door (passable)."""
    # Door frame
    draw.rectangle([4, 2, 27, 29], fill=(80, 60, 40, 255))
    # Opening (dark)
    draw.rectangle([6, 4, 25, 27], fill=(30, 30, 35, 255))
    # Door edge (ajar)
    draw.polygon([(6, 4), (10, 4), (8, 27), (6, 27)], fill=(120, 90, 60, 255))


def draw_switch_off(draw):
    """Lever/switch in off position."""
    # Base plate
    draw.rectangle([8, 20, 23, 28], fill=(100, 100, 110, 255))
    # Lever base
    draw.ellipse([12, 18, 19, 25], fill=(80, 80, 90, 255))
    # Lever arm (down position)
    draw.line([(15, 21), (10, 10)], fill=(140, 140, 150, 255), width=3)
    draw.ellipse([8, 8, 12, 12], fill=(180, 80, 80, 255))


def draw_switch_on(draw):
    """Lever/switch in on position."""
    # Base plate
    draw.rectangle([8, 20, 23, 28], fill=(100, 100, 110, 255))
    # Lever base
    draw.ellipse([12, 18, 19, 25], fill=(80, 80, 90, 255))
    # Lever arm (up position)
    draw.line([(15, 21), (20, 10)], fill=(140, 140, 150, 255), width=3)
    draw.ellipse([18, 8, 22, 12], fill=(80, 180, 80, 255))


def draw_hazard(draw):
    """Hazard/trap tile (spikes)."""
    # Base
    draw.rectangle([0, 24, SIZE - 1, SIZE - 1], fill=(80, 70, 60, 255))
    # Spikes
    for x in range(4, SIZE - 2, 6):
        draw.polygon([(x, 24), (x + 3, 6), (x + 6, 24)], fill=(160, 160, 170, 255))
        # Spike highlight
        draw.line([(x + 3, 6), (x + 3, 24)], fill=(200, 200, 210, 255), width=1)


# ============================================================================
# Items
# ============================================================================


def draw_item_potion_health(draw):
    """Red health potion."""
    # Bottle body
    draw.ellipse([8, 12, 23, 28], fill=(180, 40, 40, 255))
    # Bottle neck
    draw.rectangle([12, 6, 19, 14], fill=(180, 40, 40, 255))
    # Cork
    draw.rectangle([13, 4, 18, 8], fill=(140, 100, 60, 255))
    # Highlight
    draw.ellipse([10, 14, 14, 20], fill=(220, 80, 80, 255))
    # Label
    draw.rectangle([10, 20, 21, 24], fill=(240, 240, 230, 255))
    draw.text((12, 19), "+", fill=(180, 40, 40, 255))


def draw_item_potion_mana(draw):
    """Blue mana potion."""
    # Bottle body
    draw.ellipse([8, 12, 23, 28], fill=(40, 80, 180, 255))
    # Bottle neck
    draw.rectangle([12, 6, 19, 14], fill=(40, 80, 180, 255))
    # Cork
    draw.rectangle([13, 4, 18, 8], fill=(140, 100, 60, 255))
    # Highlight
    draw.ellipse([10, 14, 14, 20], fill=(80, 120, 220, 255))
    # Sparkle
    draw.point((18, 16), fill=(200, 220, 255, 255))
    draw.point((17, 17), fill=(200, 220, 255, 255))


def draw_item_sword(draw):
    """Basic sword."""
    # Blade
    draw.polygon([(15, 2), (18, 2), (18, 20), (15, 20)], fill=(180, 180, 200, 255))
    # Blade edge highlight
    draw.line([(15, 2), (15, 20)], fill=(220, 220, 240, 255), width=1)
    # Point
    draw.polygon([(15, 2), (16.5, 0), (18, 2)], fill=(180, 180, 200, 255))
    # Guard
    draw.rectangle([10, 20, 22, 23], fill=(140, 120, 60, 255))
    # Grip
    draw.rectangle([14, 23, 18, 30], fill=(80, 60, 40, 255))
    # Grip wrap
    draw.line([(14, 25), (18, 25)], fill=(100, 80, 50, 255), width=1)
    draw.line([(14, 28), (18, 28)], fill=(100, 80, 50, 255), width=1)


def draw_item_shield(draw):
    """Basic shield."""
    # Shield shape
    draw.polygon(
        [(16, 2), (26, 8), (26, 20), (16, 28), (6, 20), (6, 8)], fill=(100, 80, 60, 255)
    )
    # Metal rim
    draw.polygon(
        [(16, 2), (26, 8), (26, 20), (16, 28), (6, 20), (6, 8)],
        outline=(160, 140, 100, 255),
    )
    # Center emblem
    draw.ellipse([11, 11, 21, 21], fill=(140, 120, 80, 255))
    draw.ellipse([13, 13, 19, 19], fill=(180, 160, 100, 255))


def draw_item_key(draw):
    """Gold key."""
    # Key ring (top)
    draw.ellipse([10, 4, 21, 15], fill=(200, 180, 60, 255), outline=(160, 140, 40, 255))
    draw.ellipse([13, 7, 18, 12], fill=(0, 0, 0, 0))  # Hole
    # Key shaft
    draw.rectangle([14, 14, 17, 26], fill=(200, 180, 60, 255))
    # Key teeth
    draw.rectangle([17, 22, 22, 24], fill=(200, 180, 60, 255))
    draw.rectangle([17, 18, 20, 20], fill=(200, 180, 60, 255))


def draw_item_gold(draw):
    """Gold coins."""
    # Stack of coins
    draw.ellipse([6, 18, 18, 26], fill=(180, 160, 40, 255))
    draw.ellipse([6, 16, 18, 24], fill=(200, 180, 60, 255))
    draw.ellipse([10, 12, 22, 20], fill=(180, 160, 40, 255))
    draw.ellipse([10, 10, 22, 18], fill=(200, 180, 60, 255))
    draw.ellipse([14, 6, 26, 14], fill=(180, 160, 40, 255))
    draw.ellipse([14, 4, 26, 12], fill=(220, 200, 80, 255))
    # $ symbol on top coin
    draw.text((18, 4), "$", fill=(160, 140, 40, 255))


# ============================================================================
# Main
# ============================================================================


def main():
    import os

    os.makedirs("sprites", exist_ok=True)

    # Terrain tiles
    create_sprite(draw_floor, "tile_floor.png")
    create_sprite(draw_wall, "tile_wall.png")
    create_sprite(draw_void, "tile_void.png")
    create_sprite(draw_water, "tile_water.png")
    create_sprite(draw_custom, "tile_custom.png")

    # Actors
    create_sprite(draw_player, "actor_player.png")
    create_sprite(draw_enemy_goblin, "actor_goblin.png")
    create_sprite(draw_enemy_skeleton, "actor_skeleton.png")
    create_sprite(draw_enemy_slime, "actor_slime.png")

    # Props
    create_sprite(draw_door_closed, "prop_door_closed.png")
    create_sprite(draw_door_open, "prop_door_open.png")
    create_sprite(draw_switch_off, "prop_switch_off.png")
    create_sprite(draw_switch_on, "prop_switch_on.png")
    create_sprite(draw_hazard, "prop_hazard.png")

    # Items
    create_sprite(draw_item_potion_health, "item_potion_health.png")
    create_sprite(draw_item_potion_mana, "item_potion_mana.png")
    create_sprite(draw_item_sword, "item_sword.png")
    create_sprite(draw_item_shield, "item_shield.png")
    create_sprite(draw_item_key, "item_key.png")
    create_sprite(draw_item_gold, "item_gold.png")

    print("\nAll sprites generated successfully!")


if __name__ == "__main__":
    main()
