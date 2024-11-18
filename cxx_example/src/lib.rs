#[cxx::bridge(namespace = "cxx_api")]

mod ffi {

    extern "Rust" {
        /// @fn add
        /// @brief Returns left + right
        /// @param (left) unsigned integer value
        /// @param (right) unsigned integer value
        /// @version 0.1.0
        fn add(left: u64, right: u64) -> u64;
    }
}

fn add(left: u64, right: u64) -> u64 {
    rs_example::add(left, right)
}
