use std::collections::{HashMap, VecDeque};

pub(super) const DEFAULT_TEXTURE_CACHE_CAPACITY: usize = 64;

pub(super) struct RgbaTextureCache {
    capacity: usize,
    order: VecDeque<String>,
    textures: HashMap<String, egui::TextureHandle>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cache_recovers_from_missing_order_entry_and_evicts_oldest_texture() {
        let context = egui::Context::default();
        let pixels = [255, 255, 255, 255];
        let mut cache = RgbaTextureCache::new(1);
        let orphan = context.load_texture(
            "orphan",
            egui::ColorImage::from_rgba_unmultiplied([1, 1], &pixels),
            egui::TextureOptions::NEAREST,
        );
        cache.textures.insert("orphan".to_owned(), orphan);
        let _ = cache.texture_for_rgba(&context, "replacement", 1, 1, &pixels);
        assert!(cache.textures.contains_key("orphan"));
        assert!(cache.textures.contains_key("replacement"));

        cache.order.push_front("orphan".to_owned());
        let _ = cache.texture_for_rgba(&context, "newest", 1, 1, &pixels);
        assert!(!cache.textures.contains_key("orphan"));
        assert!(cache.textures.contains_key("newest"));
    }
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
