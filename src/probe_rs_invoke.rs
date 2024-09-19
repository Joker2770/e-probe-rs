pub mod probe_rs_integration {
    use probe_rs::{
        config, flashing,
        probe::{list, DebugProbeInfo},
        Permissions,
    };
    use std::error::Error;
    use std::path::PathBuf;

    pub fn get_probes_list() -> Vec<DebugProbeInfo> {
        // Get a list of all available debug probes.
        let lister = list::Lister::new();
        let probes = lister.list_all();
        probes
    }

    pub fn try_to_download(
        debug_probe_info: &DebugProbeInfo,
        target_chip: &str,
        file_path: PathBuf,
        file_format: flashing::Format,
    ) -> Result<(), Box<dyn Error>> {
        let p = debug_probe_info.open()?;
        let mut s = p.attach(target_chip, Permissions::default())?;
        probe_rs::flashing::download_file(&mut s, file_path, file_format)?;
        Ok(())
    }

    pub fn get_availabe_chips() -> Vec<String> {
        let mut vec = Vec::new();
        for family in config::families() {
            for variant in family.variants() {
                let v = variant.name.clone();
                vec.push(v);
            }
        }
        vec
    }
}
