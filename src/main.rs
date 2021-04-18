extern crate clap;
extern crate sys_info;
use clap::App;

fn main() {
    let rice = App::new("Rice")
        .version(clap::crate_version!());

    println!("{} v{}", rice.get_name(), clap::crate_version!());

    if let Ok(os) = sys_info::os_type() {
        println!("OS {}", os);
    }

    if let Ok(release) = sys_info::os_release() {
        println!("Release {}", release);
    }

    if let Ok(meminfo) = sys_info::mem_info() {
        println!("Meminfo {}", meminfo.total);
    }

    if let Ok(num) = sys_info::cpu_num() {
        println!("CPU Num {}", num)
    }
}
