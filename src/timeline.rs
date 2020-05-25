use ansi_term::{Color, Style};
use anyhow::anyhow;
use chrono::{DateTime, Duration, Local};
use rtw::{Activity, ActivityId, DurationW};
use std::cmp::max;
use tbl::{Block, BlockRenderer, RenderBlock, Renderer};

type RGB = (u8, u8, u8);
type Label = (String, RGB);
type Interval = (ActivityId, Activity);

struct ActivityRenderer {}

impl BlockRenderer<Label> for ActivityRenderer {
    fn render(&self, b: &Block<(String, (u8, u8, u8))>) -> RenderBlock {
        match b {
            Block::Space(size) => RenderBlock::Space(" ".repeat(*size)),
            Block::Segment(size, label) => {
                let (label, (r, g, b)) = label.clone().unwrap_or((String::default(), (0, 0, 0)));
                let mut truncated = label;
                truncated.truncate(*size);
                let left = size - truncated.len();
                let style = Style::new().on(Color::RGB(r, g, b));
                RenderBlock::Block(
                    style
                        .paint(format!("{}{}", truncated, " ".repeat(left),))
                        .to_string(),
                )
            }
        }
    }
}

fn color(id: ActivityId) -> RGB {
    match id % 3 {
        0 => (27, 94, 32),
        1 => (13, 71, 161),
        _ => (191, 54, 12),
    }
}

fn bounds(interval: &Interval) -> (f64, f64) {
    let (_, activity) = interval;
    let start_time: DateTime<Local> = activity.get_start_time().into();
    let stop_time: DateTime<Local> = activity.get_stop_time().into();
    (start_time.timestamp() as f64, stop_time.timestamp() as f64)
}

fn label(interval: &Interval) -> Option<Label> {
    let (activity_id, activity) = interval;
    Some((activity.get_title(), color(*activity_id)))
}

fn legend(interval: &Interval) -> Option<Label> {
    let (_activity_id, activity) = interval;
    let start_time: DateTime<Local> = activity.get_start_time().into();
    let end_time: DateTime<Local> = activity.get_stop_time().into();
    Some((
        format!(
            "{}-{}",
            start_time.format("%H:%M"),
            end_time.format("%H:%M")
        ),
        (0, 0, 0),
    ))
}

pub(crate) fn render(activities: &[Interval]) -> anyhow::Result<String> {
    let (width, _height) = term_size::dimensions().ok_or_else(|| anyhow!("fqi"))?;
    let total: Duration = activities
        .iter()
        .map(|(_, a)| {
            let duration: Duration = a.get_duration().into();
            duration
        })
        .fold(Duration::seconds(0), |total, duration| total + duration);
    let total = DurationW::from(total);
    let total_string = total.to_string();
    let right_padding = total_string.len() + 1; // +1 space
    let available_length = max(0, width - right_padding as usize) as usize;
    let legend = Renderer::new(activities, &bounds, &legend)
        .with_renderer(&ActivityRenderer {})
        .with_length(available_length)
        .render()
        .or_else(|_| Err(anyhow!("failed to create timeline")))?;
    let timeline = Renderer::new(activities, &bounds, &label)
        .with_renderer(&ActivityRenderer {})
        .with_length(available_length)
        .render()
        .or_else(|_| Err(anyhow!("failed to create timeline")))?;
    Ok(format!("{} total\n{} {}", legend, timeline, total_string))
}
