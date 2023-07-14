// Copyright 2023 Helsing GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
extern crate cc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // This produces an static library of lsm1.c that we can use to produce
    // our Rust bindings.
    cc::Build::new()
        .file("src/lsm1/lsm1-ae2e7fc.c")
        .define("LSM_MUTEX_PTHREADS", "1")
        // .define("LSM_LOG_WORK", "1")
        .static_flag(true)
        .warnings(false)
        .opt_level(2)
        .debug(true)
        .compile("liblsm1.a");

    Ok(())
}
