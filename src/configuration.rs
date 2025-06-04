/*
 *  Simple GUI for probe-rs with egui framework.
 *  Copyright (C) 2025 Joker2770
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

pub mod m_config {
    pub const APP_NAME: &str = "e-probe-rs";
    pub const WIN_WIDTH: f32 = 960.0;
    pub const WIN_HEIGHT: f32 = 720.0;
    pub const PAGE_1_LABEL: &str = "Flash";
    pub const PAGE_2_LABEL: &str = "RTT";
    pub const RTT_SYMBOL: &str = "_SEGGER_RTT";
}
