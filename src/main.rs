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
use drunken_bishop::{DrunkenBishopBuilder, DEFAULT_COLUMNS, DEFAULT_ROWS};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Data to encode
    #[clap(short, long)]
    data: String,

    /// Columns on the board
    #[clap(short, long, default_value_t = DEFAULT_COLUMNS)]
    columns: usize,

    /// Rows on the board
    #[clap(short, long, default_value_t = DEFAULT_ROWS)]
    rows: usize,
}

fn main() {
    let args = Args::parse();
    let x = DrunkenBishopBuilder::default()
        .data(args.data.bytes().collect())
        .columns(args.columns)
        .rows(args.rows)
        .build()
        .unwrap();
    println!("{}", x);
}
