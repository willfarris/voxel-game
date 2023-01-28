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
            let attachment = gl::COLOR_ATTACHMENT0 + i as u32;
            println!("{}", attachment);
            draw_buffers.push(attachment);
        }
        
        unsafe {
            gl::DrawBuffers(
                draw_buffers.len() as i32,
                draw_buffers.as_slice() as *const [u32] as *const u32,
            );
            
            //TODO: make this its own struct like Texture and pass as a parameter
            let mut depth_id = 0;
            gl::GenRenderbuffers(1, &mut depth_id);
            gl::BindRenderbuffer(gl::RENDERBUFFER, depth_id);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT24, 1600, 900);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depth_id);
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


    pub fn bind_render_textures_to_current_fb(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.textures.get("position").unwrap().id);
            
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.textures.get("normal").unwrap().id);

            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.textures.get("albedo").unwrap().id);
        }
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
