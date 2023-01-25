use std::ffi::c_void;

pub enum TextureFormat {
    Float,
    Color,
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

    pub fn empty(width: i32, height: i32, format: TextureFormat) -> Self {
        let mut id = 0;
        unsafe {
            gl::GenTextures(1, &mut id);
        }
        let texture = Texture { id };
        let (internalformat, format, typeformat) = match format {
            TextureFormat::Float => (gl::RGB16F as i32, gl::RGB, gl::FLOAT),
            TextureFormat::Color => (gl::RGBA as i32, gl::RGBA, gl::UNSIGNED_BYTE),
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
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
        }
        texture.unbind();
        texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }
}
