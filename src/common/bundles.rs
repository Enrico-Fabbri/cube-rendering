pub struct BundleManager {
    bundles: Vec<wgpu::RenderBundle>,
    depth_texture: super::texture::Texture,
}

impl BundleManager {
    pub fn new(device: &wgpu::Device, config: &wgpu::SurfaceConfiguration) -> Self {
        Self {
            bundles: Vec::new(),
            depth_texture: super::texture::Texture::create_depth_texture(
                device,
                config,
                "Depth Texture",
            ),
        }
    }

    pub fn get_bundles(&self) -> &[wgpu::RenderBundle] {
        &*self.bundles
    }

    pub fn push_bundle(&mut self, bundle: wgpu::RenderBundle) {
        self.bundles.push(bundle);
    }

    pub fn _push_bundles(&mut self, bundles: Vec<wgpu::RenderBundle>) {
        for b in bundles {
            self.push_bundle(b);
        }
    }

    pub fn set_depth_texture(&mut self, depth_texture: super::texture::Texture) {
        self.depth_texture = depth_texture;
    }

    pub fn get_depth_texture_view(&self) -> &wgpu::TextureView {
        &self.depth_texture.view
    }
}
