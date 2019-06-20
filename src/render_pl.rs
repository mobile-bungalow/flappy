use piston_window::*;

static TEXLIST: [&str; 4] = ["assets/bg.png","assets/bird.png","assets/seg.png","assets/tip.png"];

pub struct AssetMap {
    pub bg_tex: G2dTexture,
}

impl AssetMap {
    
    pub fn load_assets(window: &mut PistonWindow) -> Self {
        let bg_tex: G2dTexture = Texture::from_path(
            &mut window.create_texture_context(),
            TEXLIST[0],
            Flip::None,
            &TextureSettings::new(),
        )
        .unwrap();

        AssetMap { bg_tex }
    }

}

