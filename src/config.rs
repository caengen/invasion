use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Debug(pub bool);
#[derive(Default)]
pub struct ProgramConfig {
    pub debug: bool,
}

impl ProgramConfig {
    pub fn build(args: &[String]) -> Result<ProgramConfig, &'static str> {
        let mut cfg = ProgramConfig::default();
        if args.len() == 0 {
            return Ok(cfg);
        }

        for arg in args {
            match arg.as_ref() {
                "-d" | "--debug" => {
                    cfg.debug = true;
                }
                _ => return Err("unknown argument"),
            }
        }

        Ok(cfg)
    }
}
