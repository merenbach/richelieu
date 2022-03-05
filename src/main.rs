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

use clap::Parser;
use drunken_bishop::{DrunkenBishopBuilder, STANDARD_COLUMNS, STANDARD_ROWS, STANDARD_STEPS};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Data to digest into steps for the bishop
    #[clap(short, long)]
    data: String,

    /// Columns on the board
    #[clap(short, long, default_value_t = STANDARD_COLUMNS)]
    columns: usize,

    /// Rows on the board
    #[clap(short, long, default_value_t = STANDARD_ROWS)]
    rows: usize,

    /// Steps for the bishop to take, or 0 for unlimited
    #[clap(short, long, default_value_t = STANDARD_STEPS)]
    steps: usize,
}

fn main() {
    let args = Args::parse();
    let x = DrunkenBishopBuilder::default()
        .data(args.data.bytes().collect())
        .columns(args.columns)
        .rows(args.rows)
        .steps(args.steps)
        .build()
        .unwrap();
    println!("{}", x);
}
