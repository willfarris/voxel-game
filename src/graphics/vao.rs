pub trait VertexAttributeObject {
    fn generate(&mut self, n: i32);
    fn bind(&mut self);
}

impl VertexAttributeObject for u32 {
    fn generate(&mut self, n: i32) {
        unsafe {
            gl::GenVertexArrays(n, self);
            assert_eq!(*self, 0);
        }
    }

    fn bind(&mut self) {
        unsafe {
            gl::BindVertexArray(*self);
        }
    }
}