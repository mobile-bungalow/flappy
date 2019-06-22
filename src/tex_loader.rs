use gfx_graphics::*;

static TEXLIST: [&str; 8] = [
    //new background that does not contain ground
    "assets/bg_new.png",
    "assets/bird.png",
    "assets/seg.png",
    "assets/tip.png",
    "assets/bird_up.png",
    "assets/gameover.png",
    "assets/message.png",
    "assets/base.png",
];

/// The map of all textures that OPengl needs in order to
/// render background, bird, pipe
pub struct AssetMap {
    pub bg_tex: Texture<gfx_device_gl::Resources>,
    pub bird_tex: Texture<gfx_device_gl::Resources>,
    pub seg_tex: Texture<gfx_device_gl::Resources>,
    pub tip_tex: Texture<gfx_device_gl::Resources>,
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
            .map(move |path| {
                Texture::from_path(ctx, path, Flip::None, &TextureSettings::new()).unwrap()
            })
            .collect();

        AssetMap {
            bg_tex: tex_list[0].clone(),
            bird_tex: tex_list[1].clone(),
            seg_tex: tex_list[2].clone(),
            tip_tex: tex_list[3].clone(),
            bird_up_tex: tex_list[4].clone(),
            game_over_tex: tex_list[5].clone(),
            start_tex: tex_list[6].clone(),
            ground_tex: tex_list[7].clone(),
        }
    }
}
