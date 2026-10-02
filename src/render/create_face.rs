pub fn create_face(vertices: &mut Vec<[f64; 3]>, uvs: &mut Vec<[f32; 2]>, normals: &mut Vec<[i8; 3]>, cube_vertices: &Vec<[f64; 3]>, face_indices: [usize; 4], face_normal: [i8; 3], uv_x: f32, uv_y: f32, atlas_size: f32, uv_element: [f32; 4]) {
    vertices.extend([
        cube_vertices[face_indices[0]], cube_vertices[face_indices[1]],
        cube_vertices[face_indices[2]], cube_vertices[face_indices[2]],
        cube_vertices[face_indices[1]], cube_vertices[face_indices[3]]
    ]);

    let atlas_x = 1.0 / atlas_size * uv_x;
    let atlas_y = 1.0 / atlas_size * uv_y;

    uvs.push([
        (0.0 + (uv_element[0] / 16.0)) / atlas_size + atlas_x,
        (1.0 - (1.0 - uv_element[3] / 16.0)) / atlas_size + atlas_y]);
    uvs.push([
        (1.0 - (1.0 - uv_element[2] / 16.0)) / atlas_size + atlas_x,
        (1.0 - (1.0 - uv_element[3] / 16.0)) / atlas_size + atlas_y]);
    uvs.push([
        (0.0 + (uv_element[0] / 16.0)) / atlas_size + atlas_x,
        (0.0 + (uv_element[1] / 16.0)) / atlas_size + atlas_y]);
    uvs.push([
        (0.0 + (uv_element[0] / 16.0)) / atlas_size + atlas_x,
        (0.0 + (uv_element[1] / 16.0)) / atlas_size + atlas_y]);
    uvs.push([
        (1.0 - (1.0 - uv_element[2] / 16.0)) / atlas_size + atlas_x,
        (1.0 - (1.0 - uv_element[3] / 16.0)) / atlas_size + atlas_y]);
    uvs.push([
        (1.0 - (1.0 - uv_element[2] / 16.0)) / atlas_size + atlas_x,
        (0.0 + (uv_element[1] / 16.0)) / atlas_size + atlas_y]);

    normals.extend([face_normal, face_normal, face_normal, face_normal, face_normal, face_normal]);
}
