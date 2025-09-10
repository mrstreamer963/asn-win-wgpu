use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

pub fn setup_log() -> Result<(), String> {
    let mut c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    c.module_levels
        .insert(String::from("wgpu_core"), AsnLogLevel::Error);
    c.module_levels
        .insert(String::from("wgpu_hal"), AsnLogLevel::Trace);
    c.module_levels
        .insert(String::from("naga"), AsnLogLevel::Error);
    c.module_levels
        .insert(String::from("asn-win-wgpu"), AsnLogLevel::Trace);

    c.module_levels
        .insert(String::from("asn-wgpu"), AsnLogLevel::Trace);

    c.module_levels
        .insert(String::from("wgpu_map"), AsnLogLevel::Trace);

    init_log(&c)
}
