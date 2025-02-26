use chrono::{NaiveDate, NaiveTime};
use rand::Rng;
use std::collections::HashMap;

/// 履歴全体
#[derive(Debug, Clone)]
pub struct History<'src> {
    pub(crate) days: HashMap<NaiveDate, Day<'src>>,
}

/// 1日分のデータ
#[derive(Debug, Clone)]
pub struct Day<'src> {
    pub(crate) date: NaiveDate,
    pub(crate) chats: Vec<Chat<'src>>,
}

impl<'src> Day<'src> {
    /// 日付
    #[must_use]
    pub fn date(&self) -> &NaiveDate {
        &self.date
    }

    /// 1日分のチャットを返す
    #[must_use]
    pub fn chats(&self) -> &[Chat<'src>] {
        &self.chats
    }

    pub fn search_by_keyword(&self, keyword: &'src str) -> impl Iterator<Item = &Chat<'src>> {
        self.chats
            .iter()
            .filter(move |chat| chat.message_lines.iter().any(|line| line.contains(keyword)))
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

impl<'src> Chat<'src> {
    #[must_use]
    pub fn time(&self) -> &NaiveTime {
        &self.time
    }

    #[must_use]
    pub fn sender(&self) -> Option<&'src str> {
        self.speaker
    }

    #[must_use]
    pub fn message_lines(&self) -> &[&'src str] {
        &self.message_lines
    }
}

impl<'src> History<'src> {
    /// Create `LineHistory` structure from text.
    #[must_use]
    pub fn new(days: HashMap<NaiveDate, Day<'src>>) -> Self {
        Self { days }
    }

    /// Create `LineHistory` structure from text.
    #[must_use]
    pub fn from_text(text: &'src str) -> Self {
        match crate::parse::parse_history(text) {
            Ok(history) => history,
            Err((history_incomplete, _)) => history_incomplete,
        }
    }

    #[must_use]
    pub fn days(&self) -> &HashMap<NaiveDate, Day<'src>> {
        &self.days
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.days.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.days.is_empty()
    }

    /// Search history by date.
    #[must_use]
    pub fn search_by_date(&self, date: &NaiveDate) -> Option<&Day> {
        self.days().get(date)
    }

    /// Search history by keyword.
    pub fn search_by_keyword(
        &self,
        keyword: &'src str,
    ) -> impl Iterator<Item = (&NaiveDate, &Chat<'src>)> {
        self.days().values().flat_map(|day| {
            day.search_by_keyword(keyword)
                .map(move |chat| (day.date(), chat))
        })
    }

    /// Search history by random.
    #[cfg(feature = "rand")]
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn search_by_random(&self) -> &Day {
        let range = 0..self.len();

        let mut random = rand::rng();
        let random_index = random.random_range(range);

        let date = self.days.keys().nth(random_index).unwrap();

        self.search_by_date(date).unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct OwnedHistory {
    pub days: HashMap<NaiveDate, OwnedDay>,
}

#[derive(Debug, Clone)]
pub struct OwnedDay {
    pub date: NaiveDate,
    pub chats: Vec<OwnedChat>,
}

#[derive(Debug, Clone)]
pub struct OwnedChat {
    pub time: NaiveTime,
    pub speaker: Option<String>,
    pub message_lines: Vec<String>,
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

impl<'src> From<Day<'src>> for OwnedDay {
    fn from(day: Day<'src>) -> Self {
        let chats = day.chats.into_iter().map(std::convert::Into::into).collect();
        OwnedDay {
            date: day.date,
            chats,
        }
    }
}

impl<'src> From<Chat<'src>> for OwnedChat {
    fn from(chat: Chat<'src>) -> Self {
        OwnedChat {
            time: chat.time,
            speaker: chat.speaker.map(std::borrow::ToOwned::to_owned),
            message_lines: chat.message_lines.into_iter().map(std::borrow::ToOwned::to_owned).collect(),
        }
    }
}

// Alternatively, you could implement an `into_owned` method on History.
impl History<'_> {
    #[must_use]
    pub fn into_owned(self) -> OwnedHistory {
        self.into()
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
