// Copyright 2021 Andrew Merenbach
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::cmp;
use derive_builder::Builder;
use itertools::Itertools;
use std::fmt;
use std::iter;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub const DEFAULT_COLUMNS: usize = 17;
pub const DEFAULT_ROWS: usize = 9;
const DEFAULT_LIMIT: usize = 64;
const DEFAULT_SYMBOLS: &str = " .o+=*B0X@%&#/^";

/// Draw a frame around a grid of cells with a given row width.
/// This will result in a misshapen grid if a row is undersized.
fn frame(cells: &[char], width: usize) -> String {
    let mut buf = String::new();
    let dashes: Vec<_> = iter::repeat('-').take(width).collect();

    buf.push('+');
    buf.extend(&dashes);
    buf.push('+');
    buf.push('\n');

    if width > 0 {
        for row in cells.chunks(width) {
            buf.push('|');
            buf.extend(row);
            buf.push('|');
            buf.push('\n');
        }
    }

    buf.push('+');
    buf.extend(&dashes);
    buf.push('+');

    buf
}

struct Move {
    right: bool,
    down: bool,
}

impl From<u8> for Move {
    fn from(v: u8) -> Self {
        // v & 0b11
        Move {
            right: v & 0b01 != 0,
            down: v & 0b10 != 0,
        }
    }
}

// This is more testable if it's broken out
fn moves_from_byte(b: u8) -> Vec<Move> {
    (0..8).step_by(2).map(|i| b >> i).map(Move::from).collect()
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct DrunkenBishop {
    data: Vec<u8>,

    #[builder(default = "DEFAULT_ROWS")]
    rows: usize,
    #[builder(default = "DEFAULT_COLUMNS")]
    columns: usize,

    #[builder(default = "DEFAULT_LIMIT")]
    limit: usize, // max number of moves to run (0 for unlimited; default is 64)
    #[builder(default = "DEFAULT_SYMBOLS.chars().collect()")]
    symbols: Vec<char>,
    #[builder(setter(strip_option))]
    home: Option<usize>, // starting location in the grid
    cycle: bool, // whether to cycle symbols or only go as far as the last
}

impl DrunkenBishop {
    /// Moves that will be performed with the grid.
    fn moves(&self, data: &[u8], start_idx: usize) -> Vec<usize> {
        data.iter()
            // Convert message bytes to sequence of move instructions
            .flat_map(|&b| moves_from_byte(b))
            // Constrain sequence of visited cells to the desired number of moves
            .take(if self.limit == 0 {
                // four moves per byte multiplied by byte length of message
                4 * data.len()
            } else {
                self.limit
            })
            // Apply moves to start position to create numeric sequence of visited cell positions
            .scan(self.position_to_coordinates(start_idx), |(x, y), m| {
                if m.right {
                    *x = cmp::min(x.saturating_add(1), self.columns - 1)
                } else {
                    *x = x.saturating_sub(1)
                }

                if m.down {
                    *y = cmp::min(y.saturating_add(1), self.rows - 1)
                } else {
                    *y = y.saturating_sub(1)
                }

                Some(self.coordinates_to_position(*x, *y))
            })
            .collect()
    }

    /// Render the grid into a string.
    fn render(&self) -> Vec<char> {
        let square_count = self.rows * self.columns;
        if square_count == 0 {
            return vec![];
        }

        let start_idx = self.home.unwrap_or((square_count - 1) / 2);

        let moves = self.moves(&self.data, start_idx);
        let cur_idx = moves.last().unwrap();
        let counts = moves.iter().counts();

        (0..square_count)
            // Replace cell visit counts with symbols
            .map(|i| {
                if i == start_idx {
                    'S'
                } else if i == *cur_idx {
                    'E'
                } else {
                    let n = counts.get(&i).unwrap_or(&0);
                    let symbol_len = self.symbols.len();
                    if *n < symbol_len || self.cycle {
                        self.symbols[n % symbol_len]
                    } else {
                        *self.symbols.last().unwrap()
                    }
                }
            })
            .collect()
    }

    /// Convert (x, y) coordinates to a position in the grid.
    fn coordinates_to_position(&self, x: usize, y: usize) -> usize {
        y * self.columns + x
    }

    /// Convert a position in the grid to (x, y).
    fn position_to_coordinates(&self, v: usize) -> (usize, usize) {
        (v % self.columns, v / self.columns)
    }
}

impl fmt::Display for DrunkenBishop {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write strictly the first element into the supplied output
        // stream: `f`. Returns `fmt::Result` which indicates whether the
        // operation succeeded or failed. Note that `write!` uses syntax which
        // is very similar to `println!`.
        write!(f, "{}", frame(&self.render(), self.columns))
    }
}
