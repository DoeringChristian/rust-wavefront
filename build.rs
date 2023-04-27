use std::path::Path;
use std::process::Command;

use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder, SpirvMetadata};

fn main() {
    Command::new("cargo")
        .arg("run")
        .arg("-p builder")
        .status()
        .unwrap();
}
