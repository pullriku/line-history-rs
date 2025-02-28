use rand::Rng;

use crate::{history::{Day, History, OwnedDay, OwnedHistory}, traits::{HistoryData, SearchByRandom}};

impl<'src> SearchByRandom for History<'src> {
    type Day = Day<'src>;
    fn search_by_random(&self) -> &Self::Day {
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.len());
        self.days.values().nth(index).unwrap()
    }
}

impl SearchByRandom for OwnedHistory {
    type Day = OwnedDay;
    fn search_by_random(&self) -> &Self::Day {
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.len());
        self.days.values().nth(index).unwrap()
    }
}
