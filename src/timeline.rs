use crate::rtw_core::activity::Activity;
use crate::rtw_core::durationw::DurationW;
use crate::rtw_core::ActivityId;
use ansi_term::{Color, Style};
use anyhow::anyhow;
use chrono::{DateTime, Datelike, Duration, Local, Timelike};
use std::cmp::max;
use tbl::{Block, BlockRenderer, Bound, RenderBlock, Renderer, TBLError};

type RGB = (u8, u8, u8);
type Label = (String, RGB);
type Interval = (ActivityId, Activity);

const DEFAULT_TERMINAL_SIZE: usize = 90;

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

fn color(id: ActivityId, colors: &[RGB]) -> RGB {
    let color = colors.get(id % colors.len());
    match color {
        None => (0, 0, 0),
        Some(c) => *c,
    }
}

fn bounds(interval: &Interval) -> (f64, f64) {
    let (_, activity) = interval;
    let start_time: DateTime<Local> = activity.get_start_time().into();
    let stop_time: DateTime<Local> = activity.get_stop_time().into();
    (
        start_time.num_seconds_from_midnight() as f64,
        stop_time.num_seconds_from_midnight() as f64,
    )
}

// label for activities
fn label(interval: &Interval, colors: &[RGB]) -> Option<Label> {
    let (activity_id, activity) = interval;
    Some((activity.get_title(), color(*activity_id, colors)))
}

// label for legend
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

// day total activities duration
fn day_total(activities: &[Interval]) -> Duration {
    activities
        .iter()
        .map(|(_, a)| {
            let duration: Duration = a.get_duration().into();
            duration
        })
        .fold(Duration::seconds(0), |total, duration| total + duration)
}

// earliest and latest activity
fn day_bounds(activities: &[Interval]) -> Bound {
    let min_second: f64 = activities
        .iter()
        .map(|(_, a)| {
            let start: DateTime<Local> = a.get_start_time().into();
            start.num_seconds_from_midnight()
        })
        .min()
        .unwrap_or(0) as f64;
    let max_second = activities
        .iter()
        .map(|(_, a)| {
            let stop: DateTime<Local> = a.get_stop_time().into();
            stop.num_seconds_from_midnight()
        })
        .max()
        .unwrap_or(86_400) as f64;
    (min_second, max_second)
}

// min and max day
fn days(activities: &[Interval]) -> (i32, i32) {
    let min_day = activities
        .iter()
        .map(|(_, a)| {
            let start: DateTime<Local> = a.get_start_time().into();
            start.num_days_from_ce()
        })
        .min()
        .unwrap_or(0);
    let max_day = activities
        .iter()
        .map(|(_, a)| {
            let stop: DateTime<Local> = a.get_stop_time().into();
            stop.num_days_from_ce()
        })
        .max()
        .unwrap_or(0);
    (min_day, max_day)
}

pub(crate) fn render_days(activities: &[Interval], colors: &[RGB]) -> anyhow::Result<Vec<String>> {
    let (width, _height) = term_size::dimensions().unwrap_or((DEFAULT_TERMINAL_SIZE, 0));
    let (min_second, max_second) = day_bounds(activities);
    let (min_day, max_day) = days(activities);
    let mut rendered: Vec<String> = vec![];
    for day in min_day..=max_day {
        let day_activities: Vec<Interval> = activities
            .iter()
            .filter(|&(_, a)| {
                let start_time: DateTime<Local> = a.get_start_time().into();
                start_time.num_days_from_ce() == day
            })
            .cloned()
            .collect();
        let day_month = day_activities
            .first()
            .and_then(|(_, a)| {
                let start_time: DateTime<Local> = a.get_start_time().into();
                Some(start_time.format("%d/%m").to_string())
            })
            .unwrap_or_else(|| "??/??".to_string());
        let total: DurationW = DurationW::from(day_total(day_activities.as_slice()));
        let total_string = total.to_string();
        let right_padding = total_string.len() + 1; // +1 space
        let available_length = max(0, width - right_padding as usize) as usize;
        let legend = Renderer::new(day_activities.as_slice(), &bounds, &legend)
            .with_renderer(&ActivityRenderer {})
            .with_length(available_length)
            .with_boundaries((min_second, max_second))
            .render()
            .or_else(|e| match e {
                TBLError::NoBoundaries => Err(anyhow!("failed to create timeline")),
                TBLError::Intersection(_, _) => Err(anyhow!(
                    "failed to create timeline: some activities are overlapping"
                )),
            })?;
        rendered.push(format!("{}{:>8}", legend, day_month));
        let timeline = Renderer::new(day_activities.as_slice(), &bounds, &|a| label(a, colors))
            .with_renderer(&ActivityRenderer {})
            .with_length(available_length)
            .with_boundaries((min_second, max_second))
            .render()
            .or_else(|e| match e {
                TBLError::NoBoundaries => Err(anyhow!("failed to create timeline")),
                TBLError::Intersection(_, _) => Err(anyhow!(
                    "failed to create timeline: some activities are overlapping"
                )),
            })?;
        rendered.push(format!("{}{}", timeline, total_string));
    }
    Ok(rendered)
}
