use chrono::{NaiveDate, NaiveTime};
use std::{borrow::Borrow, collections::HashMap};

/// Search history
pub trait Search: SearchByDate + SearchByKeyword + SearchByRandom {}

pub trait SearchByDate {
    type Day: DayData<Self::Chat>;
    type Chat: ChatData;
    fn search_by_date(&self, date: &NaiveDate) -> Option<&Self::Day>;
}

pub trait SearchByKeyword {
    type Chat: ChatData;
    fn search_by_keyword(&self, keyword: &str) -> impl Iterator<Item = &Self::Chat>;
}

pub trait SearchByRandom {
    type Day: SearchByKeyword;
    fn search_by_random(&self) -> &Self::Day;
}

pub trait HistoryData<'src, D: DayData<C>, C: ChatData>: Search {
    fn from_text(text: &'src str) -> Self;
    fn days(&self) -> &HashMap<NaiveDate, D>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
}

pub trait DayData<C: ChatData>: SearchByKeyword {
    fn date(&self) -> &NaiveDate;
    fn chats(&self) -> &[C];
}

pub trait ChatData {
    type String: Borrow<str>;
    fn time(&self) -> &NaiveTime;
    fn sender(&self) -> Option<&str>;
    fn message_lines(&self) -> &[Self::String];
    fn contains(&self, keyword: &str) -> bool {
        self.message_lines()
            .iter()
            .any(|line| line.borrow().contains(keyword))
    }
}
