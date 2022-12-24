use cgmath::Matrix4;

pub trait GLRenderable {
    fn init_gl_resources(&mut self);
    fn draw(&self, perspective_matrix: Matrix4<f32>, view_matrix: Matrix4<f32>, elapsed_time: f32);
}

