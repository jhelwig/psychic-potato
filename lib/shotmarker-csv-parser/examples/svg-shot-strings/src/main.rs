use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use log::{
    debug,
    info,
};
use shotmarker_csv_parser::parser::export_parser;
use svg::{
    Document,
    node::element::{
        Circle,
        Group,
        Rectangle,
        Text,
    },
};
use tracing_subscriber::{
    EnvFilter,
    prelude::*,
};

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    #[arg(short, long, value_name = "OUTDIR")]
    outdir: Option<PathBuf>,

    input: PathBuf,

    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
}

const UNIT_SCALE_FACTOR: f64 = 10.0;
const VIEWBOX_SIZE: f64 = 60.0;
const SCORE_LABEL_FONT_SIZE: u32 = 10;
const BULLET_SIZE: f64 = 0.308;
const SHOT_FONT_SIZE: u32 = 4;
const FONT_STROKE_WIDTH: f64 = 0.1;
const SHOT_STROKE_WIDTH: f64 = 0.05;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let fmt_layer = tracing_subscriber::fmt::layer().pretty();

    let verbose_level = match cli.verbose {
        0 => "info",
        1 => "debug",
        _ => "trace",
    };

    let filter_layer =
        EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new(verbose_level))?;

    tracing_subscriber::registry().with(filter_layer).with(fmt_layer).init();
    debug!("Log verbosity set to: {verbose_level}");

    let input_file = cli.input;
    info!("Processing {}", input_file.display());

    let outdir = match cli.outdir {
        Some(outdir) => outdir,
        None => std::env::current_dir()?,
    };
    info!("Output directory: {}", outdir.display());

    let input_file = std::fs::read_to_string(input_file)?;

    let (rest, export) = export_parser(&input_file)?;
    debug!("Unparsed file content: {rest:?}");
    debug!("Export generated {} with {} shot strings", export.generated_date, export.string_count);

    for shot_string in &export.strings {
        info!(
            "Shot string ({} {} {}): {}",
            shot_string.target, shot_string.distance, shot_string.score, shot_string.name
        );

        let ring_stroke_width = 2;
        let viewbox_min = -VIEWBOX_SIZE / 2.0;
        let viewbox_min_scaled = viewbox_min * UNIT_SCALE_FACTOR;
        let viewbox_size_scaled = VIEWBOX_SIZE * UNIT_SCALE_FACTOR;
        let mut svg_document = Document::new()
            .set(
                "viewBox",
                (viewbox_min_scaled, viewbox_min_scaled, viewbox_size_scaled, viewbox_size_scaled),
            )
            .set("zoomAndPan", "magnify");
        let five_ring_radius = 24.0 * UNIT_SCALE_FACTOR;
        let six_ring_radius = 18.0 * UNIT_SCALE_FACTOR;
        let seven_ring_radius = 12.0 * UNIT_SCALE_FACTOR;
        let eight_ring_radius = 9.0 * UNIT_SCALE_FACTOR;
        let nine_ring_radius = 6.0 * UNIT_SCALE_FACTOR;
        let ten_ring_radius = 3.0 * UNIT_SCALE_FACTOR;
        let x_ring_radius = 1.5 * UNIT_SCALE_FACTOR;
        let background = Rectangle::new()
            .set("x", viewbox_min_scaled)
            .set("y", viewbox_min_scaled)
            .set("width", viewbox_size_scaled)
            .set("height", viewbox_size_scaled)
            .set("fill", "lightgray")
            .set("stroke", "lightgray");
        let five_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", five_ring_radius)
                .set("fill", "white")
                .set("stroke", "black")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("5")
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let horizontal_offset = six_ring_radius + (five_ring_radius - six_ring_radius) / 2.0;
            let left_label = label_text.clone().set("x", -horizontal_offset).set("y", 0);
            let right_label = label_text.set("x", horizontal_offset).set("y", 0);

            Group::new().set("id", "ring-5").add(ring).add(left_label).add(right_label)
        };
        let six_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", six_ring_radius)
                .set("fill", "black")
                .set("stroke", "white")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("6")
                .set("fill", "white")
                .set("stroke", "white")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let horizontal_offset = seven_ring_radius + (six_ring_radius - seven_ring_radius) / 2.0;
            let left_label = label_text.clone().set("x", -horizontal_offset).set("y", 0);
            let right_label = label_text.set("x", horizontal_offset).set("y", 0);

            Group::new().set("id", "ring-6").add(ring).add(left_label).add(right_label)
        };
        let seven_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", seven_ring_radius)
                .set("fill", "black")
                .set("stroke", "white")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("7")
                .set("fill", "white")
                .set("stroke", "white")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let horizontal_offset =
                eight_ring_radius + (seven_ring_radius - eight_ring_radius) / 2.0;
            let left_label = label_text.clone().set("x", -horizontal_offset).set("y", 0);
            let right_label = label_text.set("x", horizontal_offset).set("y", 0);

            Group::new().set("id", "ring-7").add(ring).add(left_label).add(right_label)
        };
        let eight_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", eight_ring_radius)
                .set("fill", "black")
                .set("stroke", "white")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("8")
                .set("fill", "white")
                .set("stroke", "white")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let horizontal_offset = nine_ring_radius + (eight_ring_radius - nine_ring_radius) / 2.0;
            let left_label = label_text.clone().set("x", -horizontal_offset).set("y", 0);
            let right_label = label_text.set("x", horizontal_offset).set("y", 0);

            Group::new().set("id", "ring-8").add(ring).add(left_label).add(right_label)
        };
        let nine_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", nine_ring_radius)
                .set("fill", "black")
                .set("stroke", "white")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("9")
                .set("fill", "white")
                .set("stroke", "white")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let horizontal_offset = ten_ring_radius + (nine_ring_radius - ten_ring_radius) / 2.0;
            let left_label = label_text.clone().set("x", -horizontal_offset).set("y", 0);
            let right_label = label_text.set("x", horizontal_offset).set("y", 0);

            Group::new().set("id", "ring-9").add(ring).add(left_label).add(right_label)
        };
        let ten_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", ten_ring_radius)
                .set("fill", "black")
                .set("stroke", "white")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("10")
                .set("fill", "white")
                .set("stroke", "white")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let horizontal_offset = x_ring_radius + (ten_ring_radius - x_ring_radius) / 2.0;
            let left_label = label_text.clone().set("x", -horizontal_offset).set("y", 0);
            let right_label = label_text.set("x", horizontal_offset).set("y", 0);

            Group::new().set("id", "ring-10").add(ring).add(left_label).add(right_label)
        };
        let x_ring = {
            let ring = Circle::new()
                .set("cx", 0.0)
                .set("cy", 0.0)
                .set("r", x_ring_radius)
                .set("fill", "white")
                .set("stroke", "lightgrey")
                .set("stroke-width", ring_stroke_width);
            let label_text = Text::new("X")
                .set("fill", "black")
                .set("stroke", "black")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SCORE_LABEL_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle")
                .set("x", 0)
                .set("y", 0);

            Group::new().set("id", "ring-x").add(ring).add(label_text)
        };
        let score = Text::new(shot_string.score.to_string())
            .set("x", 0.0)
            .set("y", -28.0 * UNIT_SCALE_FACTOR)
            .set("fill", "white")
            .set("stroke", "white")
            .set("stroke-width", FONT_STROKE_WIDTH)
            .set("font-size", SCORE_LABEL_FONT_SIZE)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("vertical-align", "middle");
        let name = Text::new(&shot_string.name)
            .set("x", 0.0)
            .set("y", -29.0 * UNIT_SCALE_FACTOR)
            .set("fill", "white")
            .set("stroke", "white")
            .set("stroke-width", FONT_STROKE_WIDTH)
            .set("font-size", SCORE_LABEL_FONT_SIZE)
            .set("text-anchor", "middle")
            .set("dominant-baseline", "middle")
            .set("vertical-align", "middle");

        svg_document = svg_document
            .add(background)
            .add(five_ring)
            .add(six_ring)
            .add(seven_ring)
            .add(eight_ring)
            .add(nine_ring)
            .add(ten_ring)
            .add(x_ring)
            .add(name)
            .add(score);

        let mut sighters = Vec::new();
        let mut scored = Vec::new();
        let mut min_x = None;
        let mut min_y = None;
        let mut max_x = None;
        let mut max_y = None;
        for shot in &shot_string.shots {
            debug!(
                "Adding shot {} (Score: {}) at position ({}, {})",
                shot.id, shot.score, shot.position.inch.x, shot.position.inch.y
            );

            let shot_x = shot.position.inch.x * UNIT_SCALE_FACTOR;
            // SVG and ShotMarker y-axis are inverted relative to each other.
            let shot_y = shot.position.inch.y * -UNIT_SCALE_FACTOR;

            let shot_circle = {
                let mut circle = Circle::new()
                    .set("cx", shot_x)
                    .set("cy", shot_y)
                    .set("r", BULLET_SIZE * UNIT_SCALE_FACTOR)
                    .set("stroke", "black")
                    .set("stroke-width", SHOT_STROKE_WIDTH);
                if shot.tags == "sighter" {
                    circle = circle.set("fill", "DimGray");
                } else {
                    circle = circle.set("fill", "DarkGray");
                }
                circle
            };
            let shot_label = Text::new(&shot.id)
                .set("x", shot_x)
                .set("y", shot_y)
                .set("fill", "blue")
                .set("stroke", "blue")
                .set("stroke-width", FONT_STROKE_WIDTH)
                .set("font-size", SHOT_FONT_SIZE)
                .set("text-anchor", "middle")
                .set("dominant-baseline", "middle");
            let shot_group = Group::new()
                .set("id", format!("shot-{}", shot.id))
                .add(shot_circle)
                .add(shot_label);

            if shot.tags == "sighter" {
                sighters.push(shot_group);
            } else {
                scored.push(shot_group);

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
            }
        }

        // Layer sighters first so the bounding box of scored shots, and the scored shots
        // themselves are in front of the sighters.
        let mut sighters_group = Group::new().set("id", "sighters");
        for sighter in sighters {
            sighters_group = sighters_group.add(sighter);
        }
        svg_document = svg_document.add(sighters_group);
        if let (Some(min_x), Some(min_y), Some(max_x), Some(max_y)) = (min_x, min_y, max_x, max_y) {
            let bounds_width = max_x - min_x;
            let bounds_height = max_y - min_y;

            let bounds_box = Rectangle::new()
                .set("x", min_x)
                .set("y", min_y)
                .set("width", bounds_width)
                .set("height", bounds_height)
                .set("fill", "blue")
                .set("fill-opacity", "0.40");

            svg_document = svg_document.add(bounds_box);
        }
        let mut scored_group = Group::new().set("id", "scored");
        for scored_shot in scored {
            scored_group = scored_group.add(scored_shot);
        }
        svg_document = svg_document.add(scored_group);

        let svg_file_name = format!(
            "{date}-{name}-{score}.svg",
            date = shot_string.date,
            name = shot_string.name,
            score = shot_string.score
        );
        info!("Writing SVG file for shot string {}", svg_file_name);
        svg::save(&svg_file_name, &svg_document)?;

        // Write to a string?
        let mut svg_string_vec = Vec::new();
        svg::write(&mut svg_string_vec, &svg_document)?;
        let svg_string = String::from_utf8(svg_string_vec)?;
        info!("Written SVG string length: {}", svg_string.len());
    }

    Ok(())
}
