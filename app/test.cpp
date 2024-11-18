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

#include <cstring>
#include <iostream>
#include <string>

#include "cxx_example/src/lib.rs.h"
#include "rust/cxx.h"

int main(int argc, char *argv[]) {
  auto result = cxx_api::add(1, 2);
  std::cout << result << std::endl;
  return 0;
}
