use cgmath::Vector3;

pub trait PhysicsUpdate {
    fn update_physics(&mut self, delta_time: f32);
    fn translate_relative(&mut self, translation: Vector3<f32>);
}