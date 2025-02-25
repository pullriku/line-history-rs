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

fn skip_header(src: &str) -> Option<&str> {
    let crlf_3rd_index = src.match_indices("\r\n").nth(2)?.0;
    src.get(crlf_3rd_index + "\r\n".len()..)
}

/// "HH:MM\t"形式の行かどうかを判定する
#[must_use]
pub fn is_chat_start(line: &str) -> bool {
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

fn is_day_start(line: &str) -> bool {
    let Some(date_candidate) = line.get(..10) else {
        return false;
    };

    let mut sections = date_candidate.split('/');
    for _ in 0..3 {
        let Some(section) = sections.next() else {
            return false;
        };
        let mut chars = section.chars();
        let (Some(c1), Some(c2)) = (chars.next(), chars.next()) else {
            return false;
        };
        if !(c1.is_ascii_digit() && c2.is_ascii_digit()) {
            return false;
        }
    }

    true
}

/// 入力全体のチャット履歴（History）をパースする。
/// 各部分は元の &str からのスライスとして保持するため、コピーは発生しません。
///
/// # Errors
/// フォーマットと異なる場合には、`ParseError` を返します。
pub fn parse_history<'src>(
    input: &'src str,
) -> Result<History<'src>, (History<'src>, Vec<ParseError<'src>>)> {
    let Some(first_line) = input.lines().next() else {
        return Err((History::new(HashMap::new()), vec![ParseError::EmptyFile]));
    };
    let input = if is_day_start(first_line) {
        input
    } else {
        skip_header(input).unwrap_or_default()
    };

    let mut days: HashMap<NaiveDate, Day<'_>> = HashMap::new();
    let mut errors: Vec<ParseError> = Vec::new();

    let mut current_date: Option<NaiveDate> = None;

    // 各日付セクションは "\r\n\r\n" で区切られている
    for section in input.split("\r\n\r\n") {
        if section.is_empty() {
            continue;
        }
        let mut lines = section.split("\r\n");
        // セクションの最初の行は日付行（例: "2025/01/01(水)"）
        let Some(date_line) = lines.next() else {
            continue;
        };

        // 日付部分は '(' より前を抽出し、空白を除去
        let date_str = date_line.get(..10).unwrap_or_default();

        // NaiveDateとしてパース（フォーマット例: "%Y/%m/%d"）
        let date_parsed = NaiveDate::parse_from_str(date_str, "%Y/%m/%d").ok();

        let mut chats = Vec::new();
        let mut current_chat: Option<Chat<'src>> = None;

        // 日付行以降の各行を処理
        for line in lines {
            if is_chat_start(line) {
                // 新たなチャット開始前に、既存のチャットがあれば確定
                if let Some(chat) = current_chat.take() {
                    chats.push(chat);
                }
                let mut parts = line.splitn(3, '\t');
                let Some(time_str) = parts.next() else {
                    errors.push(ParseError::InvalidEntry(line));
                    continue;
                };
                let Ok(time) = NaiveTime::parse_from_str(time_str, "%H:%M") else {
                    errors.push(ParseError::InvalidTime(time_str));
                    continue;
                };
                let Some(speaker_str) = parts.next() else {
                    errors.push(ParseError::InvalidEntry(line));
                    continue;
                };
                // 発言者が空文字の場合はNone
                let speaker = if speaker_str.trim().is_empty() {
                    None
                } else {
                    Some(speaker_str)
                };
                let Some(message_line) = parts.next() else {
                    errors.push(ParseError::InvalidEntry(line));
                    continue;
                };
                current_chat = Some(Chat {
                    time,
                    speaker,
                    message_lines: vec![message_line],
                });
            } else {
                // チャット開始行でない場合、前回のチャットの継続行として追加
                if let Some(ref mut chat) = current_chat {
                    chat.message_lines.push(line);
                } else {
                    errors.push(ParseError::ContinuationBeforeEntry(line));
                }
            }
        }
        // セクション最後のチャットを追加
        if let Some(chat) = current_chat {
            chats.push(chat);
        }

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
            day.chats.extend(chats);
        }
    }

    if errors.is_empty() {
        Ok(History::new(days))
    } else {
        Err((History::new(days), errors))
    }
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
    fn test_ai() {
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
