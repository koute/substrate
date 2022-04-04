// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! This crate contains the code necessary to gather basic hardware
//! and software telemetry information about the node on which we're running.

use futures::prelude::*;

mod sysinfo;
#[cfg(target_os = "linux")]
mod sysinfo_linux;

pub use sysinfo::{gather_hwbench, gather_sysinfo};

/// Prints out the system software/hardware information in the logs.
pub fn print_sysinfo(sysinfo: &sc_telemetry::SysInfo) {
	if let Some(ref cpu) = sysinfo.cpu {
		log::info!("💻 CPU: {}", cpu);
	}
	if let Some(core_count) = sysinfo.core_count {
		log::info!("💻 CPU cores: {}", core_count);
	}
	if let Some(memory) = sysinfo.memory {
		log::info!("💻 Memory: {}MB", memory / (1024 * 1024));
	}
	if let Some(ref linux_kernel) = sysinfo.linux_kernel {
		log::info!("💻 Kernel: {}", linux_kernel);
	}
	if let Some(ref linux_distro) = sysinfo.linux_distro {
		log::info!("💻 Linux distribution: {}", linux_distro);
	}
	if let Some(is_virtual_machine) = sysinfo.is_virtual_machine {
		log::info!("💻 Virtual machine: {}", if is_virtual_machine { "yes" } else { "no" });
	}
}

/// Prints out the results of the hardware benchmarks in the logs.
pub fn print_hwbench(hwbench: &sc_telemetry::HwBench) {
	log::info!("🏁 CPU score: {}MB/s", hwbench.cpu_hashrate_score);
	log::info!("🏁 Memory score: {}MB/s", hwbench.memory_memcpy_score);

	if let Some(score) = hwbench.disk_sequential_write_score {
		log::info!("🏁 Disk score (seq. writes): {}MB/s", score);
	}
	if let Some(score) = hwbench.disk_random_write_score {
		log::info!("🏁 Disk score (rand. writes): {}MB/s", score);
	}
}

/// Initializes the hardware benchmarks telemetry.
pub fn initialize_hwbench_telemetry(
	telemetry_handle: sc_telemetry::TelemetryHandle,
	hwbench: sc_telemetry::HwBench,
) -> impl std::future::Future<Output = ()> {
	let mut connect_stream = telemetry_handle.on_connect_stream();
	async move {
		let payload = sc_telemetry::serde_json::to_value(&hwbench)
			.expect("the `HwBench` can always be serialized into a JSON object; qed");
		let mut payload = match payload {
			sc_telemetry::serde_json::Value::Object(map) => map,
			_ => unreachable!("the `HwBench` always serializes into a JSON object; qed"),
		};
		payload.insert("msg".into(), "sysinfo.hwbench".into());
		while let Some(_) = connect_stream.next().await {
			telemetry_handle.send_telemetry(sc_telemetry::SUBSTRATE_INFO, payload.clone());
		}
	}
}
