use clap::ArgMatches;
use yowsl::{DistroFlags, Wslapi};

#[allow(non_camel_case_types)]
type WSL_DISTRIBUTION_FLAGS = u32;

pub fn run(wslapi: &Wslapi, matches: &ArgMatches) {
    let name = matches.value_of("NAME").unwrap();
    match wslapi.is_distribution_registered(name) {
        Ok(true) => {}
        Ok(false) => {
            eprintln!("\"{}\" is not a registered WSL distro name", name);
            return;
        }
        Err(e) => {
            eprintln!("I cannot set a configuration of \"{}\"\nError: {}", name, e);
            return;
        }
    }
    let mut distro_configuration = match wslapi.get_distro_configuration(name) {
        Ok(distro_configuration) => distro_configuration,
        Err(e) => {
            eprintln!("I cannot set a configuration of \"{}\"\nError: {}", name, e);
            return;
        }
    };
    if matches.is_present("default_uid") {
        distro_configuration.default_uid =
            matches.value_of("default_uid").unwrap().parse().unwrap();
    }
    if matches.is_present("flags") {
        distro_configuration.flags = DistroFlags::from_bits(
            WSL_DISTRIBUTION_FLAGS::from_str_radix(matches.value_of("flags").unwrap(), 2).unwrap(),
        ).unwrap();
    }
    if let Err(e) = wslapi.configure_distro(&distro_configuration) {
        eprintln!("I cannot set a configuration of \"{}\"\nError: {}", name, e);
    }
}
