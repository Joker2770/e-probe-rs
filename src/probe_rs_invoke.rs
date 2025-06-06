/*
 *  Simple GUI for probe-rs with egui framework.
 *  Copyright (C) 2024-2025 Joker2770
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod probe_rs_integration {
    use crate::configuration::m_config;
    use probe_rs::{
        config, flashing,
        probe::{list, DebugProbeInfo},
        rtt::{Rtt, ScanRegion},
        Core, Permissions, Session,
    };
    use std::{
        borrow::{Borrow, BorrowMut},
        error::Error,
        fs, io,
        path::PathBuf,
        time::{Duration, Instant},
    };

    fn get_rtt_symbol<T: io::Read + io::Seek>(file: &mut T) -> Option<u64> {
        get_symbol(file, m_config::RTT_SYMBOL)
    }

    fn get_symbol<T: io::Read + io::Seek>(file: &mut T, symbol: &str) -> Option<u64> {
        let mut buffer = Vec::new();
        if file.read_to_end(&mut buffer).is_ok() {
            if let Ok(binary) = goblin::elf::Elf::parse(buffer.as_slice()) {
                for sym in &binary.syms {
                    if let Some(name) = binary.strtab.get_at(sym.st_name) {
                        if name == symbol {
                            return Some(sym.st_value);
                        }
                    }
                }
            }
        }
        None
    }

    #[derive(Default)]
    pub struct ProbeRsHandler {
        pub probes_list: Vec<DebugProbeInfo>,
        pub chips_list: Vec<String>,
        pub up_chs_size: usize,
        pub session: Option<Session>,
        pub rtt: Option<Rtt>,
        pub target_cores_num: usize,
        pub scan_region: Option<ScanRegion>,
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
            if self.session.is_none() && probe_idx < self.probes_list.len() {
                let p = self.probes_list[probe_idx].open()?;
                let s = p.attach(target_chip, Permissions::default())?;
                self.session = Some(s);
            }
            Ok(&self.session)
        }

        pub fn attach_target_under_reset(
            &mut self,
            probe_idx: usize,
            target_chip: &str,
        ) -> Result<&Option<Session>, Box<dyn Error>> {
            if self.session.is_none() && probe_idx < self.probes_list.len() {
                let p = self.probes_list[probe_idx].open()?;
                let s = p.attach_under_reset(target_chip, Permissions::default())?;
                self.session = Some(s);
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
            if self.chips_list.is_empty() {
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

        #[allow(dead_code)]
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
                    // Select a core.
                    let mut core = s.core(core_idx)?;

                    // Attach to RTT
                    let mut rtt = Rtt::attach(&mut core)?;
                    self.up_chs_size = rtt.up_channels().len();
                    self.rtt = Some(rtt);
                }
            }
            Ok(&self.rtt)
        }

        pub fn get_scan_region(
            &mut self,
            elf_file: &Option<PathBuf>,
            control_block_address: Option<u64>,
        ) -> Result<&Option<ScanRegion>, Box<dyn Error>> {
            let mut scan_region = ScanRegion::Ram;
            if let Some(user_provided_addr) = control_block_address {
                scan_region = ScanRegion::Exact(user_provided_addr);
            } else if let Some(elf_file) = elf_file.as_ref() {
                let mut file = fs::File::open(elf_file).expect("open elf file");
                if let Some(rtt_addr) = get_rtt_symbol(&mut file) {
                    scan_region = ScanRegion::Exact(rtt_addr as _);
                }
            }

            self.scan_region = Some(scan_region);

            Ok(&self.scan_region)
        }

        pub fn attach_rtt_region(
            &mut self,
            core_idx: usize,
        ) -> Result<&Option<Rtt>, Box<dyn Error>> {
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    // Select a core.
                    let mut core = s.core(core_idx)?;

                    // Attach to RTT
                    if let Some(scan_region) = self.scan_region.borrow() {
                        let mut rtt = Rtt::attach_region(&mut core, scan_region)?;
                        self.up_chs_size = rtt.up_channels().len();
                        self.rtt = Some(rtt);
                    }
                }
            }
            Ok(&self.rtt)
        }

        pub fn attach_retry_loop(
            &mut self,
            core_idx: usize,
            timeout: Duration,
        ) -> Result<&Option<Rtt>, Box<dyn Error>> {
            let timeout: Duration = timeout;
            let start = Instant::now();
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    // Select a core.
                    let mut core = s.core(core_idx)?;
                    loop {
                        if let Some(scan_region) = self.scan_region.borrow() {
                            match Rtt::attach_region(&mut core, scan_region) {
                                Ok(rtt) => {
                                    self.rtt = Some(rtt);
                                    return Ok(&self.rtt);
                                }
                                Err(e) => {
                                    if matches!(e, probe_rs::rtt::Error::ControlBlockNotFound) {
                                        std::thread::sleep(Duration::from_millis(50));
                                        continue;
                                    }

                                    self.rtt = None;
                                    return Ok(&self.rtt);
                                }
                            }
                        }
                        if Instant::now().duration_since(start) <= timeout {
                            continue;
                        } else {
                            break;
                        }
                    }
                    // Timeout reached
                    self.rtt = Some(Rtt::attach(&mut core).expect("RTT attach"));
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

        pub fn rtt_read_from_channel(
            &mut self,
            buf: &mut [u8],
            core_idx: usize,
            ch_number: usize,
        ) -> Result<usize, Box<dyn Error>> {
            let mut count = 0;
            if let Some(s) = self.session.borrow_mut() {
                if core_idx < self.target_cores_num {
                    // Select a core.
                    let mut core = s.core(core_idx)?;
                    // Read from a channel
                    if let Some(r) = self.rtt.borrow_mut() {
                        if let Some(up_ch) = r.up_channel(ch_number) {
                            count = up_ch.read(&mut core, &mut buf[..])?;
                        }
                    }
                    // println!("Read data: {:?}", &buf[..count]);
                }
            }
            Ok(count)
        }
    }
}
