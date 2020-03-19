use {std::{str::*}, framework::*};

pub fn points(path: &str) -> Result<Vec<int2>> {
    let file = std::fs::read(path)?;
    let mut lines = from_utf8(&file)?.lines();
    let mut points = Vec::with_capacity(lines.next().ok()?.parse()?);
    points.try_extend( lines.enumerate().map::<Result<_>,_>(|(i, line)| {
        let fields = line.split_whitespace().map(i32::from_str).collect::<Result<Vec<_>, _>>()?;
        println!("{}, {:?}, {:?}", line, line.split(' '), fields);
        let (index, x, y) = (fields[0] as usize, fields[1], fields[2]);
        assert!(index-1 == i);
        Ok(int2{x,y})
    } ) )?;
    Ok(points)
}

fn main() -> Result {
    println!("{:?}", points("data/PointsA.txt"));
    Ok(())
}
