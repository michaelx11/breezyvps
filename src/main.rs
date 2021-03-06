#[macro_use]
extern crate clap;
extern crate breezyvps;
extern crate simplelog;

use simplelog::{Config, TermLogger, WriteLogger, CombinedLogger, LogLevelFilter};
use std::fs::File;

fn sc_doctl(doctl_matches: &clap::ArgMatches) {
    if let Some(create_droplet_matches) = doctl_matches.subcommand_matches("create_droplet") {
        if let Some(name) = create_droplet_matches.value_of("name") {
            breezyvps::digitalocean::create_droplet_by_name(
                name,
                // Both are unwrapped safely with defaults [sfo1, 512mb]
                create_droplet_matches.value_of("region"),
                create_droplet_matches.value_of("size"),
                create_droplet_matches.value_of("domain"),
                create_droplet_matches.value_of("backups"));
        } else {
            println!("Missing required name parameter!");
        }
        return;
    }
    if let Some(destroy_droplet_matches) = doctl_matches.subcommand_matches("destroy_droplet") {
        if let Some(name) = destroy_droplet_matches.value_of("name") {
            breezyvps::digitalocean::destroy_droplet_by_name(name,
                destroy_droplet_matches.value_of("domain"));
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
    if let Some(rust_matches) = configure_matches.subcommand_matches("rust") {
        if let Some(host) = rust_matches.value_of("host") {
            breezyvps::configure::install_rust(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
    if let Some(python_matches) = configure_matches.subcommand_matches("python") {
        if let Some(host) = python_matches.value_of("host") {
            breezyvps::configure::install_python(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
    if let Some(jekyll_matches) = configure_matches.subcommand_matches("jekyll") {
        if let Some(host) = jekyll_matches.value_of("host") {
            breezyvps::configure::install_jekyll(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
    if let Some(renew_matches) = configure_matches.subcommand_matches("renew") {
        if let Some(host) = renew_matches.value_of("host") {
            breezyvps::configure::renew_cert(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
    if let Some(iptables_matches) = configure_matches.subcommand_matches("setup_iptables") {
        if let Some(host) = iptables_matches.value_of("host") {
            breezyvps::configure::setup_iptables(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
    if let Some(sqlite_matches) = configure_matches.subcommand_matches("sqlite3") {
        if let Some(host) = sqlite_matches.value_of("host") {
            breezyvps::configure::install_sqlite3(host);
        } else {
            println!("Missing required host parameter!");
        }
        return;
    }
    if let Some(nodejs_matches) = configure_matches.subcommand_matches("nodejs") {
        if let Some(host) = nodejs_matches.value_of("host") {
            breezyvps::configure::install_nodejs(host);
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
            WriteLogger::new(LogLevelFilter::Info, Config::default(), File::create("/tmp/breezyvps.log").unwrap()),
        ]
    ).unwrap();

    let matches = clap_app!(breezyvps =>
        (version: "0.2.1")
        (author: "Michael Xu <michaeljxu11@gmail.com>")
        (about: "One stop shop for common command line goodness")
        (@arg verbose: -v ... "Enable verbose output")
        (@subcommand doctl =>
            (about: "Doctl wrapper")
            (author: "Michael Xu <michaeljxu11@gmail.com>")
            (@arg verbose: -v --verbose "Print test information verbosely")
            (@subcommand create_droplet =>
                (about: "Create a new droplet and domain record.")
                (@arg name: +required "Full DNS name of the droplet, must be unique. e.g. cloud.one.haus, one.haus")
                (@arg region: -r --region +takes_value "Which region? [sfo1, nyc1, etc..]")
                (@arg size: -s --size +takes_value "Which size droplet? [512mb, 1gb, 2gb, 4gb, 8gb, 16gb, 32gb, 48gb, 64gb]")
                (@arg domain: -d --domain +takes_value "Which domain name? [best.haus,log.haus,swarm.link,swarmlink.com,util.in]")
                (@arg backups: -b --backups +takes_value "[y/n (default)] Should backups be enabled ($1/month)")
            )
            (@subcommand destroy_droplet =>
                (about: "Destroy a droplet by name")
                (@arg name: +required "Name of the droplet to destroy completely")
                (@arg domain: -d --domain +takes_value "Domain name, default one.haus")
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
            (@subcommand rust =>
                (about: "Install rust on an ubuntu host")
                (@arg host: +required "Host name of the droplet")
            )
            (@subcommand python =>
                (about: "Install python2.7 on an ubuntu host")
                (@arg host: +required "Host name of the droplet")
            )
            (@subcommand jekyll =>
                (about: "Install jekyll on an ubuntu host")
                (@arg host: +required "Host name of the droplet")
            )
            (@subcommand renew =>
                (about: "Renew letsencrypt cert on ubuntu host")
                (@arg host: +required "Host name of the droplet")
            )
            (@subcommand setup_iptables =>
                (about: "Setup iptables to only allow 80,443,22")
                (@arg host: +required "Host name of the droplet")
            )
            (@subcommand sqlite3 =>
                (about: "Install sqlite3 on Ubuntu")
                (@arg host: +required "Host name of the droplet")
            )
            (@subcommand nodejs =>
                (about: "Install nodejs on Ubuntu")
                (@arg host: +required "Host name of the droplet")
            )
        )
    ).get_matches();

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
}
