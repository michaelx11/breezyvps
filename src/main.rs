#[macro_use]
extern crate clap;
extern crate breezyvps;
extern crate simplelog;

use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;
use breezyvps::command;

fn sc_doctl(doctl_matches: &clap::ArgMatches) {
    if let Some(create_droplet_matches) = doctl_matches.subcommand_matches("create_droplet") {
        if let Some(name) = create_droplet_matches.value_of("name") {
            breezyvps::digitalocean::create_droplet_by_name(name);
        } else {
            println!("Missing required name parameter!");
        }
        return;
    }
    if let Some(destroy_droplet_matches) = doctl_matches.subcommand_matches("destroy_droplet") {
        if let Some(name) = destroy_droplet_matches.value_of("name") {
            breezyvps::digitalocean::destroy_droplet_by_name(name);
        } else {
            println!("Missing required name parameter!");
        }
        return;
    }
    if let Some(create_ssh_key_matches) = doctl_matches.subcommand_matches("create_sshkey") {
        if let Some(name) = create_ssh_key_matches.value_of("name") {
            breezyvps::digitalocean::create_sshkey(name);
        } else {
            println!("Missing required sshkey name parameter!");
        }
        return;
    }
}

fn sc_configure(configure_matches: &clap::ArgMatches) {
    if let Some(nginx_matches) = configure_matches.subcommand_matches("nginx") {
        if let Some(host) = nginx_matches.value_of("host") {
            // Default 8080
            let port = nginx_matches.value_of("port").unwrap_or("8080");
            breezyvps::configure::install_nginx(host);
            breezyvps::configure::add_nginx_host(host, port);
            breezyvps::configure::install_letsencrypt_cert(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
}

fn main() {
    // Configure logging with simplelogger
    CombinedLogger::init(
        vec![
            TermLogger::new(LogLevelFilter::Info, Config::default()).unwrap(),
            WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("breezyvps.log").unwrap()),
        ]
    ).unwrap();

    let matches = clap_app!(myapp =>
        (version: "0.1.3")
        (author: "Michael Xu <michaeljxu11@gmail.com>")
        (about: "One stop shop for common command line goodness")
        (@arg verbose: -v ... "Enable verbose output")
        (@subcommand doctl =>
            (about: "Doctl wrapper")
            (author: "Michael Xu <michaeljxu11@gmail.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
            (@subcommand create_droplet =>
                (about: "Create a new droplet")
                (@arg name: +required "Name of the droplet, must be unique")
                (@arg config: -c "Which configuration to use [small|medium|large]")
            )
            (@subcommand destroy_droplet =>
                (about: "Destroy a droplet by name")
                (@arg name: +required "Name of the droplet to destroy completely")
            )
            (@subcommand create_sshkey =>
                (about: "Add an ssh key, which will be added upon instance creation")
                (@arg name: +required "Name of the new ssh keys")
            )
        )
        (@subcommand configure =>
            (about: "Configure droplets / nodes")
            (@subcommand nginx =>
                (about: "Install nginx and configure for host")
                (@arg host: +required "Host name of the droplet")
                (@arg port: "Port to run webapp from (default: 8080)")
            )
        )
    ).get_matches();

//    // Vary the output based on how many times the user used the "verbose" flag
//    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//    match matches.occurrences_of("v") {
//        0 => println!("No verbose info"),
//        1 => println!("Some verbose info"),
//        2 => println!("Tons of verbose info"),
//        3 | _ => println!("Don't be crazy"),
//    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("doctl") {
        sc_doctl(matches);
        return;
    }
    if let Some(matches) = matches.subcommand_matches("configure") {
        sc_configure(matches);
        return;
    }

    let mut x = breezyvps::chain::CommandChain::new();
    x.cmd("echo hello")
        .cmd("test2")
        .cmd("test3")
        .execute();

    breezyvps::chain::CommandChain::new()
        .cmd_nonfatal("echo hello")
        .cmd_nonfatal("restinpeace")
        .cmd("echo yo")
        .execute();


    let res = breezyvps::chain::CommandChain::new()
        .cmd("echo hello")
        .result_proc(&|res: &command::Result| -> command::Result {
            let mut extra_stdout = res.stdout.clone();
            extra_stdout.push_str(" + processing");

            command::Result {
                exit_code: res.exit_code,
                success: res.success,
                stdout: extra_stdout,
                stderr: res.stderr.clone()
            }
        })
        .execute();

    println!("{}", res.result.unwrap().stdout);
}
