//! Timeline display
use crate::rtw_core::activity::{Activity, OngoingActivity};
use crate::rtw_core::durationw::DurationW;
use crate::rtw_core::ActivityId;
use ansi_term::{Color, Style};
use anyhow::anyhow;
use chrono::{DateTime, Datelike, Duration, Local, Timelike};
use std::cmp::max;
use tbl::{Block, Bound, RenderBlock, Renderer, TBLError};

type Rgb = (u8, u8, u8);
type Label = (String, Rgb);
type Interval = (ActivityId, Activity);

const DEFAULT_TERMINAL_SIZE: usize = 90;

fn chunkify(s: &str, size: usize) -> Vec<String> {
    if size == 0 {
        vec![]
    } else {
        let inter: Vec<char> = s.chars().collect();
        let chunks = inter.chunks_exact(size);
        let remainder = chunks.remainder().to_vec();
        let padding: Vec<char> = std::iter::repeat(' ')
            .take(size - remainder.len())
            .collect();
        let padded_remainder: Vec<char> = remainder.iter().chain(padding.iter()).cloned().collect();
        let chunks: Vec<String> = chunks
            .chain(std::iter::once(padded_remainder.as_slice()))
            .map(|s| s.iter().collect::<String>())
            .collect();
        chunks
    }
}

fn split_interval_if_needed(interval: &Interval) -> (Interval, Option<Interval>) {
    let (activity_id, activity) = interval;
    let start_time: DateTime<Local> = activity.get_start_time().into();
    let stop_time: DateTime<Local> = activity.get_stop_time().into();
    let day_span: i32 = stop_time.num_days_from_ce() - start_time.num_days_from_ce();
    if day_span < 1 {
        (interval.clone(), None) // activity start time and stop time same day
    } else {
        let same_day_midnight: DateTime<Local> = start_time.date().and_hms_milli(23, 59, 59, 999);
        // Paranoia in case 23:59:59:999 < start_time < midnight
        let (same_day_start, same_day_end) = if start_time < same_day_midnight {
            (start_time, same_day_midnight)
        } else {
            (same_day_midnight, start_time)
        };
        let same_day_to_midnight = OngoingActivity::new(
            same_day_start.into(),
            activity.get_tags(),
            activity.get_description(),
        )
        .into_activity(same_day_end.into())
        .unwrap(); // safe to unwrap thanks to previous test: same_day_start <= same_day_end
        let day_after: DateTime<Local> = start_time.date().and_hms(0, 0, 0) + Duration::days(1);
        let other_days = OngoingActivity::new(
            day_after.into(),
            activity.get_tags(),
            activity.get_description(),
        )
        .into_activity(stop_time.into())
        .unwrap(); // safe to unwrap because stop_time >= day_after
        (
            (*activity_id, same_day_to_midnight),
            Some((*activity_id, other_days)),
        )
    }
}

fn split_interval(interval: &Interval) -> Vec<Interval> {
    match split_interval_if_needed(interval) {
        (i, None) => vec![i],
        (i, Some(other)) => std::iter::once(i).chain(split_interval(&other)).collect(),
    }
}

fn render(b: &Block<(String, (u8, u8, u8))>) -> RenderBlock {
    match b {
        Block::Space(size) => RenderBlock::Space(" ".repeat(*size)),
        Block::Segment(size, label) => {
            let (label, (r, g, b)) = label.clone().unwrap_or((String::default(), (0, 0, 0)));
            let chunks = chunkify(&label, *size);
            let style = Style::new().on(Color::RGB(r, g, b));
            let color_chunks = chunks.iter().map(|s| style.paint(s).to_string()).collect();
            RenderBlock::MultiLineBlock(color_chunks)
        }
    }
}

fn color(id: ActivityId, colors: &[Rgb]) -> Rgb {
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
// Wrapping with `Option` is unnecessary but this signature
// is expected by `Renderer`
#[allow(clippy::unnecessary_wraps)]
fn label(interval: &Interval, colors: &[Rgb]) -> Option<Label> {
    let (activity_id, activity) = interval;
    Some((activity.get_title(), color(*activity_id, colors)))
}

fn legend(interval: &Interval) -> Label {
    let (_activity_id, activity) = interval;
    let start_time: DateTime<Local> = activity.get_start_time().into();
    let end_time: DateTime<Local> = activity.get_stop_time().into();
    (
        format!(
            "{}-{}",
            start_time.format("%H:%M"),
            end_time.format("%H:%M")
        ),
        (0, 0, 0),
    )
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

pub(crate) fn render_days(activities: &[Interval], colors: &[Rgb]) -> anyhow::Result<Vec<String>> {
    let (width, _height) = term_size::dimensions().unwrap_or((DEFAULT_TERMINAL_SIZE, 0));
    let (min_second, max_second) = day_bounds(activities);
    let (min_day, max_day) = days(activities);
    let mut rendered: Vec<String> = vec![];
    for day in min_day..=max_day {
        let day_activities: Vec<Interval> = activities
            .iter()
            .flat_map(split_interval)
            .filter(|(_, a)| {
                let start_time: DateTime<Local> = a.get_start_time().into();
                start_time.num_days_from_ce() == day
            })
            .collect();
        let day_month = day_activities
            .first()
            .map(|(_, a)| {
                let start_time: DateTime<Local> = a.get_start_time().into();
                start_time.format("%d/%m").to_string()
            })
            .unwrap_or_else(|| "??/??".to_string());
        let total: DurationW = DurationW::from(day_total(day_activities.as_slice()));
        let total_string = total.to_string();
        let right_padding = total_string.len() + 1; // +1 space
        let available_length = max(0, width - right_padding as usize) as usize;
        let data = Renderer::new(day_activities.as_slice(), &bounds, &|a| label(a, colors))
            .with_renderer(&render)
            .with_length(available_length)
            .with_boundaries((min_second, max_second))
            .render()
            .map_err(|e| match e {
                TBLError::NoBoundaries => anyhow!("failed to create timeline"),
                TBLError::Intersection(left, right) => anyhow!(
                    "failed to create timeline: some activities are overlapping: {:?} intersects {:?}", left, right
                ),
            })?;
        let legend = Renderer::new(day_activities.as_slice(), &bounds, &|interval| {
            Some(legend(interval))
        })
        .with_renderer(&render)
        .with_length(available_length)
        .with_boundaries((min_second, max_second))
        .render()
        .map_err(|e| match e {
            TBLError::NoBoundaries => anyhow!("failed to create timeline"),
            TBLError::Intersection(left, right) => anyhow!(
                "failed to create timeline: some activities are overlapping: {:?} intersects {:?}",
                left,
                right
            ),
        })?;
        let timeline = legend.iter().zip(data.iter());
        for (legend_timelines, data_timelines) in timeline {
            for (j, line) in legend_timelines.iter().enumerate() {
                if j == 0 {
                    rendered.push(format!("{}{:>8}", line, day_month));
                } else {
                    rendered.push(format!("{}{:>8}", line, " "));
                }
            }
            for (j, line) in data_timelines.iter().enumerate() {
                if j == 0 {
                    rendered.push(format!("{}{}", line, total_string));
                } else {
                    rendered.push(format!("{}{:>8}", line, " "));
                }
            }
        }
    }
    Ok(rendered)
}
