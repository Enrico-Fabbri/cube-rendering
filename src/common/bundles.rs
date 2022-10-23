pub struct BundleManager {
    bundles: Vec<wgpu::RenderBundle>,
}

impl BundleManager {
    pub fn new() -> Self {
        Self {
            bundles: Vec::new(),
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
}
