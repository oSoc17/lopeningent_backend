use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::{SyncSender, Receiver, channel, sync_channel};
use std::thread;
use graph::Graph;
use data::Conversion;
use graph::Path;

use std::io;
use std::io::Write;

pub struct Limit {
    conversion : Arc<Conversion>,
    hits : AtomicUsize,
    max_hits : usize,
    async_sender : SyncSender<()>,
    async_queue : SyncSender<()>,
    async_receiver : Mutex<Receiver<usize>>,
}

impl Limit {
    pub fn new(conversion : Arc<Conversion>, factor : f64) -> Limit {
        let count = conversion.graph.list_ids().flat_map(|id| conversion.graph.get_edges(id).unwrap()).count();
        let (sx, rx) = sync_channel(1);
        let (sx_2, rx_2) = sync_channel(1);
        let (sx_inv, rx_inv) = channel();
        let conv = conversion.clone();
        thread::spawn(move || {
            loop {
                rx.recv();
                let _ = writeln!(io::stderr(), "Resetting...");
                sx_inv.send(Limit::reset(&conv));
                rx_2.recv();
            }
        });

        Limit {
            conversion : conversion,
            hits : AtomicUsize::new(0),
            max_hits : (count as f64 * factor) as usize,
            async_sender : sx,
            async_queue : sx_2,
            async_receiver : Mutex::new(rx_inv),
        }
    }

    pub fn improve(&self, path : &Path) {
        use std::sync::atomic::Ordering;
        let conversion = &self.conversion;
        let indices = path.get_indices();
        let mut counter = 0;
        for (&from, &to) in indices.iter().zip(indices[1..].iter()) {
            counter += 1;
            conversion.graph.get_edge(from, to).unwrap().hits.fetch_add(1, Ordering::Relaxed);
        }
        match self.async_receiver.try_lock().map(|u| u.try_recv()) {
            Ok(Ok(x)) => {self.hits.fetch_sub(x, Ordering::Relaxed);},
            _ => (),
        }
        let count = self.hits.fetch_add(counter, Ordering::Relaxed);
        let _ = writeln!(io::stderr(), "Currently at {}/{}", count, self.max_hits);
        if count > self.max_hits {
            match self.async_queue.try_send(()) {
                Ok(_) => {self.async_sender.send(());},
                _ => (),
            }
        }
    }

    pub fn reset(conversion : &Conversion) -> usize  {
        use std::sync::atomic::Ordering;
        let mut counter = 0;
        for edge in conversion.graph.list_ids().flat_map(|i| conversion.graph.get_edges(i).unwrap()) {
            let mut previous = edge.hits.load(Ordering::Relaxed);
            let mut next = 0;
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
