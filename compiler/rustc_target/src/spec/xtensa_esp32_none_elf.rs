use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "xtensa-none-elf".into(),
        pointer_width: 32,
        data_layout: "e-m:e-p:32:32-i64:32-i128:32-n32-f64:32".into(),
        arch: "xtensa".into(),
        
        options: TargetOptions {
            cpu: "esp32".into(),
            linker: Some("xtensa-esp32-elf-gcc".into()),
            max_atomic_width: Some(32),
            atomic_cas: true,
            ..super::xtensa_base::opts()
        },
    }
}