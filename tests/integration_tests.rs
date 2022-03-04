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

use drunken_bishop::DrunkenBishopBuilder;
use itertools::Itertools;
use serde::Deserialize;
use std_ext::read_data_from_file;

#[test]
fn test_digest() {
    #[derive(Deserialize)]
    struct TestCase {
        rows: usize,
        cols: usize,
        limit: usize,
        cycle: bool,
        input: String,
        output: Vec<String>,
    }

    let tables: Vec<TestCase> = read_data_from_file("testdata/digest.json").unwrap();

    for t in tables {
        let output: String = t.output.iter().join("\n");
        let out = DrunkenBishopBuilder::default()
            .columns(t.cols)
            .rows(t.rows)
            .data(t.input.bytes().collect())
            .limit(t.limit)
            .cycle(t.cycle)
            .build()
            .unwrap()
            .render();
        assert_eq!(out, output);
    }
}
