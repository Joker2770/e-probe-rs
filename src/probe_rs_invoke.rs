pub mod probe_rs_integration {
    use probe_rs::{
        config, flashing,
        probe::{list, DebugProbeInfo},
        rtt::{Rtt, ScanRegion, UpChannel},
        Core, Permissions, Session,
    };
    use std::borrow::BorrowMut;
    use std::path::PathBuf;
    use std::{borrow::Borrow, error::Error};

    #[derive(Default)]
    pub struct ProbeRsHandler {
        pub probes_list: Vec<DebugProbeInfo>,
        pub chips_list: Vec<String>,
        pub up_chs_size: usize,
        pub cur_ch: Option<UpChannel>,
        pub session: Option<Session>,
        pub rtt: Option<Rtt>,
        pub target_cores_num: usize,
        pub scan_region: ScanRegion,
    }

    impl ProbeRsHandler {
        pub fn get_probes_list(&mut self) -> &Vec<DebugProbeInfo> {
            // Get a list of all available debug probes.
            let lister = list::Lister::new();
            self.probes_list = lister.list_all();
            &self.probes_list
        }

        pub fn attach_target(
            &mut self,
            probe_idx: usize,
            target_chip: &str,
        ) -> Result<&Option<Session>, Box<dyn Error>> {
            if let None = self.session {
                if probe_idx < self.probes_list.len() {
                    let p = self.probes_list[probe_idx].open()?;
                    let s = p.attach(target_chip, Permissions::default())?;
                    self.session = Some(s);
                }
            }
            Ok(&self.session)
        }

        pub fn attach_target_under_reset(
            &mut self,
            probe_idx: usize,
            target_chip: &str,
        ) -> Result<&Option<Session>, Box<dyn Error>> {
            if let None = self.session {
                if probe_idx < self.probes_list.len() {
                    let p = self.probes_list[probe_idx].open()?;
                    let s = p.attach_under_reset(target_chip, Permissions::default())?;
                    self.session = Some(s);
                }
            }
            Ok(&self.session)
        }

        pub fn try_to_download(
            &mut self,
            file_path: &PathBuf,
            file_format: flashing::Format,
        ) -> Result<(), Box<dyn Error>> {
            if let Some(s) = self.session.borrow_mut() {
                probe_rs::flashing::download_file(s, file_path, file_format)?;
            }
            Ok(())
        }

        pub fn reset_all_cores(&mut self) -> Result<(), Box<dyn Error>> {
            if let Some(s) = self.session.borrow_mut() {
                for c in s.list_cores() {
                    let mut c_u = s.core(c.0)?;
                    c_u.reset()?;
                }
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

        pub fn get_core_num(&mut self) -> usize {
            if let Some(s) = self.session.borrow() {
                self.target_cores_num = s.list_cores().len();
            }
            self.target_cores_num
        }

        pub fn get_core(&mut self, core_idx: usize) -> Result<Option<Core>, Box<dyn Error>> {
            let mut opt_core = None;
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    // Select a core.
                    let core = s.core(core_idx)?;
                    opt_core = Some(core);
                }
            }

            Ok(opt_core)
        }

        pub fn attach_rtt(&mut self, core_idx: usize) -> Result<&Option<Rtt>, Box<dyn Error>> {
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    let memory_map = s.target().memory_map.clone();
                    // Select a core.
                    let mut core = s.core(core_idx)?;

                    // Attach to RTT
                    let mut rtt = Rtt::attach(&mut core, &memory_map)?;
                    self.up_chs_size = rtt.up_channels().len();
                    self.rtt = Some(rtt);
                }
            }
            Ok(&self.rtt)
        }

        pub fn attach_rtt_region(
            &mut self,
            core_idx: usize,
        ) -> Result<&Option<Rtt>, Box<dyn Error>> {
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    let memory_map = s.target().memory_map.clone();
                    // Select a core.
                    let mut core = s.core(core_idx)?;

                    // Attach to RTT
                    let mut rtt = Rtt::attach_region(&mut core, &memory_map, &self.scan_region)?;
                    self.up_chs_size = rtt.up_channels().len();
                    self.rtt = Some(rtt);
                }
            }
            Ok(&self.rtt)
        }

        pub fn get_up_channels_size(&mut self) -> usize {
            if let Some(r) = self.rtt.borrow_mut() {
                self.up_chs_size = r.up_channels().len();
            }
            self.up_chs_size
        }

        pub fn get_one_up_ch(&mut self, ch_number: usize) -> &Option<UpChannel> {
            if let Some(r) = self.rtt.borrow_mut() {
                if ch_number < self.up_chs_size {
                    let up_chs = r.up_channels();
                    self.cur_ch = up_chs.take(ch_number);
                }
            }
            &self.cur_ch
        }

        pub fn rtt_read_from_channel(
            &mut self,
            buf: &mut [u8],
            core_idx: usize,
        ) -> Result<usize, Box<dyn Error>> {
            let mut count = 0;
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    // Select a core.
                    let mut core = s.core(core_idx)?;
                    // Read from a channel
                    if let Some(up_ch) = &self.cur_ch {
                        count = up_ch.read(&mut core, &mut buf[..])?;
                    }
                    // println!("Read data: {:?}", &buf[..count]);
                }
            }
            Ok(count)
        }
    }
}
