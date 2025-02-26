use chrono::{NaiveDate, NaiveTime};
use rand::Rng;
use std::collections::HashMap;

use crate::traits::{
    ChatData, DayData, HistoryData, Search, SearchByDate, SearchByKeyword, SearchByRandom,
};

/// 履歴全体
#[derive(Debug, Clone)]
pub struct History<'src> {
    pub(crate) days: HashMap<NaiveDate, Day<'src>>,
}

impl<'src> History<'src> {
    /// Create `LineHistory` structure from text.
    #[must_use]
    pub fn new(days: HashMap<NaiveDate, Day<'src>>) -> Self {
        Self { days }
    }

    #[must_use]
    pub fn into_owned(self) -> OwnedHistory {
        self.into()
    }
}

impl Search for History<'_> {}

impl<'src> SearchByDate for History<'src> {
    type Day = Day<'src>;
    type Chat = Chat<'src>;
    fn search_by_date(&self, date: &NaiveDate) -> Option<&Self::Day> {
        self.days.get(date)
    }
}

impl<'src> SearchByKeyword for History<'src> {
    type Chat = Chat<'src>;
    fn search_by_keyword(&self, keyword: &str) -> impl Iterator<Item = (NaiveDate, &Self::Chat)> {
        self.days
            .values()
            .flat_map(move |day| day.search_by_keyword(keyword))
    }
}

impl<'src> SearchByRandom for History<'src> {
    type Day = Day<'src>;
    fn search_by_random(&self) -> &Self::Day {
        let mut rng = rand::rng();
        let index = rng.random_range(0..self.len());
        self.days.values().nth(index).unwrap()
    }
}

impl<'src> HistoryData<'src, Day<'src>, Chat<'src>> for History<'src> {
    fn from_text(text: &'src str) -> Self {
        match crate::parse::parse_history(text) {
            Ok(history) => history,
            Err((history_incomplete, _)) => history_incomplete,
        }
    }

    fn days(&self) -> &HashMap<NaiveDate, Day<'src>> {
        &self.days
    }

    fn len(&self) -> usize {
        self.days.len()
    }

    fn is_empty(&self) -> bool {
        self.days.is_empty()
    }
}

/// 1日分のデータ
#[derive(Debug, Clone)]
pub struct Day<'src> {
    pub(crate) date: NaiveDate,
    pub(crate) chats: Vec<Chat<'src>>,
}

impl<'src> SearchByKeyword for Day<'src> {
    type Chat = Chat<'src>;
    fn search_by_keyword(&self, keyword: &str) -> impl Iterator<Item = (NaiveDate, &Self::Chat)> {
        self.chats
            .iter()
            .map(move |chat| (self.date, chat))
            .filter(move |(_, chat)| chat.contains(keyword))
    }
}

impl<'src> DayData<Chat<'src>> for Day<'src> {
    fn date(&self) -> &NaiveDate {
        &self.date
    }

    fn chats(&self) -> &[Chat<'src>] {
        &self.chats
    }
}

impl Day<'_> {
    #[must_use]
    pub fn into_owned(self) -> OwnedDay {
        self.into()
    }
}

/// 1チャットのデータ
#[derive(Debug, Clone)]
pub struct Chat<'src> {
    pub(crate) time: NaiveTime,
    pub(crate) speaker: Option<&'src str>,
    /// 複数行にまたがる発言内容（各行を保持）
    pub(crate) message_lines: Vec<&'src str>,
}

impl<'src> ChatData for Chat<'src> {
    type String = &'src str;
    fn time(&self) -> &NaiveTime {
        &self.time
    }

    fn sender(&self) -> Option<&str> {
        self.speaker
    }

    fn message_lines(&self) -> &[Self::String] {
        &self.message_lines
    }
}

impl Chat<'_> {
    #[must_use]
    pub fn into_owned(self) -> OwnedChat {
        self.into()
    }
}

#[derive(Debug, Clone)]
pub struct OwnedHistory {
    pub days: HashMap<NaiveDate, OwnedDay>,
}

impl Search for OwnedHistory {}

impl SearchByDate for OwnedHistory {
    type Day = OwnedDay;
    type Chat = OwnedChat;
    fn search_by_date(&self, date: &NaiveDate) -> Option<&Self::Day> {
        self.days.get(date)
    }
}

impl SearchByKeyword for OwnedHistory {
    type Chat = OwnedChat;
    fn search_by_keyword(&self, keyword: &str) -> impl Iterator<Item = (NaiveDate, &Self::Chat)> {
        self.days
            .values()
            .flat_map(move |day| day.search_by_keyword(keyword))
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

impl<'src> HistoryData<'src, OwnedDay, OwnedChat> for OwnedHistory {
    fn from_text(text: &'src str) -> Self {
        match crate::parse::parse_history(text) {
            Ok(history) => history.into(),
            Err((history_incomplete, _)) => history_incomplete.into(),
        }
    }

    fn days(&self) -> &HashMap<NaiveDate, OwnedDay> {
        &self.days
    }

    fn len(&self) -> usize {
        self.days.len()
    }

    fn is_empty(&self) -> bool {
        self.days.is_empty()
    }
}

impl<'src> From<History<'src>> for OwnedHistory {
    fn from(history: History<'src>) -> Self {
        let days = history
            .days
            .into_iter()
            .map(|(date, day)| (date, day.into()))
            .collect();
        OwnedHistory { days }
    }
}

impl OwnedHistory {
    #[must_use]
    pub fn as_ref_history(&self) -> History<'_> {
        History {
            days: self.days.iter().map(|(date, day)| (*date, day.as_ref_day())).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedDay {
    pub date: NaiveDate,
    pub chats: Vec<OwnedChat>,
}

impl DayData<OwnedChat> for OwnedDay {
    fn date(&self) -> &NaiveDate {
        &self.date
    }

    fn chats(&self) -> &[OwnedChat] {
        &self.chats
    }
}

impl SearchByKeyword for OwnedDay {
    type Chat = OwnedChat;
    fn search_by_keyword(&self, keyword: &str) -> impl Iterator<Item = (NaiveDate, &Self::Chat)> {
        self.chats
            .iter()
            .map(move |chat| (self.date, chat))
            .filter(move |(_, chat)| chat.contains(keyword))
    }
}

impl<'src> From<Day<'src>> for OwnedDay {
    fn from(day: Day<'src>) -> Self {
        let chats = day
            .chats
            .into_iter()
            .map(std::convert::Into::into)
            .collect();
        OwnedDay {
            date: day.date,
            chats,
        }
    }
}

impl OwnedDay {
    pub fn as_ref_day(&self) -> Day<'_> {
        Day {
            date: self.date,
            chats: self.chats.iter().map(OwnedChat::as_ref_chat).collect(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedChat {
    pub time: NaiveTime,
    pub speaker: Option<String>,
    pub message_lines: Vec<String>,
}

impl ChatData for OwnedChat {
    type String = String;
    fn time(&self) -> &NaiveTime {
        &self.time
    }

    fn sender(&self) -> Option<&str> {
        self.speaker.as_deref()
    }

    fn message_lines(&self) -> &[String] {
        &self.message_lines
    }
}

impl<'src> From<Chat<'src>> for OwnedChat {
    fn from(chat: Chat<'src>) -> Self {
        OwnedChat {
            time: chat.time,
            speaker: chat.speaker.map(std::borrow::ToOwned::to_owned),
            message_lines: chat
                .message_lines
                .into_iter()
                .map(std::borrow::ToOwned::to_owned)
                .collect(),
        }
    }
}

impl OwnedChat {
    pub fn as_ref_chat(&self) -> Chat<'_> {
        Chat {
            time: self.time,
            speaker: self.speaker.as_deref(),
            message_lines: self.message_lines.iter().map(AsRef::as_ref).collect(),
        }
    }
}

#[must_use]
pub fn ignore_errors<'src, E>(
    result: Result<History<'src>, (History<'src>, Vec<E>)>,
) -> History<'src> {
    match result {
        Ok(history) => history,
        Err((history_incomplete, _)) => history_incomplete,
    }
}
