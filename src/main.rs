fn main() -> Result<(), Box<std::error::Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(std::io::stdin());
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here.
        let record = result?;
        println!("{:?}", record);
    }

    let h = 10;
    let w = 10;
    let mut buf: Vec<u8> = Vec::with_capacity(w * h * 3);
    let mut r: u32 = 7;
    for _ in 0..(h * w * 3) {
        buf.push(r as u8);
        r = (r * 3) % 255;
    }

    let writer = std::fs::File::create("image.png")?;
    let encoder = image::png::PNGEncoder::new(writer);
    encoder.encode(&buf, h as u32, w as u32, image::ColorType::RGB(8))?;
    Ok(())
}
