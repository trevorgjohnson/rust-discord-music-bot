use std::time::Duration;

use serenity::{model::channel::Message, Result as SerenityResult};

/// Checks that a message successfully sent; if not, then logs why to stdout.
pub fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}

fn show_double_digits(num: u64) -> String {
    if num < 10 {
        return format!("0{}", num);
    } else {
        return num.to_string();
    }
}

pub fn format_duration(duration: Duration) -> String {
    let dur = duration.as_secs();

    if dur > 3600 {
        format!(
            "{}:{}:{}",
            dur / 3600,
            show_double_digits((dur % 3600) / 60),
            show_double_digits(dur % 60)
        )
    } else {
        format!(
            "{}:{}",
            show_double_digits(dur / 60),
            show_double_digits(dur % 60)
        )
    }
}
