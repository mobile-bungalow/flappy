extern crate image;

use gfx_graphics::*;


static TEXLIST: [&[u8]; 7] = [
    //new background that does not contain ground
    include_bytes!("assets/bg_new.png"),
    include_bytes!("assets/bird.png"),
    include_bytes!("assets/pipe.png"),
    include_bytes!("assets/bird_up.png"),
    include_bytes!("assets/gameover.png"),
    include_bytes!("assets/message.png"),
    include_bytes!("assets/base.png"),
];

/// The map of all textures that OPengl needs in order to
/// render background, bird, pipe
pub struct AssetMap {
    pub bg_tex: Texture<gfx_device_gl::Resources>,
    pub bird_tex: Texture<gfx_device_gl::Resources>,
    pub pipe_tex: Texture<gfx_device_gl::Resources>,
    pub bird_up_tex: Texture<gfx_device_gl::Resources>,
    pub game_over_tex: Texture<gfx_device_gl::Resources>,
    pub start_tex: Texture<gfx_device_gl::Resources>,
    pub ground_tex: Texture<gfx_device_gl::Resources>,
}

impl AssetMap {
    //function to load assets
    pub fn load_assets(
        ctx: &mut gfx_graphics::TextureContext<
            gfx_device_gl::Factory,
            gfx_device_gl::Resources,
            gfx_device_gl::CommandBuffer,
        >,
    ) -> Self {

        let tex_list: Vec<Texture<gfx_device_gl::Resources>> = TEXLIST
            .iter()
            .map(move |img| {
                let img = image::load_from_memory(img);
                let img = match img {
                    Ok(image::DynamicImage::ImageRgba8(img)) => img,
                    Ok(img) => img.to_rgba(),
                    Err(e) => panic!("{}", e),
                };
                Texture::from_image(ctx, &img, &TextureSettings::new()).unwrap()
            })
            .collect();


        AssetMap {
            bg_tex: tex_list[0].clone(),
            bird_tex: tex_list[1].clone(),
            pipe_tex: tex_list[2].clone(),
            bird_up_tex: tex_list[3].clone(),
            game_over_tex: tex_list[4].clone(),
            start_tex: tex_list[5].clone(),
            ground_tex: tex_list[6].clone(),
        }
    }
}
