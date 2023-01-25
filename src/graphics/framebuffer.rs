use std::collections::HashMap;

use super::texture::Texture;

pub(crate) struct Framebuffer {
    id: u32,
    textures: HashMap<String, Texture>,
}

impl Framebuffer {
    pub fn with_textures(textures: Vec<(String, Texture)>) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenFramebuffers(1, &mut id);
        }
        let mut framebuffer = Self {
            id,
            textures: HashMap::new(),
        };

        framebuffer.bind();
        let mut draw_buffers = Vec::new();
        for (i, (name, texture)) in textures.into_iter().enumerate() {
            unsafe {
                gl::FramebufferTexture2D(
                    gl::FRAMEBUFFER,
                    gl::COLOR_ATTACHMENT0 + i as u32,
                    gl::TEXTURE_2D,
                    texture.id,
                    0,
                );
            }
            framebuffer.textures.insert(name, texture);
            draw_buffers.push(gl::COLOR_ATTACHMENT0 + i as u32);
        }
        unsafe {
            gl::DrawBuffers(
                draw_buffers.len() as i32,
                draw_buffers.as_slice() as *const [u32] as *const u32,
            );
        }

        let fb_status = unsafe { gl::CheckFramebufferStatus(gl::FRAMEBUFFER) };
        if fb_status != gl::FRAMEBUFFER_COMPLETE {
            #[cfg(target_feature = "android-lib")]
            {
                debug!(
                    "Could not setup framebuffer: glCheckFramebufferStatus() returned {}",
                    fb_status
                );
            }
            panic!("Could not setup framebuffer! (error: {})", fb_status);
        }

        framebuffer.unbind();
        framebuffer
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}
