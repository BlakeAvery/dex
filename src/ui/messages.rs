use rustbox::{self, Color, RustBox};
use serenity::model::channel;
use std::borrow::Cow;

const LEFT_PADDING: usize = 12;
const RIGHT_PADDING: usize = 12;

pub struct Messages {
    pub messages: Vec<channel::Message>,
    timestamp_fmt: String,
}

impl Messages {
    pub fn new(timestamp_fmt: String) -> Messages {
        Messages {
            messages: Vec::new(),
            timestamp_fmt,
        }
    }

    pub fn add_msg(&mut self, msg: channel::Message) {
        self.messages.push(msg);
    }

    fn wrap<'a>(string: &'a str, length: usize) -> Vec<Cow<'a, str>> {
        string
            .as_bytes()
            .chunks(length)
            .map(String::from_utf8_lossy)
            .collect()
    }

    pub fn render(&mut self, area: &super::layout::Rect, rustbox: &RustBox) {
        let rough_msg_count = area.height;
        let msg_diff = self.messages.len().saturating_sub(rough_msg_count);

        self.messages.drain(0..msg_diff);

        let mut unfolded_msgs = self.messages.clone();
        for mut msg in &mut unfolded_msgs {
            let wrapped_lines: Vec<String> = msg.content
                .lines()
                .map(|line| {
                    Self::wrap(
                        line,
                        area.width.saturating_sub(RIGHT_PADDING + LEFT_PADDING),
                    ).join("\n")
                })
                .collect();
            msg.content = wrapped_lines.join("\n");
        }

        let mut y = area.height - 1;
        for message in unfolded_msgs.iter().rev() {
            let lines: Vec<_> = message.content.lines().rev().collect();
            for (i, line) in lines.iter().enumerate() {
                if i == (lines.len() - 1) {
                    rustbox.print(
                        area.x,
                        y + area.y,
                        rustbox::RB_NORMAL,
                        Color::Default,
                        Color::Default,
                        &format!("{}:", message.author.name),
                    );

                    rustbox.print(
                        area.x + area.width - RIGHT_PADDING,
                        y + area.y,
                        rustbox::RB_NORMAL,
                        Color::Default,
                        Color::Default,
                        &format!(
                            "{}",
                            message
                                .timestamp
                                .with_timezone(&::chrono::offset::Local)
                                .format(&self.timestamp_fmt)
                        ),
                    );
                }
                rustbox.print(
                    LEFT_PADDING + area.x,
                    y + area.y,
                    rustbox::RB_NORMAL,
                    Color::Default,
                    Color::Default,
                    line,
                );
                if y == 0 {
                    return;
                }
                y -= 1;
            }
        }
    }
}