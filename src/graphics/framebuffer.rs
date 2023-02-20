use std::collections::HashMap;

use super::{texture::Texture, depthbuffer::{Depthbuffer, self}};

pub struct Framebuffer {
    id: u32,
    textures: HashMap<&'static str, Texture>,
}

impl Framebuffer {
    pub fn with_textures(textures: &[(&'static str, Texture)], depthbuffer: Option<Depthbuffer>) -> Self {
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

            framebuffer.textures.insert(name, *texture);
            let attachment = gl::COLOR_ATTACHMENT0 + i as u32;
            draw_buffers.push(attachment);
        }
        
        unsafe {
            gl::DrawBuffers(
                draw_buffers.len() as i32,
                draw_buffers.as_slice() as *const [u32] as *const u32,
            );
        }
            
        //TODO: make this its own struct like Texture and pass as a parameter
        if let Some(depthbuffer) = depthbuffer {
            unsafe {
                gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, depthbuffer.id);
            }
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

    pub fn bind_render_textures_to_current_fb(&self, textures: &[(&'static str, u32)]) {
        for (name, texture_index) in textures {
            if let Some(texture) = self.textures.get(name) {
                unsafe {
                    gl::ActiveTexture(gl::TEXTURE0 + texture_index);
                    gl::BindTexture(gl::TEXTURE_2D, texture.id);
                }
            }
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

    pub fn clear_color_and_depth(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn blit_depth_to_fbo(&self, target: &Framebuffer, width: i32, height: i32) {
        unsafe {
            gl::BindFramebuffer(gl::READ_FRAMEBUFFER, self.id);
            gl::BindFramebuffer(gl::DRAW_FRAMEBUFFER, target.id);
            gl::BlitFramebuffer(0, 0, width, height, 0, 0, width, height, 
                              gl::DEPTH_BUFFER_BIT, gl::NEAREST);
        }
    }
}
