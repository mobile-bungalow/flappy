use graphics::ImageSize;
use opengl_graphics::Texture;
use piston_window::*;

static TEXLIST: [&str; 4] = [
    "assets/bg.png",
    "assets/bird.png",
    "assets/seg.png",
    "assets/tip.png",
];

/// The map of all textures that OPengl needs in order to
/// render background, bird, pipe
pub struct AssetMap {
    pub bg_tex: G2dTexture,
    pub bird_tex: Texture,
    pub seg_tex: G2dTexture,
    pub tip_tex: G2dTexture,
}

impl AssetMap {
    // function to load assets
    pub fn load_assets(window: &mut PistonWindow) -> Self {
        let tex_list: Vec<G2dTexture> = TEXLIST
            .iter()
            .map(move |path| {
                piston_window::Texture::from_path(
                    &mut window.create_texture_context(),
                    path,
                    Flip::None,
                    &TextureSettings::new(),
                )
                .unwrap()
            })
            .collect();

        let bird_tex =
            opengl_graphics::Texture::from_path("assets/bird.png", &TextureSettings::new())
                .unwrap();

        AssetMap {
            bg_tex: tex_list[0].clone(),
            bird_tex: bird_tex,
            seg_tex: tex_list[2].clone(),
            tip_tex: tex_list[3].clone(),
        }
    }
}
