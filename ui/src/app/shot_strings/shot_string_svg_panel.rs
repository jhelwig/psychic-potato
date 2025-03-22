use std::rc::Rc;

use log::error;
use patternfly_yew::prelude::*;
use shared_types::response::{
    League,
    Match,
    ShotMarkerShot,
};
use uuid::Uuid;
use yew::{
    prelude::*,
    suspense::use_future,
};

use crate::app::shot_strings::fetch_shots_for_string;

#[derive(Debug, Clone, PartialEq, Eq, Properties)]
pub struct ShotStringSvgPanelProps {
    pub league:         Rc<League>,
    pub match_object:   Rc<Match>,
    pub shot_string_id: Uuid,
}

#[function_component(ShotStringSvgPanel)]
pub fn shot_string_svg_panel(props: &ShotStringSvgPanelProps) -> HtmlResult {
    let league = props.league.clone();
    let league_id = league.id;
    let match_object = props.match_object.clone();
    let match_id = match_object.id;
    let shot_string_id = props.shot_string_id;
    let shot_string_shots_result =
        use_future(
            || async move { fetch_shots_for_string(league_id, match_id, shot_string_id).await },
        )?;

    let html_result = match &*shot_string_shots_result {
        Ok(shot_string_shots) => {
            let shot_string_shots = Rc::new(shot_string_shots.clone());
            html!(<ShotStringSvg {shot_string_shots} />)
        }
        Err(e) => {
            error!("Error fetching shot string shots: {e}");
            html!(
                <Content>
                    { format!("Error: {e}") }
                </Content>
            )
        }
    };

    Ok(html_result)
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct ShotStringSvgProps {
    pub shot_string_shots: Rc<Vec<ShotMarkerShot>>,
}

const UNIT_SCALE_FACTOR: f64 = 10.0;

#[function_component(ShotStringSvg)]
fn shot_string_svg(props: &ShotStringSvgProps) -> Html {
    let render_width = 900;
    let render_height = 900;

    struct RingDef {
        id:                 &'static str,
        radius:             f64,
        fill:               &'static str,
        stroke:             &'static str,
        stroke_width:       f64,
        label_text:         &'static str,
        label_color:        &'static str,
        label_font_size:    f64,
        label_stroke_width: f64,
    }

    let stroke_width = 0.2;
    let label_font_size = 10.0;
    let label_stroke_width = 0.1;
    let ring_defs = [
        RingDef {
            id: "ring-5",
            radius: 24.0,
            fill: "white",
            stroke: "black",
            stroke_width,
            label_text: "5",
            label_color: "black",
            label_font_size,
            label_stroke_width,
        },
        RingDef {
            id: "ring-6",
            radius: 18.0,
            fill: "black",
            stroke: "white",
            stroke_width,
            label_text: "6",
            label_color: "white",
            label_font_size,
            label_stroke_width,
        },
        RingDef {
            id: "ring-7",
            radius: 12.0,
            fill: "black",
            stroke: "white",
            stroke_width,
            label_text: "7",
            label_color: "white",
            label_font_size,
            label_stroke_width,
        },
        RingDef {
            id: "ring-8",
            radius: 9.0,
            fill: "black",
            stroke: "white",
            stroke_width,
            label_text: "8",
            label_color: "white",
            label_font_size,
            label_stroke_width,
        },
        RingDef {
            id: "ring-9",
            radius: 6.0,
            fill: "black",
            stroke: "white",
            stroke_width,
            label_text: "9",
            label_color: "white",
            label_font_size,
            label_stroke_width,
        },
        RingDef {
            id: "ring-10",
            radius: 3.0,
            fill: "black",
            stroke: "white",
            stroke_width,
            label_text: "10",
            label_color: "white",
            label_font_size,
            label_stroke_width,
        },
        RingDef {
            id: "ring-x",
            radius: 1.5,
            fill: "white",
            stroke: "black",
            stroke_width,
            label_text: "X",
            label_color: "black",
            label_font_size,
            label_stroke_width,
        },
    ];
    let mut rings = Vec::new();
    for (i, ring_def) in ring_defs.iter().enumerate() {
        let label_horizontal_offset = if i == ring_defs.len() - 1 {
            0.0
        } else {
            let next_ring_radius = ring_defs[i + 1].radius;
            next_ring_radius + (ring_def.radius - next_ring_radius) / 2.0
        };
        rings.push(target_ring(
            ring_def.id,
            ring_def.radius * UNIT_SCALE_FACTOR,
            ring_def.fill,
            ring_def.stroke,
            ring_def.stroke_width,
            ring_def.label_text,
            ring_def.label_color,
            ring_def.label_font_size,
            ring_def.label_stroke_width,
            label_horizontal_offset * UNIT_SCALE_FACTOR,
        ));
    }
    let target = html!({ for rings });

    let background_width = 60.0 * UNIT_SCALE_FACTOR;
    let background_height = 60.0 * UNIT_SCALE_FACTOR;

    let background_left = -background_width / 2.0;
    let background_top = -background_height / 2.0;

    let background = html!(
        <rect
            fill="lightgray"
            x={background_left.to_string()}
            y={background_top.to_string()}
            width={background_width.to_string()}
            height={background_height.to_string()}
        />
    );

    let mut sighters = Vec::new();
    let mut scored_shots = Vec::new();
    let mut shot_hover_boxes = Vec::new();
    let mut shot_hover_box_css = Vec::new();
    let mut velocities = Vec::new();

    let mut min_scored_x = None;
    let mut min_scored_y = None;
    let mut max_scored_x = None;
    let mut max_scored_y = None;
    let mut min_x = None;
    let mut min_y = None;
    let mut max_x = None;
    let mut max_y = None;

    for shot in &*props.shot_string_shots {
        velocities.push(shot.velocity.fps);

        let shot_x = shot_x(shot, UNIT_SCALE_FACTOR);
        // SVG and ShotMarker y-axis are inverted relative to each other.
        let shot_y = shot_y(shot, UNIT_SCALE_FACTOR);

        let curr_min_x = min_x.get_or_insert(shot_x);
        if shot_x < *curr_min_x {
            *curr_min_x = shot_x;
        }
        let curr_min_y = min_y.get_or_insert(shot_y);
        if shot_y < *curr_min_y {
            *curr_min_y = shot_y;
        }
        let curr_max_x = max_x.get_or_insert(shot_x);
        if shot_x > *curr_max_x {
            *curr_max_x = shot_x;
        }
        let curr_max_y = max_y.get_or_insert(shot_y);
        if shot_y > *curr_max_y {
            *curr_max_y = shot_y;
        }

        if !shot.tags.contains("sighter") {
            let curr_min_x = min_scored_x.get_or_insert(shot_x);
            if shot_x < *curr_min_x {
                *curr_min_x = shot_x;
            }
            let curr_min_y = min_scored_y.get_or_insert(shot_y);
            if shot_y < *curr_min_y {
                *curr_min_y = shot_y;
            }
            let curr_max_x = max_scored_x.get_or_insert(shot_x);
            if shot_x > *curr_max_x {
                *curr_max_x = shot_x;
            }
            let curr_max_y = max_scored_y.get_or_insert(shot_y);
            if shot_y > *curr_max_y {
                *curr_max_y = shot_y;
            }
        }
    }

    let (view_left, view_top, view_width, view_height) = if let (
        Some(min_x),
        Some(min_y),
        Some(max_x),
        Some(max_y),
    ) = (min_x, min_y, max_x, max_y)
    {
        let view_max_dimension = (max_x - min_x).max(max_y - min_y);
        let view_center_x = (max_x + min_x) / 2.0;
        let view_center_y = (max_y + min_y) / 2.0;
        let view_width = view_max_dimension * 1.5;
        let view_height = view_max_dimension * 1.5;
        let view_left = view_center_x - view_width / 2.0;
        let view_top = view_center_y - view_height / 2.0;

        (view_left, view_top, view_width, view_height)
    } else {
        (background_left, background_top, background_width, background_height)
    };
    let view_box = format!("{view_left} {view_top} {view_width} {view_height}");

    let velocity_avg = mean(&velocities);
    let velocity_sd = std_deviation(&velocities);
    let velocity_min = velocities.iter().fold(f64::MAX, |acc, &v| f64::min(acc, v as f64));
    let velocity_max = velocities.iter().fold(f64::MIN, |acc, &v| f64::max(acc, v as f64));
    let velocity_es = velocity_max - velocity_min;

    let (hover_box_top, hover_box_left, hover_box_width, hover_box_height) = {
        let hover_box_width = 0.20 * view_width;
        let hover_box_height = 0.15 * view_height;
        let hover_box_left = view_left;
        let hover_box_top = view_top;

        (hover_box_top, hover_box_left, hover_box_width, hover_box_height)
    };
    let hover_box_text_line_height = hover_box_height / 7.0;
    let hover_box_font_size = 0.75 * hover_box_text_line_height;
    let hover_box_text_x = hover_box_left + hover_box_width * 0.05;
    let hover_box_line_one_y = hover_box_top + hover_box_text_line_height;
    let hover_box_line_two_y = hover_box_line_one_y + hover_box_text_line_height;
    let hover_box_line_three_y = hover_box_line_two_y + hover_box_text_line_height;
    let hover_box_line_four_y = hover_box_line_three_y + hover_box_text_line_height;
    let hover_box_line_five_y = hover_box_line_four_y + hover_box_text_line_height;
    let hover_box_line_six_y = hover_box_line_five_y + hover_box_text_line_height;

    for shot in &*props.shot_string_shots {
        let marker_radius = 0.308 / 2.0 * UNIT_SCALE_FACTOR;
        let shot_x = shot_x(shot, UNIT_SCALE_FACTOR);
        // SVG and ShotMarker y-axis are inverted relative to each other.
        let shot_y = shot_y(shot, UNIT_SCALE_FACTOR);
        let marker_stroke_width = 0.3;

        let stroke_color = if shot.tags == "sighter" {
            "DimGray"
        } else {
            "Gainsboro"
        };
        let shot_circle = html!(
            <circle
                cx={shot_x.to_string()}
                cy={shot_y.to_string()}
                r={marker_radius.to_string()}
                fill="black"
                stroke={stroke_color}
                stroke-width={marker_stroke_width.to_string()}
            />
        );

        let shot_label = html!(
            <text
                x={shot_x.to_string()}
                y={shot_y.to_string()}
                fill="white"
                font-size="2"
                stroke="black"
                stroke-width="0.03"
                text-anchor="middle"
                dominant-baseline="middle"
                align-baseline="middle"
            >
                { shot.shot_id.to_string() }
            </text>
        );

        let css_shot_id = format!("shot_{}", shot.shot_id);
        let css_shot_hover_id = format!("shot_hover_{}", shot.shot_id);

        let shot_hover = {
            let shot_deviation_fps = shot.velocity.fps as f64 - velocity_avg;

            html!(
                <g id={css_shot_hover_id.clone()} class="shot_hover">
                    <rect
                        fill="DarkSlateGray"
                        x={hover_box_left.to_string()}
                        y={hover_box_top.to_string()}
                        width={hover_box_width.to_string()}
                        height={hover_box_height.to_string()}
                    />
                    <text
                        x={hover_box_text_x.to_string()}
                        y={hover_box_line_one_y.to_string()}
                        fill="white"
                        font-size={hover_box_font_size.to_string()}
                        stroke="white"
                        stroke-width="0.05"
                        text-anchor="left"
                        dominant-baseline="bottom"
                        align-baseline="bottom"
                    >
                        { format!("Shot: {}", shot.shot_id) }
                    </text>
                    <text
                        x={hover_box_text_x.to_string()}
                        y={hover_box_line_two_y.to_string()}
                        fill="white"
                        font-size={hover_box_font_size.to_string()}
                        stroke="white"
                        stroke-width="0.05"
                        text-anchor="left"
                        dominant-baseline="bottom"
                        align-baseline="bottom"
                    >
                        { format!("Score: {}", shot.score) }
                    </text>
                    <text
                        x={hover_box_text_x.to_string()}
                        y={hover_box_line_three_y.to_string()}
                        fill="white"
                        font-size={hover_box_font_size.to_string()}
                        stroke="white"
                        stroke-width="0.05"
                        text-anchor="left"
                        dominant-baseline="bottom"
                        align-baseline="bottom"
                    >
                        { format!("{} in., {} in.", shot.position.inch.x, shot.position.inch.y) }
                    </text>
                    <text
                        x={hover_box_text_x.to_string()}
                        y={hover_box_line_four_y.to_string()}
                        fill="white"
                        font-size={hover_box_font_size.to_string()}
                        stroke="white"
                        stroke-width="0.05"
                        text-anchor="left"
                        dominant-baseline="bottom"
                        align-baseline="bottom"
                    >
                        { format!("{} fps", shot.velocity.fps) }
                    </text>
                    <text
                        x={hover_box_text_x.to_string()}
                        y={hover_box_line_five_y.to_string()}
                        fill="white"
                        font-size={hover_box_font_size.to_string()}
                        stroke="white"
                        stroke-width="0.05"
                        text-anchor="left"
                        dominant-baseline="bottom"
                        align-baseline="bottom"
                    >
                        { format!("Avg: {:.2} ({:+.2})", velocity_avg, shot_deviation_fps) }
                    </text>
                    <text
                        x={hover_box_text_x.to_string()}
                        y={hover_box_line_six_y.to_string()}
                        fill="white"
                        font-size={hover_box_font_size.to_string()}
                        stroke="white"
                        stroke-width="0.05"
                        text-anchor="left"
                        dominant-baseline="bottom"
                        align-baseline="bottom"
                    >
                        { format!("SD: {:.2}, ES: {}", velocity_sd, velocity_es) }
                    </text>
                </g>
            )
        };

        let shot_group = html!(
            <g id={css_shot_id.clone()} class="shot">
                { shot_circle }
                { shot_label }
            </g>
        );

        shot_hover_boxes.push(shot_hover);
        shot_hover_box_css
            .push(format!("#{css_shot_id}:hover ~ #{css_shot_hover_id} {{ display: block; }}"));

        if shot.tags.contains("sighter") {
            sighters.push(shot_group);
        } else {
            scored_shots.push(shot_group);
        }
    }

    let scored_shot_bounding_box =
        if let (Some(min_scored_x), Some(min_scored_y), Some(max_scored_x), Some(max_scored_y)) =
            (min_scored_x, min_scored_y, max_scored_x, max_scored_y)
        {
            html!(
                <rect
                    id="scored_shot_bounding_box"
                    fill="DeepSkyBlue"
                    fill-opacity="0.40"
                    x={min_scored_x.to_string()}
                    y={min_scored_y.to_string()}
                    width={(max_scored_x - min_scored_x).to_string()}
                    height={(max_scored_y - min_scored_y).to_string()}
                />
            )
        } else {
            html!()
        };

    html!(
        <>
            <style>
                { ".shot_hover { display: none; }" }
                { for shot_hover_box_css }
            </style>
            <svg
                viewBox={view_box}
                zoomAndPan="magnify"
                height={render_height.to_string()}
                width={render_width.to_string()}
            >
                { background }
                { target }
                { scored_shot_bounding_box }
                { for sighters }
                { for scored_shots }
                { for shot_hover_boxes }
            </svg>
        </>
    )
}

fn shot_x(shot: &ShotMarkerShot, scale_factor: f64) -> f64 { shot.position.inch.x * scale_factor }

fn shot_y(shot: &ShotMarkerShot, scale_factor: f64) -> f64 { -shot.position.inch.y * scale_factor }

#[allow(clippy::too_many_arguments)]
fn target_ring(
    id: &str,
    radius: f64,
    fill: &str,
    stroke: &str,
    stroke_width: f64,
    label_text: &str,
    label_color: &str,
    font_size: f64,
    label_stroke_width: f64,
    label_horizontal_offset: f64,
) -> Html {
    let circle = html!(
        <circle
            cx="0"
            cy="0"
            r={radius.to_string()}
            fill={fill.to_string()}
            stroke={stroke.to_string()}
            stroke-width={stroke_width.to_string()}
        />
    );
    let label =
        ring_label(label_text, label_color, font_size, label_stroke_width, label_horizontal_offset);
    let negative_label = if label_horizontal_offset != 0.0 {
        ring_label(label_text, label_color, font_size, label_stroke_width, -label_horizontal_offset)
    } else {
        html!()
    };

    html!(
        <g id={id.to_string()}>
            { circle }
            { label }
            { negative_label }
        </g>
    )
}

fn ring_label(
    label: &str,
    color: &str,
    font_size: f64,
    label_stroke_width: f64,
    horizontal_offset: f64,
) -> Html {
    html!(
        <text
            x={horizontal_offset.to_string()}
            y="0"
            text-anchor="middle"
            dominant-baseline="middle"
            align-baseline="middle"
            font-size={font_size.to_string()}
            stroke-width={label_stroke_width.to_string()}
            fill={color.to_string()}
            stroke={color.to_string()}
        >
            { label.to_string() }
        </text>
    )
}

fn mean(data: &[u32]) -> f64 {
    let sum = data.iter().sum::<u32>() as f64;
    let count = data.len();

    sum / count as f64
}

fn std_deviation(data: &[u32]) -> f64 {
    let data_mean = mean(data);
    let count = data.len();
    let variance = data
        .iter()
        .map(|value| {
            let diff = data_mean - (*value as f64);

            diff * diff
        })
        .sum::<f64>()
        / count as f64;

    variance.sqrt()
}
