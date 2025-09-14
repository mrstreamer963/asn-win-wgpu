use asn_logger::{AsnLogConfig, AsnLogLevel, init_log};

pub fn setup_log() -> Result<(), String> {
    let c = AsnLogConfig {
        global_level: AsnLogLevel::Trace,
        module_levels: Default::default(),
    };

    init_log(&c)
}
