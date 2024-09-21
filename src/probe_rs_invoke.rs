pub mod probe_rs_integration {
    use probe_rs::{
        config, flashing,
        probe::{list, DebugProbeInfo},
        Permissions, Session,rtt::Rtt,
    };
    use std::error::Error;
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    #[derive(Default)]
    pub struct ProbeRsHandler {
        pub chips_list: Vec<String>,
    }

    impl ProbeRsHandler {
        pub fn get_probes_list() -> Vec<DebugProbeInfo> {
            // Get a list of all available debug probes.
            let lister = list::Lister::new();
            let probes = lister.list_all();
            probes
        }

        pub fn attach(
            debug_probe_info: &DebugProbeInfo,
            target_chip: &str,
        ) -> Result<Session, Box<dyn Error>> {
            let p = debug_probe_info.open()?;
            let s = p.attach(target_chip, Permissions::default())?;
            Ok(s)
        }

        pub fn try_to_download(
            session: &mut Session,
            file_path: &PathBuf,
            file_format: flashing::Format,
        ) -> Result<(), Box<dyn Error>> {
            probe_rs::flashing::download_file(session, file_path, file_format)?;
            for c in session.list_cores() {
                let mut c_u = session.core(c.0)?;
                c_u.reset()?;
            }
            Ok(())
        }

        pub fn get_availabe_chips(&mut self) -> &Vec<String> {
            if 0 >= self.chips_list.len() {
                for family in config::families() {
                    for variant in family.variants() {
                        let v = variant.name.clone();
                        self.chips_list.push(v);
                    }
                }
            }
            &self.chips_list
        }

        pub fn rtt_read_from_channel(
            session: &mut Session,
            buf: &mut [u8],
            core_idx: usize,
            channel_number: usize,
        ) -> Result<usize, Box<dyn Error>> {
            let memory_map = session.target().memory_map.clone();
            // Select a core.
            let mut core = session.core(core_idx)?;

            // Attach to RTT
            let mut rtt = Rtt::attach(&mut core, &memory_map)?;

            // Read from a channel
            let mut count = 0;
            if let Some(input) = rtt.up_channels().take(channel_number) {
                count = input.read(&mut core, &mut buf[..])?;
                // println!("Read data: {:?}", &buf[..count]);
            }
            Ok(count)
        }
    }
}
