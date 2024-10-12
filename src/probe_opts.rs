/*
 *  Simple GUI for probe-rs with egui framework.
 *  Copyright (C) 2024 Joker2770
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

pub mod m_probe_opts {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ProbeOperations {
        FlashProgram,
        RTTIO,
    }

    impl Default for ProbeOperations {
        fn default() -> Self {
            Self::FlashProgram
        }
    }
}
