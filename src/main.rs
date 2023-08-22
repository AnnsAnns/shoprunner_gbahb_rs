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
    include_background_gfx,
    include_aseprite, include_font,
};
use agb::display::tiled::TileSet;

use agb::display::tiled::TiledMap;
use agb::display::Font;
use agb::input::Button;
use agb::println;
use core::{fmt::Write, u16::MAX};
use agb::display::tiled::TileSetting;

const NPC: &Graphics = include_aseprite!("gfx/npc.aseprite");
const OBJECTS: &Graphics = include_aseprite!("gfx/objects.aseprite");

include_background_gfx!(backgrounds, "121105", background => "gfx/newgraphs.aseprite");

const NPC_PLAYER: &Tag = NPC.tags().get("npc player");
const TELLER: &Tag = OBJECTS.tags().get("Teller");
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

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let (gfx, mut vram) = gba.display.video.tiled0();

    let object = gba.display.object.get_managed();
    let mut npc_player = object.object_sprite(NPC_PLAYER.sprite(0));
    let mut teller = object.object_sprite(TELLER.sprite(0));

    npc_player.set_x(128).set_y(24).show();
    let mut tellerPos = 100;
    teller.set_x(0).set_y(tellerPos).show();

    vram.set_background_palettes(backgrounds::PALETTES);
    let tile_set = TileSet::new(
        backgrounds::background.tiles,
        agb::display::tiled::TileFormat::FourBpp,
    );

    
    let mut bg = gfx.background(
        Priority::P0,
        agb::display::tiled::RegularBackgroundSize::Background32x32,
        agb::display::tiled::TileFormat::FourBpp,
    );

    for x in 0..30u16 {
        for y in 0..20u16 {
            let tile_id = y * 30 + x;
            bg.set_tile(
                &mut vram,
                (x, y).into(),
                &tile_set,
                TileSetting::new(
                    tile_id,
                    false,
                    false,
                    backgrounds::background.palette_assignments[tile_id as usize],
                ),
            );
        }
    }

    bg.commit(&mut vram);
    bg.show();

    let vblank = agb::interrupt::VBlank::get();

    let mut input = agb::input::ButtonController::new();
    let mut isSelected = false;
    let mut remainingAnimationDistance = 0;

    loop {
        if remainingAnimationDistance > 0 {
            remainingAnimationDistance -= 1;
            tellerPos -= 1;
            teller.set_y(tellerPos).show();
        }

        if input.is_just_pressed(Button::A) {
            isSelected = !isSelected;
            teller = object.object_sprite(TELLER.sprite(isSelected as usize));
            teller.set_x(0).set_y(tellerPos).show();
        } else if input.is_just_pressed(Button::UP) {
            remainingAnimationDistance = 50;
        }

        vblank.wait_for_vblank();
        object.commit();

        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
    }
}
