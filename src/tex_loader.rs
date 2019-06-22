use gfx_graphics::*;

static TEXLIST: [&str; 6] = [
    "assets/bg.png",
    "assets/bird.png",
    "assets/seg.png",
    "assets/tip.png",
    "assets/bird_up.png",
    "assets/gameover.png",
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
        }
    }
}
