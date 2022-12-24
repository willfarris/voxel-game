pub trait VertexBufferObject {
    fn generate(&mut self, n: i32);
    fn bind(&mut self);
}

impl VertexBufferObject for u32 {
    fn generate(&mut self, n: i32) {
        unsafe {
            gl::GenBuffers(n, self);
            assert_eq!(*self, 0);
        }
    }

    fn bind(&mut self) {
        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, *self);
        }
    }
}