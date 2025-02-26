use chrono::{NaiveDate, NaiveTime};
use std::collections::HashMap;

use crate::history::{Chat, Day, History};

#[derive(Debug)]
pub enum ParseError<'src> {
    EmptyFile,
    InvalidEntry(&'src str),
    ContinuationBeforeEntry(&'src str),
    InvalidDate(&'src str),
    InvalidTime(&'src str),
    InternalError { line: &'src str, error: &'src str },
}

/// Parses the entire chat history (History) from the input.
/// Each part is retained as a slice from the original &str, so no copying occurs.
///
/// # Errors
/// Returns `ParseError` if the format is incorrect.
pub fn parse_history(input: &str) -> Result<History<'_>, (History<'_>, Vec<ParseError<'_>>)> {
    let Some(first_line) = input.lines().next() else {
        return Err((History::new(HashMap::new()), vec![ParseError::EmptyFile]));
    };
    let input = if parse_date_line(first_line).is_some() {
        input
    } else {
        skip_header(input).unwrap_or_default()
    };

    let mut days: HashMap<NaiveDate, Day<'_>> = HashMap::new();
    let mut errors: Vec<ParseError> = Vec::new();

    let mut current_date: Option<NaiveDate> = None;

    // Each date section is separated by "\r\n\r\n"
    for section in input.split("\r\n\r\n") {
        if section.is_empty() {
            continue;
        }
        let mut lines = section.split("\r\n");
        // The first line of the section is the date line (e.g., "2025/01/01(水)")
        let Some(date_line) = lines.next() else {
            continue;
        };

        // Parse the date line into a NaiveDate (format example: "%Y/%m/%d")
        let date_parsed = parse_date_line(date_line);

        let (mut chats, mut section_errors) = parse_chats(lines);
        errors.append(&mut section_errors);
        drop(section_errors);

        if let Some(date) = date_parsed {
            days.insert(date, Day { date, chats });
            current_date = Some(date);
        } else {
            let Some(current_date) = current_date else {
                errors.push(ParseError::InternalError {
                    line: date_line,
                    error: "Failed to parse date",
                });
                continue;
            };
            let Some(day) = days.get_mut(&current_date) else {
                errors.push(ParseError::InternalError {
                    line: date_line,
                    error: "Day not found",
                });
                continue;
            };
            day.chats.append(&mut chats);
        }
    }

    if errors.is_empty() {
        Ok(History::new(days))
    } else {
        Err((History::new(days), errors))
    }
}

/// Extracts the first 10 characters from the date line (e.g., "2025/01/01")
/// and parses it into a `NaiveDate`. Returns None if parsing fails.
fn parse_date_line(date_line: &str) -> Option<NaiveDate> {
    let date_str = date_line.get(..10)?;
    NaiveDate::parse_from_str(date_str, "%Y/%m/%d").ok()
}

fn skip_header(src: &str) -> Option<&str> {
    let crlf_3rd_index = src.match_indices("\r\n").nth(2)?.0;
    src.get(crlf_3rd_index + "\r\n".len()..)
}

/// Determines whether the line is in the "HH:MM\t" format.
fn is_chat_start(line: &str) -> bool {
    if line.len() < 6 {
        return false;
    }
    let mut chars = line.chars();
    let h1 = chars.next();
    let h2 = chars.next();
    let colon = chars.next();
    let m1 = chars.next();
    let m2 = chars.next();
    let tab = chars.next();
    matches!(
        (h1, h2, colon, m1, m2, tab),
        (Some(c1), Some(c2), Some(':'), Some(c3), Some(c4), Some('\t'))
            if c1.is_ascii_digit() && c2.is_ascii_digit() && c3.is_ascii_digit() && c4.is_ascii_digit()
    )
}

/// Parses chat information from the given iterator over lines.
///
/// Processes chat starting lines and their continuation lines,
/// returning a vector of chats and a vector of errors.
fn parse_chats<'src, I>(lines: I) -> (Vec<Chat<'src>>, Vec<ParseError<'src>>)
where
    I: Iterator<Item = &'src str>,
{
    let mut chats = Vec::new();
    let mut errors = Vec::new();
    let mut current_chat: Option<Chat<'src>> = None;

    for line in lines {
        if is_chat_start(line) {
            // Finalize the existing chat and start a new chat.
            if let Some(chat) = current_chat.take() {
                chats.push(chat);
            }
            match parse_chat_entry(line) {
                Ok(chat) => current_chat = Some(chat),
                Err(err) => errors.push(err),
            }
        } else {
            // If the line is not a chat starting line, add it as a continuation of the previous chat.
            if let Some(ref mut chat) = current_chat {
                chat.message_lines.push(line);
            } else if !line.trim().is_empty() {
                errors.push(ParseError::ContinuationBeforeEntry(line));
            }
        }
    }
    if let Some(chat) = current_chat {
        chats.push(chat);
    }
    (chats, errors)
}

/// Parses a chat starting line (a line beginning with "HH:MM\t") and generates a Chat.
fn parse_chat_entry(line: &str) -> Result<Chat<'_>, ParseError<'_>> {
    let mut parts = line.splitn(3, '\t');
    let time_str = parts.next().ok_or(ParseError::InvalidEntry(line))?;
    let time = NaiveTime::parse_from_str(time_str, "%H:%M")
        .map_err(|_| ParseError::InvalidTime(time_str))?;
    let speaker_str = parts.next().ok_or(ParseError::InvalidEntry(line))?;
    let speaker = if speaker_str.trim().is_empty() {
        None
    } else {
        Some(speaker_str)
    };
    let message_line = parts.next().ok_or(ParseError::InvalidEntry(line))?;
    Ok(Chat {
        time,
        speaker,
        message_lines: vec![message_line],
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;

    const CONTENT: &str = "2020/02/29(土)\r
13:00\tC\t衛生面に気をつけよう\r
13:00\tA\tおはよう\r
13:01\tB\tOK\r
\r
2023/07/21(金)\r
01:00\tD\t夏だね\r
01:01\tA\t\"過去の会話でも見ようか
2017/01/01(日)
00:00\tA\tあけおめ
000:01\tB\tおめでとう

2020/02/29(土)
13:00\tC\t同じ日付
13:00\tA\tおなじ
13:01\tB\tOK\"\r
01:02\tB\tUnit Test はクリアできたかな？\r
\r
2023/07/31(月)\r
00:00\tname\t\"\r
\r
\"\r
00:01\tname\ta\r
\r
2023/08/01(火)\r
06:00\tA\t夏だね2\r
06:11\tD\tおはよう\r
";
    #[test]
    fn test_parse() {
        let history = parse_history(CONTENT).unwrap();
        assert_eq!(history.days.len(), 4);
        assert_eq!(
            history
                .days
                .into_iter()
                .collect::<BTreeMap<_, _>>()
                .into_iter()
                .map(|(date, day)| (date, day.chats.len()))
                .collect::<Vec<_>>(),
            vec![
                (NaiveDate::from_ymd_opt(2020, 2, 29).unwrap(), 3),
                (NaiveDate::from_ymd_opt(2023, 7, 21).unwrap(), 3),
                (NaiveDate::from_ymd_opt(2023, 7, 31).unwrap(), 2),
                (NaiveDate::from_ymd_opt(2023, 8, 1).unwrap(), 2),
            ],
        );
    }
}
