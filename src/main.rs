// Games made using `agb` are no_std which means you don't have access to the standard
// rust library. This is because the game boy advance doesn't really have an operating
// system, so most of the content of the standard library doesn't apply.
//
// Provided you haven't disabled it, agb does provide an allocator, so it is possible
// to use both the `core` and the `alloc` built in crates.
#![no_std]
// `agb` defines its own `main` function, so you must declare your game's main function
// using the #[agb::entry] proc macro. Failing to do so will cause failure in linking
// which won't be a particularly clear error message.
#![no_main]
// This is required to allow writing tests
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use agb::{
    display::{
        font::{TextRenderer, TextWriter},
        object::{self, Graphics, Object, Tag},
        Priority,
    },
    include_aseprite, include_font,
};

use agb::display::tiled::TiledMap;
use agb::display::Font;
use agb::input::Button;
use agb::println;
use core::{fmt::Write, u16::MAX};

const GRAPHICS: &Graphics = include_aseprite!("gfx/player.aseprite");
const GRAPHICS_TABLE: &Graphics = include_aseprite!("gfx/table.aseprite");
const TEXT_BUBBLE: &Graphics = include_aseprite!("gfx/text.aseprite");
const NPC: &Graphics = include_aseprite!("gfx/npc.aseprite");
const OBJECTS: &Graphics = include_aseprite!("gfx/objects.aseprite");

const PLAYER: &Tag = GRAPHICS.tags().get("Player");
const TABLE_CORNER: &Tag = GRAPHICS_TABLE.tags().get("Table Corner");
const TABLE_TOP: &Tag = GRAPHICS_TABLE.tags().get("Table Top");
const TEXT: &Tag = TEXT_BUBBLE.tags().get("Text");
const BODEN: &Tag = OBJECTS.tags().get("Boden");
const WOOD: &Tag = OBJECTS.tags().get("wood I think");
const TEXT_MIDDLE: &Tag = TEXT_BUBBLE.tags().get("Text Mid");
const TABLE_Y: u16 = 96;
const NPC_PLAYER: &Tag = NPC.tags().get("npc player");
const TOTAL_LINES: usize = 4;
const MAX_CHARS_PER_LINE: usize = 20;

const FONT: Font = include_font!("gfx/yoster.ttf", 12);

fn text_renderer(mut writer: &mut TextWriter<'_, '_>, text: &str) {
    let mut iter = text.split_ascii_whitespace();
    let mut line_size = 0;
    for word in iter {
        if word.contains("\n") {
            line_size = 0;
        }

        if line_size + word.len() > MAX_CHARS_PER_LINE {
            writer.write_char('\n').unwrap();
            line_size = 0;
        }

        writer.write_str(word).unwrap();
        writer.write_char(' ').unwrap();
        line_size += word.len() + 1;
    }
}

const TEXT_LINES: [&str; 11] = [
    "To be fair,\n you have to have a very high\n IQ to understand Rick and Morty.\n",
    "The humour is extremely subtle,\n and without a solid grasp",
    "of theoretical physics most of the jokes will go over a",
    "typical viewer's head.\n",
    "There's also Rick's nihilistic outlook,\n which is deftly",
    "woven into his characterisation-\n his personal philosophy",
    "draws heavily from Narodnaya Volya literature,\n for instance.",
    "The fans understand this stuff;\n they have the intellectual",
    "capacity to truly appreciate the depths of these jokes,",
    "to realise that they're not just funny- they say something",
    "deep about LIFE.\n",
];

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (gfx, mut vram) = gba.display.video.tiled0();

    let object = gba.display.object.get_managed();
    let mut boden = object.object_sprite(BODEN.sprite(0));
    let mut wood_01 = object.object_sprite(WOOD.sprite(0));
    let mut wood_02 = object.object_sprite(WOOD.sprite(0));
    let mut wood_03 = object.object_sprite(WOOD.sprite(0));
    let mut wood_04 = object.object_sprite(WOOD.sprite(0));
    let mut wood_05 = object.object_sprite(WOOD.sprite(0));
    let mut wood_06 = object.object_sprite(WOOD.sprite(0));
    let mut table_corner_right = object.object_sprite(TABLE_CORNER.sprite(0));
    let mut table_corner_left = object.object_sprite(TABLE_CORNER.sprite(0));
    let mut table_top = object.object_sprite(TABLE_TOP.sprite(0));
    let mut bubble_right = object.object_sprite(TEXT.sprite(0));
    let mut bubble_middle = object.object_sprite(TEXT_MIDDLE.sprite(0));
    let mut bubble_left = object.object_sprite(TEXT.sprite(0));
    let mut npc_player = object.object_sprite(NPC_PLAYER.sprite(0));
    let mut player = object.object_sprite(PLAYER.sprite(0));

    boden.set_x(192).set_y(109).show();
    wood_01.set_x(188).set_y(45).show();
    wood_02.set_x(124).set_y(64).set_hflip(true).show();
    wood_03.set_x(60).set_y(64).show();
    wood_04.set_x(188).set_y(0).show();
    wood_05.set_x(0).set_y(64).show();
    table_corner_right
        .set_x(0)
        .set_y(TABLE_Y)
        .set_hflip(true)
        .show();
    table_top.set_x(64).set_y(TABLE_Y).show();
    table_corner_left.set_x(64 * 2).set_y(TABLE_Y).show();
    bubble_right.set_x(128).set_y(0).set_hflip(true).show();
    bubble_middle.set_x(64).set_y(0).show();
    bubble_left.set_x(0).set_y(0).show();
    npc_player.set_x(128).set_y(32).show();
    player.set_x(50).set_y(50).show();

    vram.set_background_palette_raw(&[
        0x0000, 0x0A2A, 0xFFFF, 0x0000, 0xf0f0, 0x0f0f, 0xaaaa, 0x5555, 0x6666, 0x0000, 0x0000,
        0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
    ]);

    let mut bg = gfx.background(
        Priority::P0,
        agb::display::tiled::RegularBackgroundSize::Background32x32,
        agb::display::tiled::TileFormat::FourBpp,
    );

    let background_tile = vram.new_dynamic_tile().fill_with(2);

    for y in 0..20u16 {
        for x in 0..30u16 {
            bg.set_tile(
                &mut vram,
                (x, y).into(),
                &background_tile.tile_set(),
                agb::display::tiled::TileSetting::from_raw(background_tile.tile_index()),
            );
        }
    }

    vram.remove_dynamic_tile(background_tile);

    bg.commit(&mut vram);
    bg.show();

    let vblank = agb::interrupt::VBlank::get();

    let mut player_x = 50;
    let mut player_y = 50;
    let mut x_velocity = 0;
    let mut y_velocity = 0;

    let mut input = agb::input::ButtonController::new();
    let mut frame: u16 = 0;
    let mut text_line = 0;

    loop {
        // This will calculate the new position and enforce the position
        // of the ball remains within the screen
        player_x = (player_x + x_velocity).clamp(0, agb::display::WIDTH - 16);
        player_y = (player_y + y_velocity).clamp(0, agb::display::HEIGHT - 16);

        // x_tri and y_tri describe with -1, 0 and 1 which way the d-pad
        // buttons are being pressed
        x_velocity = input.x_tri() as i32;
        y_velocity = input.y_tri() as i32;

        // Set the position of the ball to match our new calculated position
        player.set_x(player_x as u16).set_y(player_y as u16);

        if input.is_just_pressed(Button::A) && text_line < TEXT_LINES.len() {
            let background_tile = vram.new_dynamic_tile().fill_with(2);

            for y in 0..20u16 {
                for x in 0..30u16 {
                    bg.set_tile(
                        &mut vram,
                        (x, y).into(),
                        &background_tile.tile_set(),
                        agb::display::tiled::TileSetting::from_raw(background_tile.tile_index()),
                    );
                }
            }

            vram.remove_dynamic_tile(background_tile);

            bg.commit(&mut vram);
            bg.show();
            let mut renderer = FONT.render_text((1u16, 1u16).into());
            let mut writer = renderer.writer(1, 2, &mut bg, &mut vram);
            text_renderer(&mut writer, TEXT_LINES[text_line]);
            text_line += 1;
            vblank.wait_for_vblank();
            writer.commit();
        }

        vblank.wait_for_vblank();

        bg.commit(&mut vram);
        object.commit();

        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
        frame += 1;
    }
}
