mod gfx;
mod game;
mod assets;
mod impfile;

use game::{Game, assets::models::draw_elements};
use glfw::{WindowMode, Context};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).expect("Failed to init glfw!");
    //Init window
    let (mut window, events) = glfw.create_window(960, 540, "Astrocards", WindowMode::Windowed)
        .expect("Failed to init window!");
    window.make_current();
    //Load OpenGL
    gl::load_with(|s| glfw.get_proc_address_raw(s));

    //Initialize game
    let mut gamestate = Game::new();
    //Load config
    gamestate.load_config("cfg.impfile");
    //Load assets
    gamestate.load_assets();
   
    gfx::set_default_gl_state();
    while !window.should_close() {
        //Clear screen
        gfx::clear();

        //Draw quad
        gamestate.shaders.use_program("quadshader");
        let quad = gamestate.models.bind("quad2d");
        draw_elements(quad);

        //Print OpenGL errors
        gfx::get_gl_errors();
        //Poll events and swap buffers
        glfw.poll_events();
        window.swap_buffers();
        //Handle events
        game::process_events(&events);
    }
}
