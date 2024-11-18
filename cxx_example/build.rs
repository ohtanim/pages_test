//
// (C) Copyright IBM 2024
//
// This code is licensed under the Apache License, Version 2.0. You may
// obtain a copy of this license in the LICENSE.txt file in the root directory
// of this source tree or at http://www.apache.org/licenses/LICENSE-2.0.
//
// Any modifications or derivative works of this code must retain this
// copyright notice, and modified files need to carry a notice indicating
// that they have been altered from the originals.

use cxx_build::CFG;

fn main() {
    // Enables Doxygen-style comments in the generated C++ header.
    CFG.doxygen = true;

    // This will consider the ffi part in lib.rs in order to
    // generate lib.rs.h and lib.rs.cc
    // minimal example: no C++ code to be called from Rust
    cxx_build::bridge("src/lib.rs")
        .flag_if_supported("-std=c++17")
        .compile("cxx_example");

    println!("cargo:rerun-if-changed=src/lib.rs");
}
