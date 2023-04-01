// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::fs;
use std::path::PathBuf;
use std::fs::File;
use std::env;
use std::io::Write;

fn main() {
    let uuid = match fs::read_to_string("../uuid.txt") {
        Ok(u) => {
            u.trim().to_string()
        },
        Err(_) => {
            panic!("Cannot find uuid.txt");
        }
    };
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let mut buffer = File::create(out.join("uuid.txt")).unwrap();
    write!(buffer, "{}", uuid).unwrap();
/*
    let bip39_en_words = match fs::read_to_string("../bip39_en_words.txt") {
        Ok(u) => {
            u.trim().to_string()
        },
        Err(_) => {
            panic!("Cannot find bip39_en_words.txt");
        }
    };
    let mut buffer_2 = File::create(out.join("bip39_en_words.txt")).unwrap();
    write!(buffer_2, "{}", bip39_en_words).unwrap();
    */
}
