use chrono::{NaiveDate, NaiveTime};
use rand::Rng;
use std::collections::HashMap;

/// 履歴全体
#[derive(Debug)]
pub struct History<'src> {
    pub(crate) days: HashMap<NaiveDate, Day<'src>>,
}

/// 1日分のデータ
#[derive(Debug)]
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
#[derive(Debug)]
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
    #[allow(clippy::missing_panics_doc)]
    #[must_use]
    pub fn new(days: HashMap<NaiveDate, Day<'src>>) -> Self {
        Self { days }
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

#[must_use]
pub fn ignore_errors<'src, E>(
    result: Result<History<'src>, (History<'src>, Vec<E>)>,
) -> History<'src> {
    match result {
        Ok(history) => history,
        Err((history_incomplete, _)) => history_incomplete,
    }
}
