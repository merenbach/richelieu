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

mod step;

use core::cmp;
use derive_builder::Builder;
use itertools::Itertools;
use std::fmt;
use std::iter;
use step::steps_from_bytes;

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }

pub const STANDARD_COLUMNS: usize = 17;
pub const STANDARD_ROWS: usize = 9;
pub const STANDARD_STEPS: usize = 64;
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

/// Convert (x, y) coordinates to a position in a grid.
fn coordinates_to_position(columns: usize, x: usize, y: usize) -> usize {
    y * columns + x
}

/// Convert a grid position in to (x, y) coordinates.
fn position_to_coordinates(columns: usize, i: usize) -> (usize, usize) {
    (i % columns, i / columns)
}

#[derive(Builder, Default)]
#[builder(default)]
pub struct DrunkenBishop {
    /// Data to digest into steps for the bishop
    data: Vec<u8>,

    /// Rows on the board
    rows: usize,

    /// Columns on the board
    columns: usize,

    /// Steps for the bishop to take, or 0 for unlimited
    steps: usize,

    #[builder(setter(skip), default = "DEFAULT_SYMBOLS.chars().collect()")]
    symbols: Vec<char>,
    #[builder(setter(skip, strip_option))]
    home: Option<usize>, // starting location in the grid
    cycle: bool, // whether to recycle symbols or only go as far as the last
}

impl DrunkenBishop {
    /// Trace a route for the bishop based on byte data.
    fn trace(&self, start_idx: usize) -> Vec<usize> {
        steps_from_bytes(&self.data, self.steps)
            .iter()
            // Apply moves to start position to create numeric sequence of visited cell positions
            .scan(
                position_to_coordinates(self.columns, start_idx),
                |(x, y), m| {
                    let (a, b) = m.transform(*x, *y);
                    *x = cmp::min(self.columns - 1, a);
                    *y = cmp::min(self.rows - 1, b);
                    Some(coordinates_to_position(self.columns, *x, *y))
                },
            )
            .collect()
    }

    /// Render the grid into a string.
    fn render(&self) -> Vec<char> {
        let square_count = self.rows * self.columns;
        match square_count {
            0 => vec![],
            1 => vec!['S'],
            _ => {
                let start_idx = self.home.unwrap_or((square_count - 1) / 2);

                let moves = self.trace(start_idx);
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
        }
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
