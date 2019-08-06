use chrono::prelude::*;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), Box<std::error::Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    let mut moodmap = HashMap::with_capacity(365);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        let date = NaiveDate::parse_from_str(&record[0], "%Y-%m-%d")?;
        let mood: Mood = (&record[4]).parse()?;
        moodmap.insert(date, mood);
    }

    let h = 27;
    let w = 23;
    let mut buf: Vec<u8> = vec![0; w * h * 3];
    let mut date = Utc.ymd(2018, 1, 1).naive_utc();
    let mut week_row = 0;
    while date.year() == 2018 {
        let month_row = date.month0() / 3;
        let month_col = date.month0() % 3;
        let week_col = date.weekday().num_days_from_monday();

        let x = month_col * 8 + week_col;
        let y = month_row * 7 + week_row;
        let index = (x as usize + y as usize * w) * 3;

        let mood = moodmap.get(&date).unwrap_or(&Mood::Unknown);
        let (r, g, b) = mood.to_color();
        buf[index + 0] = r;
        buf[index + 1] = g;
        buf[index + 2] = b;

        date += chrono::Duration::days(1);
        if date.weekday() == Weekday::Mon {
            week_row += 1;
        }
        if date.day() == 1 {
            week_row = 0;
        }
    }

    let writer = std::fs::File::create("image.png")?;
    let encoder = image::png::PNGEncoder::new(writer);
    encoder.encode(&buf, w as u32, h as u32, image::ColorType::RGB(8))?;
    Ok(())
}

#[derive(Debug, Copy, Clone)]
enum Mood {
    Rad,
    Good,
    Meh,
    Bad,
    Awful,
    Unknown,
}

impl Mood {
    fn to_color(&self) -> (u8, u8, u8) {
        match self {
            Mood::Rad => (251, 149, 23),
            Mood::Good => (87, 175, 115),
            Mood::Meh => (125, 56, 151),
            Mood::Bad => (31, 77, 137),
            Mood::Awful => (46, 78, 92),
            _ => (0, 0, 0),
        }
    }
}

impl FromStr for Mood {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Mood, &'static str> {
        match s {
            "rad" | "T" => Ok(Mood::Rad),
            "good" | "Confused" => Ok(Mood::Good),
            "meh" => Ok(Mood::Meh),
            "bad" => Ok(Mood::Bad),
            "awful" => Ok(Mood::Awful),
            _ => Ok(Mood::Unknown),
        }
    }
}
