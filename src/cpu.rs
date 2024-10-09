use sysinfo::{CpuRefreshKind, RefreshKind, System};

pub struct Cpu {
    pub name: String,
    pub usage: f32,
}

pub fn get_cpus() -> Vec<Cpu> {
    let mut cpus: Vec<Cpu> = vec![];

    let mut sys =
        System::new_with_specifics(RefreshKind::new().with_cpu(CpuRefreshKind::everything()));

    // Wait a bit because CPU usage is based on diff.
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    // Refresh CPUs again to get actual value.
    sys.refresh_cpu_all();

    for syscpu in sys.cpus() {
        let cpu = Cpu {
            name: syscpu.name().to_string(),
            usage: syscpu.cpu_usage(),
        };
        cpus.push(cpu);
    }

    cpus
}

pub fn get_cpu_percentage(cpus: &Vec<Cpu>) -> u8 {
    let sum_of_cpus_usage: u16 = cpus.iter().map(|cpu| cpu.usage as u16).sum();
    let cpus_number = cpus.len() as u16;

    (sum_of_cpus_usage / cpus_number) as u8
}
