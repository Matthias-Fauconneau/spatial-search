#![feature(iter_order_by)]
use {std::{str::*}, framework::{*, Image as Grid}};

pub fn points(path: &str) -> Result<Vec<vec2>> {
    let file = std::fs::read(path)?;
    let mut lines = from_utf8(&file)?.lines();
    let mut points = Vec::with_capacity(lines.next().ok()?.parse()?);
    points.try_extend( lines.enumerate().map::<Result<_>,_>(|(i, line)| {
        let fields = line.split_whitespace().map(i32::from_str).collect::<Result<Vec<_>, _>>()?;
        let (index, x, y) = (fields[0] as usize, fields[1] as f32, fields[2] as f32);
        assert!(index-1 == i);
        Ok(vec2{x,y})
    } ) )?;
    Ok(points)
}

fn main() -> Result {
    let radius = from_utf8(&std::fs::read("data/Radius.txt")?)?.lines().next().ok()?.parse()?;
    let candidates = points("data/PointsA.txt")?;
    let queries = points("data/PointsB.txt")?;
    let direct = queries.iter().map(|&query| { candidates.iter().enumerate().filter(move |(_, &candidate)| { sq(candidate-query) < radius } ).map(|(id,_)|id) }); // O(N)
    // Extents
    let min = candidates.iter().fold(f32::MAX.into(), |m, &p| min(m, p) );
    let max = candidates.iter().fold(f32::MIN.into(), |m, &p| max(m, p) );
    // O(√N) with uniformly distributed positions
    let n = sqrt(sqrt(candidates.len() as f32)) as u32; // 10K points => Single level grid of 100 bins × ~100 point/bin => 10x10 grid
    assert!(n == 10);
    let grid_size = size2{ x:n, y: n };
    let bin_coords = |point| uint2::from( (point-min) / (max-min) * (vec2::from(grid_size)-1.0.into()) );
    // Accumulate bin point counts
    let mut counts = Grid::zero(grid_size);
    for &point in &candidates { counts[bin_coords(point)] += 1; }
    let starts = Image::from_iter(grid_size, counts.iter().scan(0, |size, &count| { let start = *size; *size += count; Some(start) }));
    let mut binned_candidates = Vec::new();
    binned_candidates.resize_with(starts.last().ok()? + counts.last().ok()?, Zero::zero);
    // Bin points
    let mut counts = Grid::zero(grid_size); // Zero counts
    for (point_index, &point) in candidates.iter().enumerate() {
        let bin_index = counts.index(bin_coords(point));
        let count = &mut counts[bin_index];
        binned_candidates[starts[bin_index]+*count] = (point_index, point); // Indirect index is sufficient but inlining point position is probably more efficient for search
        *count += 1;
    }
    let grid = queries.iter().map(|&query| {
        // Evaluates search bounding box coordinates on grid
        let min = bin_coords(query-vec2::from(radius));
        let max = bin_coords(query+vec2::from(radius));
        assert!(max.x - min.x <= 3 && max.y - min.y <= 3);
        let mut neighbours = Vec::new(); //(todo opti: reserve a good average capacity for efficient collection)
        for y in min.y ..= max.y {
            for x in min.x ..= max.x {
                let bin_index = counts.index(uint2{x,y});
                for &(candidate_index, candidate) in &binned_candidates[starts[bin_index]..][..counts[bin_index]] {
                    if sq(candidate-query) < radius { neighbours.push( candidate_index ); }
                }
            }
        }
        neighbours
    });
    let grid = grid.map(|mut r|{r.sort(); r});
    assert!(direct.eq_by(grid, |a, b| a.eq(b)));
    Ok(())
}
