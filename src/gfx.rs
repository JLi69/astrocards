pub mod models;

pub fn set_default_gl_state() {
    unsafe {
        gl::Enable(gl::BLEND);
        gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
        gl::ClearColor(0.1, 0.1, 0.1, 1.0);
    }
}

//Clear screen
pub fn clear() {
    unsafe {
        gl::Clear(gl::COLOR_BUFFER_BIT);
    }
}

pub fn get_gl_errors() {
    unsafe {
        let mut err = gl::GetError();
        while err != gl::NO_ERROR {
            eprintln!("OpenGL Error: {err}");
            err = gl::GetError();
        }
    }
}
