use chrono::prelude::*;
use image::ImageEncoder;
use std::collections::HashMap;
use std::str::FromStr;

fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    let cell_size = 5_usize.pow(2);

    let h = 27 * cell_size;
    let w = 23 * cell_size;
    let mut buf: Vec<u8> = vec![0; w * h * 3];
    for (i, item) in buf.iter_mut().enumerate() {
        *item = match i % 3 {
            0 => 74,
            1 => 87,
            2 => 99,
            _ => unreachable!(),
        }
    }
    let year = 2018;
    let mut date = Utc.ymd(year, 1, 1).naive_utc();
    let mut week_row = 0;
    let cell_size = cell_size as u32;
    while date.year() == year {
        let month_row = date.month0() / 3;
        let month_col = date.month0() % 3;
        let week_col = date.weekday().num_days_from_monday();

        let x = month_col * 8 * cell_size + week_col * cell_size;
        let y = month_row * 7 * cell_size + week_row * cell_size;

        let mood = moodmap.get(&date).unwrap_or(&Mood::Unknown);
        let (r, g, b) = mood.to_color();

        for y in y..(y + cell_size) {
            for x in x..(x + cell_size) {
                let index = (x as usize + y as usize * w) * 3;
                buf[index] = r;
                buf[index + 1] = g;
                buf[index + 2] = b;
            }
        }

        date += chrono::Duration::days(1);
        if date.weekday() == Weekday::Mon {
            week_row += 1;
        }
        if date.day() == 1 {
            week_row = 0;
        }
    }

    let writer = std::fs::File::create("image.png")?;
    let encoder = image::codecs::png::PngEncoder::new(writer);
    encoder.write_image(&buf, w as u32, h as u32, image::ColorType::Rgb8)?;
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
    fn to_color(self) -> (u8, u8, u8) {
        match self {
            Mood::Rad => (99, 158, 90),
            Mood::Good => (147, 199, 101),
            Mood::Meh => (74, 221, 187),
            Mood::Bad => (237, 153, 143),
            Mood::Awful => (253, 108, 110),
            _ => (75, 87, 99),
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
