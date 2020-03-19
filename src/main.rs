use {std::{str::*}, framework::*};

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
    let direct = queries.iter().map(|&query| { candidates.iter().enumerate().filter(move |(_, &candidate)| { sq(candidate-query) < radius } ) }); // Direct // O(AB) // ~1050ms
    //println!("{:?}", direct);
    /*// Grid // ~O(B√A) Assuming A uniformly distributed points. // 200ms // inner loop speedup should be 10K => ~10x100 ~ 10x
    const auto b = solve("data/Solution2.txt", [&]{
        const auto N = int(sqrt(sqrt(vPointListImageA.size()))); // 10K points => Single level grid of 100 cells × ~100 point/cell => 10x10 grid
        //assert(N == 10);
        auto countGrid = Grid<uint>(int2{ .x = N, .y = N }, 0);

        // Get input data value ranges
        auto min = int2::from(INT_MAX), max = int2::from(INT_MIN);
        for(const auto& a_: vPointListImageA) {
            const auto a = int2::from(a_);
            min = ::min(min, a);
            max = ::max(max, a);
        }
        const auto cell_index = [=](const auto p) { return (p-min) * (countGrid.size-int2::from(1)) / (max-min); };
        //error(min, max); // 1-300, radius=20, 10x10 x 30x30 grid ~ check ~9 cells

        // Computes maximum number of points in a cell
        for(const auto& a_: vPointListImageA) {
            const auto a = int2::from(a_);
            countGrid[cell_index(a)] += 1;
        }
        //error(countGrid); // max~140
        const auto maxCount = ::max(countGrid.buffer);

        auto grid = GridArray<bpPoint>(int2{ .x = N, .y = N }, maxCount, bpPoint());

        // Bin points in cells
        std::fill(countGrid.buffer.begin(), countGrid.buffer.end(), 0); // countGrid.buffer.fill(0);
        for(const auto& a_: vPointListImageA) {
            const auto a = int2::from(a_);
            const auto index = cell_index(a);
            auto& len = countGrid[index];
            grid(index, len) = a_;
            len += 1;
        }

        /*mut*/ map<int, list<int>> solution;
        for(const auto& b : vPointListImageB) {
            // Search bounding box on grid
            const auto min_cell = cell_index(int2::from(b)-int2::from(radius));
            const auto max_cell = cell_index(int2::from(b)+int2::from(radius));
            //error(min_cell, max_cell);
            assert(max_cell.x - min_cell.x <= 3);
            assert(max_cell.y - min_cell.y <= 3);

            /*mut*/ std::list<int> neighbours; // FIXME: vector
            for(const auto& y: range(min_cell.y, max_cell.y+1)) {
                for(const auto& x: range(min_cell.x, max_cell.x+1)) {
                    const auto& cell = int2{.x=int(x),.y=int(y)}; //uint2
                    for(const auto& i: range(0, countGrid[cell])) {
                        const auto& a = grid(cell, i); // Assumes CSE
                        const auto sqDistance = ::sqDistance(int2::from(a), int2::from(b));
                        if(sqDistance < sq(radius)) { // sq(radius): Assumes CSE
                            neighbours.push_back(a.GetId());
                        }
                    }
                }
            }
            neighbours.sort(); // Sort for external validation (diff)
            solution.insert(pair<int, list<int>>(b.GetId(), neighbours));
        }
        return solution;
    });

    for(auto a_: a) { // Internal validation
        auto b_ = b.at(a_.first/*key*/);
        a_.second.sort();
        //cout << a_.second << endl;
        b_.sort();
        //cout << b_ << endl;
        assert(a_.second/*value*/ == b_);
    }*/

    Ok(())
}
