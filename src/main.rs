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
    display::{object::{Graphics, Tag, self, Object}, Priority},
    include_aseprite,
};

const GRAPHICS: &Graphics = include_aseprite!("gfx/player.aseprite");
const GRAPHICS_TABLE: &Graphics = include_aseprite!("gfx/table.aseprite");

const PLAYER: &Tag = GRAPHICS.tags().get("Player");
const TABLE_CORNER: &Tag = GRAPHICS_TABLE.tags().get("Table Corner");
const TABLE_TOP: &Tag = GRAPHICS_TABLE.tags().get("Table Top");
const TABLE_Y: u16 = 96;

// The main function must take 1 arguments and never return. The agb::entry decorator
// ensures that everything is in order. `agb` will call this after setting up the stack
// and interrupt handlers correctly. It will also handle creating the `Gba` struct for you.
#[agb::entry]
fn main(mut gba: agb::Gba) -> ! {
    let object = gba.display.object.get_managed();
    let mut player = object.object_sprite(PLAYER.sprite(0));
    let mut table_corner_right = object.object_sprite(TABLE_CORNER.sprite(0));
    let mut table_corner_left = object.object_sprite(TABLE_CORNER.sprite(0));
    let mut table_top = object.object_sprite(TABLE_TOP.sprite(0));

    player.set_x(50).set_y(50).show();
    table_corner_right.set_x(0).set_y(TABLE_Y).set_hflip(true).show();
    table_top.set_x(64).set_y(TABLE_Y).show();
    table_corner_left.set_x(64*2).set_y(TABLE_Y).show();



    let mut player_x = 50;
    let mut player_y = 50;
    let mut x_velocity = 0;
    let mut y_velocity = 0;

    let mut input = agb::input::ButtonController::new();

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

        agb::display::busy_wait_for_vblank();
        object.commit();

        // We must call input.update() every frame otherwise it won't update based
        // on the actual button press state.
        input.update();
    }
}
