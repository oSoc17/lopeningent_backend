/// To prevent routes from being taken too often, this structure occasionally poisons edges that have been taken before,
/// so the algorithm will try to find other ways.

use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{SyncSender, Receiver, channel, sync_channel};
use std::thread;
use data::ServingModel;
use graph::Path;

/// Control structure of this module.
pub struct Limit {
    serving_model : Arc<ServingModel>,
    hits : AtomicUsize,
    max_hits : usize,
    async_sender : SyncSender<()>,
    async_queue : SyncSender<()>,
    async_receiver : Mutex<Receiver<usize>>,
}

impl Limit {
    /// Create a new one.
    pub fn new(serving_model : Arc<ServingModel>, factor : f64) -> Limit {
        let count = serving_model.graph.list_ids().flat_map(|id| serving_model.graph.get_edges(id).unwrap()).count();
        let (sx, rx) = sync_channel(1);
        let (sx_2, rx_2) = sync_channel(1);
        let (sx_inv, rx_inv) = channel();
        let conv = serving_model.clone();
        thread::spawn(move || {
            loop {
                if rx.recv().is_err() {break;};
                info!("Resetting...");
                if sx_inv.send(Limit::reset(&conv)).is_err() {break;};
                if rx_2.recv().is_err() {break;};
            }
            error!("The resetter has decided to give up.");
        });

        Limit {
            serving_model : serving_model,
            hits : AtomicUsize::new(0),
            max_hits : (count as f64 * factor) as usize,
            async_sender : sx,
            async_queue : sx_2,
            async_receiver : Mutex::new(rx_inv),
        }
    }

    /// Poison a path.
    pub fn improve(&self, path : &Path) {
        use std::sync::atomic::Ordering;
        let serving_model = &self.serving_model;
        let indices = path.get_indices();
        let mut counter = 0;
        for (&from, &to) in indices.iter().zip(indices[1..].iter()) {
            counter += 1;
            serving_model.graph.get_edge(from, to).unwrap().hits.fetch_add(1, Ordering::Relaxed);
        }
        match self.async_receiver.try_lock().map(|u| u.try_recv()) {
            Ok(Ok(x)) => {self.hits.fetch_sub(x, Ordering::Relaxed);},
            _ => (),
        }
        let count = self.hits.fetch_add(counter, Ordering::Relaxed);
        trace!("Currently at {}/{}", count, self.max_hits);
        if count > self.max_hits {
            match self.async_queue.try_send(()) {
                Ok(_) => {let _ = self.async_sender.send(());},
                _ => (),
            }
        }
    }

    /// Subtract 1 from every edge, to prevent overflow or indifference.
    pub fn reset(serving_model : &ServingModel) -> usize  {
        use std::sync::atomic::Ordering;
        let mut counter = 0;
        for edge in serving_model.graph.list_ids().flat_map(|i| serving_model.graph.get_edges(i).unwrap()) {
            let mut previous = edge.hits.load(Ordering::Relaxed);
            let mut next;
            loop {
                next = previous - 1;
                if previous == 0 {next = 0;}
                let next = edge.hits.compare_and_swap(previous, next, Ordering::Relaxed);
                if next == previous {break;}
                previous = next;
            }
            counter += previous - next;
        }
        counter
    }
}
