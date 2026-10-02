use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ElementFaceTexture {
    pub uv: [f32; 4]
}
#[derive(Deserialize, Debug, Clone)]
pub struct ElementFaces {
    pub north: ElementFaceTexture,
    pub east: ElementFaceTexture,
    pub south: ElementFaceTexture,
    pub west: ElementFaceTexture,
    pub up: ElementFaceTexture,
    pub down: ElementFaceTexture,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Element {
    pub from: [f64; 3],
    pub to: [f64; 3],
    pub faces: ElementFaces
}
