pub const TEST_INPUT: &str = "\
498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

pub const TEST_OUTPUT_CAVE: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
........#.
#########.\
";

pub const TEST_OUTPUT_DROP_ONE_SAND: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
......o.#.
#########.\
";

pub const TEST_OUTPUT_DROP_TWO_SAND: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
........#.
.....oo.#.
#########.\
";

pub const TEST_OUTPUT_DROP_FIVE_SAND: &str = "\
......+...
..........
..........
..........
....#...##
....#...#.
..###...#.
......o.#.
....oooo#.
#########.\
";