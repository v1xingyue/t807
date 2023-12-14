use {log::info, os_info};

pub fn description() {
    let info = os_info::get();
    info!("os type : {}", info.os_type());
    info!("os version  : {}", info.version());
    info!("os edition  : {:?}", info.edition());
    info!("os codename : {:?}", info.codename());
    info!("os bitness  : {}", info.bitness());
    info!("os architecture : {}", info.architecture().unwrap());
}
