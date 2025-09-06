use macroquad::prelude::*;

pub(crate) fn draw_circular_sector(
    x: f32,
    y: f32,
    n: u16,
    radius: f32,
    rotation: f32,
    arc: f32,
    color: Color,
    gl: &mut InternalGlContext,
) {
    if arc <= 0. || radius <= 0. || n == 0 {
        return;
    }
    let step = arc / (n as f32);
    let center = Vertex::new(x, y, 0., 0., 0., color);
    let center_idx = 0;
    let mut vertices = vec![center];
    let mut indices = vec![center_idx];
    let mut previous_vertex_idx = None;
    let mut theta = rotation;
    let mut i = 0;
    while i <= n {
        let new_vertex = Vertex::new(
            radius * theta.cos() + x,
            radius * theta.sin() + y,
            0.,
            0.,
            0.,
            color,
        );
        vertices.push(new_vertex);
        let new_vertex_idx = (vertices.len() - 1) as u16;
        if let Some(previous_vertex_idx) = previous_vertex_idx {
            indices.extend_from_slice(&[center_idx, new_vertex_idx, previous_vertex_idx]);
        }
        previous_vertex_idx = Some(new_vertex_idx);
        theta += step;
        i += 1;
    }
    gl.flush();
    gl.quad_gl.texture(None);
    gl.quad_gl.draw_mode(DrawMode::Triangles);
    gl.quad_gl.geometry(&vertices, &indices);
    gl.flush();
}
