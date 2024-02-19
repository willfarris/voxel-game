use cgmath::{Matrix4, Vector2};

use crate::c_str;
use crate::{engine::Engine, player::camera::perspective_matrix};

use crate::graphics::{
    depthbuffer::Depthbuffer,
    framebuffer::Framebuffer,
    mesh::FULLSCREEN_QUAD,
    resources::GLRenderable,
    shader::Shader,
    source::{COMPOSITE_FRAG_SRC, LIGHTING_FRAG_SRC, POSTPROCESS_FRAG_SRC, SCREENQUAD_VERT_SRC},
    texture::{Texture, TextureFormat},
    uniform::Uniform,
};

impl Engine {
    pub fn init_gl(&mut self, width: i32, height: i32) {
        self.pause();

        #[cfg(target_os = "android")]
        {
            gl::load_with(|s| unsafe { std::mem::transmute(egli::egl::get_proc_address(s)) });
            debug!("Loaded GL pointer");
        }

        {
            let mut gl_resources = self.gl_resources.write().unwrap();
            gl_resources.invalidate_resources();
        }

        self.width.store(width, std::sync::atomic::Ordering::Relaxed);
        self.height.store(height, std::sync::atomic::Ordering::Relaxed);
        let mut framebuffer_id = 0;
        unsafe {
            gl::Enable(gl::DEPTH_TEST);
            gl::DepthFunc(gl::LEQUAL);

            gl::Enable(gl::CULL_FACE);
            gl::CullFace(gl::BACK);

            gl::FrontFace(gl::CW);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut framebuffer_id);
        }

        /*let ssao_program = graphics::shader::Shader::new(SCREENQUAD_VERT_SRC, SSAO_FRAG_SRC).unwrap();

        ssao_program.use_program();
        let mut ssao_kernel = [Vector3::zero(); 64];
        for (i, s) in &mut ssao_kernel.iter_mut().enumerate() {
            let sample: Vector3<f32> = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>(),
            ).normalize() * rand::random();
            let sample_name_rust = format!("samples[{}]", i);
            let sample_name = std::ffi::CString::new(sample_name_rust.as_str()).unwrap();
            ssao_program.set_vec3(&sample_name, &sample);
            *s = sample;
        }

        const SSAO_NOISE_SIZE: usize = 4;
        ssao_program.set_float(unsafe {c_str!("ssao_noise_size")}, SSAO_NOISE_SIZE as f32);
        let mut ssao_noise = [Vector3::zero(); SSAO_NOISE_SIZE*SSAO_NOISE_SIZE];
        for pixel in ssao_noise.iter_mut() {
            *pixel = Vector3::new(
                rand::random::<f32>() * 2.0 - 1.0,
                rand::random::<f32>() * 2.0 - 1.0,
                0.0
            );
        }
        let ssao_noise_texture = Texture::from_vector3_array(&ssao_noise, SSAO_NOISE_SIZE as i32, SSAO_NOISE_SIZE as i32);*/

        // Deferred rendering pass
        let gbuffer_position = Texture::empty(width, height, TextureFormat::Float);
        let gbuffer_normal = Texture::empty(width, height, TextureFormat::Float);
        let gbuffer_albedo = Texture::empty(width, height, TextureFormat::Color);
        let gbuffer_depthbuffer = Depthbuffer::new(width, height);
        let gbuffer_textures = &[
            ("position", gbuffer_position),
            ("normal", gbuffer_normal),
            ("albedo", gbuffer_albedo),
        ];
        let gbuffer = Framebuffer::with_textures(gbuffer_textures, Some(gbuffer_depthbuffer));

        // Lighting pass
        let lighting_program = Shader::new(SCREENQUAD_VERT_SRC, LIGHTING_FRAG_SRC).unwrap();
        let lighting_color = Texture::empty(width, height, TextureFormat::Color);
        let lighting_depthbuffer = Depthbuffer::new(width, height);
        let lighting_framebuffer =
            Framebuffer::with_textures(&[("color", lighting_color)], Some(lighting_depthbuffer));

        // Composite lit scene w/ skybox
        let composite_program = Shader::new(SCREENQUAD_VERT_SRC, COMPOSITE_FRAG_SRC).unwrap();
        let composite_color = Texture::empty(width, height, TextureFormat::Color);
        let composite_depthbuffer = Depthbuffer::new(width, height);
        let composite_framebuffer =
            Framebuffer::with_textures(&[("color", composite_color)], Some(composite_depthbuffer));

        // Postprocessing shader
        let postprocess_program = Shader::new(SCREENQUAD_VERT_SRC, POSTPROCESS_FRAG_SRC).unwrap();

        {
            let mut gl_resources = self.gl_resources.write().unwrap();

            // Register created shaders & FBOs
            gl_resources.add_framebuffer("gbuffer", gbuffer);
            gl_resources.add_framebuffer("lighting", lighting_framebuffer);
            gl_resources.add_framebuffer("composite", composite_framebuffer);

            gl_resources.add_shader("lighting", lighting_program);
            gl_resources.add_shader("composite", composite_program);
            gl_resources.add_shader("postprocess", postprocess_program);

            // Graphics resources for world components

            gl_resources.add_vao(
                "screenquad".to_string(),
                Box::new(Vec::from(FULLSCREEN_QUAD)),
            );

            self.skybox.write().unwrap().init_gl_resources(&mut gl_resources);

            self.terrain
                .write()
                .unwrap()
                .init_gl_resources(&mut gl_resources);

            for entity in self.entities.iter() {
                entity.init_gl_resources(&mut gl_resources);
            }
        }

        self.resume();
    }

    pub fn reset_gl_resources(&mut self) {
        self.gl_resources.write().unwrap().invalidate_resources();
    }

    pub fn draw(&mut self) {
        let player = self.player.read().unwrap();
        let terrain = self.terrain.read().unwrap();
        {
            self.gl_resources
                .write()
                .unwrap()
                .process_vao_buffer_updates(2);
        }

        let gl_resources = self.gl_resources.read().unwrap();
        let width = self.width.load(std::sync::atomic::Ordering::Relaxed);
        let height = self.height.load(std::sync::atomic::Ordering::Relaxed);
        let render_distance = self.render_distance.load(std::sync::atomic::Ordering::Relaxed);
        let elapsed_time = {
            self.engine_state.read().unwrap().elapsed_time
        };

        let screenquad = gl_resources.get_vao("screenquad").unwrap();

        /* ************************************ *
         * Render terrain + entities to GBuffer *
         * ************************************ */
        let gbuffer_fbo = gl_resources.get_framebuffer("gbuffer").unwrap();
        gbuffer_fbo.bind();

        unsafe {
            gl::Viewport(0, 0, width, height);
            gl::ClearColor(0.0, 0.0, 0.0, 0.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let perspective_matrix =
            perspective_matrix(width, height, self.render_distance.load(std::sync::atomic::Ordering::Relaxed) as f32);
        let view_matrix = player.camera_view_matrix();

        let geometry_uniforms: Vec<(&str, Box<dyn Uniform>)> = vec![
            ("perspective_matrix", Box::new(perspective_matrix)),
            ("view_matrix", Box::new(view_matrix)),
            ("time", Box::new(elapsed_time.as_secs_f32())),
        ];

        terrain.draw(&gl_resources, &geometry_uniforms);

        for entity in &self.entities {
            entity.draw(&gl_resources, &geometry_uniforms);
        }

        gbuffer_fbo.unbind();

        /* ********************************************* *
         * Calculate lighting for GBuffer-rendered items *
         * ********************************************* */

        let lighting_fbo = gl_resources.get_framebuffer("lighting").unwrap();
        let lighting_program = gl_resources.get_shader("lighting").unwrap();
        lighting_fbo.bind();
        lighting_fbo.clear_color_and_depth();

        gbuffer_fbo.bind_render_textures_to_current_fb(&[
            ("albedo", 1),
            ("position", 2),
            ("normal", 3),
        ]);

        lighting_program.use_program();
        lighting_program.set_texture(unsafe { c_str!("albedo") }, 1);
        lighting_program.set_texture(unsafe { c_str!("position") }, 2);
        lighting_program.set_texture(unsafe { c_str!("normal") }, 3);

        Vector2::new(width as f32, height as f32)
            .set_as_uniform(lighting_program, "resolution");

        screenquad.draw();

        lighting_fbo.unbind();

        /* ****************************************** *
         * Render lit scene + skybox on Composite FBO *
         * ****************************************** */

        let composite_fbo = gl_resources.get_framebuffer("composite").unwrap();
        composite_fbo.bind();
        composite_fbo.clear_color_and_depth();

        let composite_program = gl_resources.get_shader("composite").unwrap();
        composite_program.use_program();
        lighting_fbo.bind_render_textures_to_current_fb(&[("color", 0)]);
        composite_program.set_texture(unsafe { c_str!("lighting_output") }, 0);

        screenquad.draw();

        gbuffer_fbo.blit_depth_to_fbo(composite_fbo, width, height);

        // Draw skybox
        let skybox_model_matrix = Matrix4::from_translation(player.camera.position)
            * Matrix4::from_scale(render_distance as f32 * 16.0 * 2.0);
        let _geometry_uniforms: Vec<(&str, Box<dyn Uniform>)> = vec![
            ("model_matrix", Box::new(skybox_model_matrix)),
            ("perspective_matrix", Box::new(perspective_matrix)),
            ("view_matrix", Box::new(view_matrix)),
        ];
        //self.skybox.read().unwrap().draw(&gl_resources, &geometry_uniforms);

        composite_fbo.unbind();

        /**********************************
         * Render Composite FBO to screen *
         * ******************************** */

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        let postprocess_shader = gl_resources.get_shader("postprocess").unwrap();
        postprocess_shader.use_program();

        composite_fbo.bind_render_textures_to_current_fb(&[("color", 0)]);
        postprocess_shader.set_texture(unsafe { c_str!("composite_output") }, 0);

        screenquad.draw();
    }
}
