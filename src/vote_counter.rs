use crate::mediator::{MediatorUpdate, MediatorUpdateSender};

use stats::Frequencies;
use std::hash::Hash;
use std::sync::{Arc, Mutex, MutexGuard};

#[derive(Clone)]
pub struct VoteCounter<T: Eq + Hash + Clone> {
    frequencies: Arc<Mutex<Frequencies<T>>>,
    tx_update: MediatorUpdateSender,
    update_reference_element: T,
}

impl<T> VoteCounter<T>
where
    T: Eq + Hash + Clone,
{
    pub fn new(tx_update: MediatorUpdateSender, reference_element: T) -> Self {
        Self {
            frequencies: Arc::new(Mutex::new(Frequencies::new())),
            tx_update: tx_update,
            update_reference_element: reference_element,
        }
    }

    pub fn vote(&self, key: T) {
        let mut _freq = self.frequencies.lock().unwrap();
        _freq.add(key);

        self.tx_update
            .send(MediatorUpdate::VoteSystemPercentageChange(Some(
                self._percentage(self.update_reference_element.clone(), _freq),
            )))
            .unwrap();
    }

    pub fn winner(&self) -> Option<T> {
        self.frequencies
            .lock()
            .unwrap()
            .most_frequent()
            .first()
            .map(|&(winner, _)| winner.clone())
    }

    fn _percentage(&self, key: T, _freq: MutexGuard<Frequencies<T>>) -> f64 {
        let count_key = _freq.count(&key);

        let total: u64 = _freq.most_frequent().iter().map(|&(_, count)| count).sum();
        count_key as f64 / total as f64
    }

    pub fn percentage(&self, key: T) -> f64 {
        self._percentage(key, self.frequencies.lock().unwrap())
    }

    pub fn reset(&self) {
        let mut _freq = self.frequencies.lock().unwrap();
        *_freq = Frequencies::new();

        self.tx_update
            .send(MediatorUpdate::VoteSystemPercentageChange(None))
            .unwrap();
    }
}
