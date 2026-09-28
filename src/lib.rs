use caevern_sdk::game;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    game::log("HELLO WORLD FROM CAEVERN POLYDURAL");
    game::create_mesh_object(
        &[0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        &[0, 1, 2, 2, 1, 3],
    );
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    game::log(&format!("I'm a module... {}", dt));
}
