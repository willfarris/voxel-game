pub struct Depthbuffer {
    pub(crate) id: u32,
}


impl Depthbuffer {
    pub fn new(width: i32, height: i32) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenRenderbuffers(1, &mut id);
        }
        let depthbuffer = Self {
            id,
        };
        depthbuffer.bind();

        let depth_bits = gl::DEPTH_COMPONENT24;

        unsafe {
            gl::RenderbufferStorage(gl::RENDERBUFFER, depth_bits, width, height);
        }
        depthbuffer.unbind();
        depthbuffer
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindRenderbuffer(gl::RENDERBUFFER, 0);
        }
    }
}