use caevern_sdk::game;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    game::log("HELLO WORLD FROM CAEVERN POLYDURAL");
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    game::log(&format!("I'm a module... {}", dt));
}
