use chrono::{
    Datelike,
    NaiveDate,
    NaiveTime,
};
use nom::{
    IResult,
    Parser,
    branch::alt,
    bytes::complete::{
        tag,
        tag_no_case,
        take_till,
        take_until,
    },
    character::complete::space1,
    combinator::{
        opt,
        value,
    },
    error::{
        ErrorKind,
        FromExternalError,
        context,
    },
    multi::{
        many_m_n,
        many0,
    },
    number::complete::double,
    sequence::terminated,
};

use crate::{
    ShotMarkerExport,
    error::ShotMarkerCsvParserError,
    parser::util::ws,
    string::{
        ShotMarkerShotString,
        ShotMarkerStringMetrics,
        shot::{
            ShotMarkerShot,
            ShotPosition,
            ShotScore,
            ShotVelocity,
            ShotXYinch,
            ShotXYmil,
            ShotXYmm,
            ShotXYmoa,
        },
    },
};

#[cfg(test)]
mod tests;
pub(crate) mod util;

pub fn export_parser(input: &str) -> IResult<&str, ShotMarkerExport> {
    let (rest, _) =
        context("Parsing start of export", ws(tag_no_case("ShotMarker Archived Data (generated")))
            .parse(input)?;
    // let (rest, _) = nom::character::complete::space1(rest)?;
    let (rest, generated_date) = context("Parsing generated date", ws(date_parser)).parse(rest)?;
    // let (rest, _) = nom::character::complete::space0(rest)?;
    let (rest, _) = ws(tag(")")).parse(rest)?;
    let (rest, _) = ws(tag_no_case("Exported")).parse(rest)?;
    let (rest, string_count) =
        ws(context("Parsing number of exported shot strings", nom::character::complete::usize))
            .parse(rest)?;
    let (rest, _) = ws(alt((tag("strings"), tag("string")))).parse(rest)?;
    let (rest, _) = ws(tag_no_case("from")).parse(rest)?;
    let (rest, string_date) = month_and_day_parser(generated_date.year(), rest)?;
    let (rest, _) = ws(tag_no_case("of")).parse(rest)?;
    let (rest, _) =
        ws(context("Parsing total shot strings in archive", nom::character::complete::u32))
            .parse(rest)?;
    let (rest, _) = ws(tag_no_case("total in archive")).parse(rest)?;
    let (rest, strings) = shot_strings_parser(rest)?;

    if string_count != strings.len() {
        return Err(nom::Err::Failure(nom::error::Error::from_external_error(
            input,
            ErrorKind::Verify,
            ShotMarkerCsvParserError::UnexpectedStringCount {
                expected: string_count,
                found:    strings.len(),
            },
        )));
    }

    let shot_marker_export = ShotMarkerExport {
        generated_date,
        string_count,
        string_date,
        strings,
    };

    Ok((rest, shot_marker_export))
}

fn shot_strings_parser(input: &str) -> IResult<&str, Vec<ShotMarkerShotString>> {
    let (rest, strings) = many0(shot_string_parser).parse(input)?;

    Ok((rest, strings))
}

fn shot_string_parser(input: &str) -> IResult<&str, ShotMarkerShotString> {
    let (rest, (date, name, target, distance, score)) =
        ws(shot_string_header_parser).parse(input)?;
    let (rest, shots) = many0(ws(shot_string_entry_parser)).parse(rest)?;
    let (rest, metrics) = opt(ws(shot_string_metrics_parser)).parse(rest)?;

    let shot_string = ShotMarkerShotString {
        date,
        name,
        target,
        distance,
        score,
        shots,
        metrics,
    };

    Ok((rest, shot_string))
}

/// Parses the shot string header into:
///   Date, Name, Target, Distance, Score
fn shot_string_header_parser(
    input: &str,
) -> IResult<&str, (NaiveDate, String, String, String, String)> {
    let (rest, date) = ws(date_parser).parse(input)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, name) = ws(take_until(",")).parse(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, target) = ws(take_until(",")).parse(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, distance) = ws(take_until(",")).parse(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, score) = ws(take_till(|c| c == '\r' || c == '\n')).parse(rest)?;

    // Individual shot field header.
    let (rest, _) = ws(tag(",time,id,tags,score,x (mm),y (mm),x (inch),y (inch),x (moa),y (moa),x (mil),y (mil),v (m/s),v (fps),yaw (deg), pitch (deg),quality")).parse(rest)?;

    Ok((rest, (date, name.to_owned(), target.to_owned(), distance.to_owned(), score.to_owned())))
}

fn shot_string_metrics_parser(input: &str) -> IResult<&str, ShotMarkerStringMetrics> {
    let (rest, _) = ws(tag_no_case(",,,,avg:,")).parse(input)?;
    let (rest, _) = ws(many_m_n(13, 13, terminated(ws(double), tag(",")))).parse(rest)?;
    let (rest, _) = ws(tag_no_case(",,,,SD:,")).parse(rest)?;
    let (rest, _) = ws(many_m_n(13, 13, terminated(ws(double), tag(",")))).parse(rest)?;

    Ok((rest, ShotMarkerStringMetrics {}))
}

fn shot_string_entry_parser(input: &str) -> IResult<&str, ShotMarkerShot> {
    let (rest, _) = tag(",")(input)?;
    let (rest, time) = time_parser(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, id) = take_until(",")(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, tags) = take_until(",")(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, score) = parse_shot_score(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, x_mm) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y_mm) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, x_inch) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y_inch) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, x_moa) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y_moa) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, x_mil) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, y_mil) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, velocity_ms) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, velocity_fps) = nom::character::complete::u32(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, yaw_degrees) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, pitch_degrees) = double(rest)?;
    let (rest, _) = tag(",")(rest)?;
    let (rest, quality) = opt(double).parse(rest)?;
    let (rest, _) = tag(",")(rest)?;

    let shot = ShotMarkerShot {
        time,
        id: id.to_string(),
        tags: tags.to_string(),
        score,
        position: ShotPosition {
            mm:   ShotXYmm {
                x: x_mm,
                y: y_mm,
            },
            inch: ShotXYinch {
                x: x_inch,
                y: y_inch,
            },
            moa:  ShotXYmoa {
                x: x_moa,
                y: y_moa,
            },
            mil:  ShotXYmil {
                x: x_mil,
                y: y_mil,
            },
        },
        velocity: ShotVelocity {
            ms:  velocity_ms,
            fps: velocity_fps,
        },
        yaw: yaw_degrees,
        pitch: pitch_degrees,
        quality,
    };

    Ok((rest, shot))
}

fn time_parser(input: &str) -> IResult<&str, NaiveTime> {
    let (rest, mut hours) = nom::character::complete::u32(input)?;
    let (rest, _) = tag(":")(rest)?;
    let (rest, minutes) = nom::character::complete::u32(rest)?;
    let (rest, _) = tag(":")(rest)?;
    let (rest, seconds) = nom::character::complete::u32(rest)?;
    let (rest, _) = space1(rest)?;
    let (rest, is_pm) =
        alt((value(false, tag_no_case("am")), value(true, tag_no_case("pm")))).parse(rest)?;

    if is_pm {
        hours += 12;
    }

    let parsed_time = NaiveTime::from_hms_opt(hours, minutes, seconds).ok_or(nom::Err::Failure(
        nom::error::Error::from_external_error(
            input,
            ErrorKind::Verify,
            ShotMarkerCsvParserError::InvalidTime {
                hours,
                minutes,
                seconds,
                is_pm,
            },
        ),
    ))?;

    Ok((rest, parsed_time))
}

fn month_and_day_parser(year: i32, input: &str) -> IResult<&str, NaiveDate> {
    let (rest, month) = month_parser(input)?;
    let (rest, _) = nom::character::complete::space1(rest)?;
    let (rest, day) = month_day_parser(rest)?;

    let parsed_date = NaiveDate::from_ymd_opt(year, month, day).ok_or(nom::Err::Failure(
        nom::error::Error::from_external_error(
            input,
            ErrorKind::Verify,
            ShotMarkerCsvParserError::InvalidDate {
                year,
                month,
                day,
            },
        ),
    ))?;

    Ok((rest, parsed_date))
}

fn date_parser(input: &str) -> IResult<&str, NaiveDate> {
    let (rest, month) = month_parser(input)?;
    let (rest, _) = nom::character::complete::space1(rest)?;
    let (rest, day) = month_day_parser(rest)?;
    let (rest, _) = nom::character::complete::space1(rest)?;
    let (rest, year) = year_parser(rest)?;

    let parsed_date = NaiveDate::from_ymd_opt(year, month, day).ok_or(nom::Err::Failure(
        nom::error::Error::from_external_error(
            input,
            ErrorKind::Verify,
            ShotMarkerCsvParserError::InvalidDate {
                year,
                month,
                day,
            },
        ),
    ))?;

    Ok((rest, parsed_date))
}

fn month_parser(input: &str) -> IResult<&str, u32> {
    alt((
        value(1, tag_no_case("Jan")),
        value(2, tag_no_case("Feb")),
        value(3, tag_no_case("Mar")),
        value(4, tag_no_case("Apr")),
        value(5, tag_no_case("May")),
        value(6, tag_no_case("Jun")),
        value(7, tag_no_case("Jul")),
        value(8, tag_no_case("Aug")),
        value(9, tag_no_case("Sep")),
        value(10, tag_no_case("Oct")),
        value(11, tag_no_case("Nov")),
        value(12, tag_no_case("Dec")),
    ))
    .parse(input)
}

fn month_day_parser(input: &str) -> IResult<&str, u32> { nom::character::complete::u32(input) }

fn year_parser(input: &str) -> IResult<&str, i32> { nom::character::complete::i32(input) }

fn parse_shot_score(input: &str) -> IResult<&str, ShotScore> {
    let (rest, score) = opt(alt((parse_shot_score_x, parse_shot_score_numeric))).parse(input)?;

    let score = score.unwrap_or(ShotScore::None);

    Ok((rest, score))
}

fn parse_shot_score_x(input: &str) -> IResult<&str, ShotScore> {
    let (rest, _) = tag_no_case("X")(input)?;

    Ok((rest, ShotScore::X))
}

fn parse_shot_score_numeric(input: &str) -> IResult<&str, ShotScore> {
    let (rest, score) = nom::character::complete::u8(input)?;

    Ok((rest, ShotScore::Numeric(score)))
}
