use std::ffi::c_void;

use cgmath::Vector3;

pub enum TextureFormat {
    Float,
    Color,
    SingleChannel,
}

#[derive(Clone, Copy, Debug)]
pub struct Texture {
    pub id: u32,
}

impl Texture {
    pub fn from_dynamic_image_bytes(img_bytes: &[u8], format: image::ImageFormat) -> Texture {
        let img = image::load_from_memory_with_format(img_bytes, format)
            .unwrap()
            .flipv();
        let format = match img {
            image::DynamicImage::ImageLuma8(_) => gl::RED,
            image::DynamicImage::ImageLumaA8(_) => gl::RG,
            image::DynamicImage::ImageRgb8(_) => gl::RGB,
            image::DynamicImage::ImageRgba8(_) => gl::RGBA,
            _ => panic!("Unknown image format"),
        };

        let data = img.as_bytes();

        let mut texture_id = 0;

        unsafe {
            gl::GenTextures(1, &mut texture_id);

            gl::BindTexture(gl::TEXTURE_2D, texture_id);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                format as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                format,
                gl::UNSIGNED_BYTE,
                &data[0] as *const u8 as *const c_void,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture { id: texture_id }
    }

    pub fn from_vector3_array(img_bytes: &[Vector3<f32>], width: i32, height: i32) -> Texture {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        let texture = Texture { id };
        texture.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB32F as i32,
                width,
                height,
                0,
                gl::RGB,
                gl::FLOAT,
                &img_bytes[0] as *const Vector3<f32> as *const c_void
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);  
        }
        texture.unbind();
        texture
    }

    pub fn empty(width: i32, height: i32, format: TextureFormat) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        let texture = Texture { id };
        let (internalformat, format, typeformat) = match format {
            TextureFormat::Float => (gl::RGB16F as i32, gl::RGB, gl::FLOAT),
            TextureFormat::Color => (gl::RGBA as i32, gl::RGBA, gl::UNSIGNED_BYTE),
            TextureFormat::SingleChannel => (gl::R16F as i32, gl::RED, gl::FLOAT),
        };
        texture.bind();
        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                internalformat,
                width,
                height,
                0,
                format,
                typeformat,
                std::ptr::null::<std::ffi::c_void>(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        }
        texture.unbind();
        texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn use_as_framebuffer_texture(&self, index: u32) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0 + index);
        }
        self.bind();
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn _delete(&mut self) {
        if self.id != 0 {
            unsafe {
                gl::DeleteTextures(1, &self.id as *const u32);
            }
            self.id = 0;
        }
    }
}
