use std::collections::HashMap;
use std::path::Path;

use serde::{Deserialize, Serialize};
use spirv_builder::{Capability, MetadataPrintout, SpirvBuilder, SpirvMetadata};

#[derive(Serialize, Deserialize)]
pub struct CompileResult {
    entry_to_module: HashMap<String, String>,
}

fn main() {
    let builder_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let compile_results =
        SpirvBuilder::new(builder_root.join("../rust-shaders"), "spirv-unknown-spv1.5")
            // .extension("SPV_KHR_ray_query")
            .extension("SPV_KHR_ray_tracing")
            //.extension("SPV_KHR_physical_storage_buffer")
            // .capability(Capability::RayQueryKHR)
            .capability(Capability::RayTracingKHR)
            .capability(Capability::Int64)
            .capability(Capability::Int8)
            .capability(Capability::Int64Atomics)
            //.capability(Capability::PhysicalStorageBufferAddresses)
            //.capability(Capability::RuntimeDescriptorArray)
            .print_metadata(MetadataPrintout::None)
            .spirv_metadata(SpirvMetadata::Full)
            .preserve_bindings(true)
            .multimodule(true)
            .build()
            .unwrap();

    let target_spv_dir = builder_root.join("../../assets/spv");
    let _target_spv_dir = builder_root.join("../../assets/spv");
    std::fs::create_dir(&target_spv_dir);

    let result = match compile_results.module {
        spirv_builder::ModuleResult::MultiModule(entry_shader) => CompileResult {
            entry_to_module: entry_shader
                .iter()
                .map(|(entry, src_file)| {
                    let filename = src_file.file_name().unwrap();
                    let dst_file = _target_spv_dir.join(filename);

                    if src_file.exists() {
                        std::fs::rename(src_file, &dst_file).unwrap();
                    } else {
                        assert!(dst_file.exists());
                    }
                    (entry.clone(), filename.to_string_lossy().into())
                })
                .collect(),
        },
        _ => panic!(),
    };

    std::fs::write(
        target_spv_dir.join("shaders.json"),
        serde_json::to_string_pretty(&result).unwrap(),
    )
    .unwrap();
}
