use chrono::{Local, Timelike};

pub fn select_token(tokens: Vec<String>) -> Option<String> {
    let total_tokens: u32 = tokens.len() as u32;
    if total_tokens == 0 {
        return None;
    }

    let now = Local::now();
    let current_minutes = now.hour() * 60 + now.minute(); // Minutes since midnight

    let minutes_per_segment = 1440 / total_tokens; // 1440 = 24 * 60

    let segment_index = (current_minutes / minutes_per_segment) % total_tokens;

    Some(tokens[segment_index as usize].clone())
}
