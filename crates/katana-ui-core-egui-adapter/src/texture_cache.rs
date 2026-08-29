use std::collections::{HashMap, VecDeque};

pub(super) const DEFAULT_TEXTURE_CACHE_CAPACITY: usize = 64;

pub(super) struct RgbaTextureCache {
    capacity: usize,
    order: VecDeque<String>,
    textures: HashMap<String, egui::TextureHandle>,
}

impl RgbaTextureCache {
    pub(super) fn new(capacity: usize) -> Self {
        Self {
            capacity: capacity.max(1),
            order: VecDeque::new(),
            textures: HashMap::new(),
        }
    }

    pub(super) fn texture_for_rgba(
        &mut self,
        context: &egui::Context,
        identity: &str,
        width: usize,
        height: usize,
        pixels: &[u8],
    ) -> egui::TextureHandle {
        if let Some(texture) = self.textures.get(identity) {
            return texture.clone();
        }
        while self.textures.len() >= self.capacity {
            let Some(expired) = self.order.pop_front() else {
                break;
            };
            self.textures.remove(&expired);
        }
        let texture = context.load_texture(
            identity,
            egui::ColorImage::from_rgba_unmultiplied([width, height], pixels),
            egui::TextureOptions::NEAREST,
        );
        self.order.push_back(identity.to_string());
        self.textures.insert(identity.to_string(), texture.clone());
        texture
    }
}
