use caevern_sdk::game;

#[unsafe(no_mangle)]
pub extern "C" fn init() {
    game::log("HELLO WORLD FROM CAEVERN POLYDURAL");
    let object = game::create_mesh_object(
        &[
            -1.0, -1.0,  1.0,
             1.0, -1.0,  1.0,
             1.0,  1.0,  1.0,
            -1.0,  1.0,  1.0,

            -1.0, -1.0, -1.0,
             1.0, -1.0, -1.0,
             1.0,  1.0, -1.0,
            -1.0,  1.0, -1.0,
        ],
        &[
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,

            1.0, 0.0,
            0.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
        ],
        &[
            0, 1, 2,
            2, 3, 0,

            5, 4, 7,
            7, 6, 5,

            4, 0, 3,
            3, 7, 4,

            1, 5, 6,
            6, 2, 1,

            3, 2, 6,
            6, 7, 3,

            4, 5, 1,
            1, 0, 4,
        ],
    );
    game::log(&format!("object: {}", object.get_id()));
}

#[unsafe(no_mangle)]
pub extern "C" fn update(dt: f32) {
    game::log(&format!("I'm a module... {}", dt));
}
