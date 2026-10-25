use std::sync::{
    Arc, Weak,
    atomic::{AtomicU8, Ordering},
};

const PENDING: u8 = 0;
const COMMITTED: u8 = 1;
const DISCARDED: u8 = 2;

#[derive(Debug, Clone)]
pub(super) struct SelectionCommit(Arc<AtomicU8>);

#[derive(Default)]
struct PendingPass {
    pending: Vec<Weak<AtomicU8>>,
    recent: Vec<Weak<AtomicU8>>,
    recent_frame: u64,
}

impl SelectionCommit {
    pub(super) fn committed() -> Self {
        Self(Arc::new(AtomicU8::new(COMMITTED)))
    }

    pub(super) fn is_committed(&self) -> bool {
        self.0.load(Ordering::Acquire) == COMMITTED
    }

    pub(super) fn watch(&self, context: &egui::Context) {
        self.0.store(PENDING, Ordering::Release);
        let key = egui::Id::new("kuc.selection-pass-commit");
        let (passes, install) = context.data_mut(|data| {
            if let Some(passes) = data.get_temp::<Arc<egui::mutex::Mutex<PendingPass>>>(key) {
                (passes, false)
            } else {
                let passes = Arc::new(egui::mutex::Mutex::new(PendingPass::default()));
                data.insert_temp(key, Arc::clone(&passes));
                (passes, true)
            }
        });
        passes.lock().pending.push(Arc::downgrade(&self.0));
        if install {
            let begin = Arc::clone(&passes);
            context.on_begin_pass(
                "kuc selection discard",
                Arc::new(move |ui| {
                    let mut passes = begin.lock();
                    let discarded = passes.recent_frame == ui.ctx().cumulative_frame_nr();
                    for observation in passes.recent.drain(..).filter_map(|value| value.upgrade()) {
                        if discarded {
                            observation.store(DISCARDED, Ordering::Release);
                        }
                    }
                }),
            );
            context.on_end_pass(
                "kuc selection commit",
                Arc::new(move |ui| {
                    let mut passes = passes.lock();
                    let status = if ui.ctx().will_discard() {
                        DISCARDED
                    } else {
                        COMMITTED
                    };
                    passes.recent_frame = ui.ctx().cumulative_frame_nr();
                    let pending = std::mem::take(&mut passes.pending);
                    for observation in pending.iter().filter_map(Weak::upgrade) {
                        observation.store(status, Ordering::Release);
                    }
                    passes.recent = pending;
                }),
            );
        }
    }
}
