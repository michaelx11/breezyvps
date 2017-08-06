#[macro_use]
extern crate clap;
use std::process::Command;

fn sc_do(do_matches: &clap::ArgMatches) {
    let mut verbose = false;
    if do_matches.is_present("verbose") {
	verbose = true;
    }
    if let Some(create_droplet_matches) = do_matches.subcommand_matches("create_droplet") {
	if let Some(name) = create_droplet_matches.value_of("name") {
            let config = create_droplet_matches.value_of("config").unwrap_or("small");
            println!("Creating a new droplet [{}] with config: {}", name, config);
            let create_str = format!("doctl compute droplet create {} --image=ubuntu-16-04-x64 --region=sfo1 --size=512mb --wait", name);
            println!("Running command:\n\t\t{}", create_str);
            // Create the actual droplet
            Command::new("sh")
                    .arg("-c")
                    .arg(create_str)
                    .output()
                    .expect("doctl create failed!");
            // Get the IP address!
            let droplet_list_output = Command::new("sh")
                                              .arg("-c")
                                              .arg("doctl compute droplet list --format Name,PublicIPv4,PublicIPv6,Status")
                                              .output()
                                              .expect("doctl create failed!");
            let output_raw = droplet_list_output.stdout;
            let output_str = String::from_utf8(output_raw).expect("Found invalid UTF-8 in droplet list output");
            let lines = output_str.lines();
            let mut ip_address = "";
            for line in lines {
                if line.starts_with(name) {
                    println!("Found: {}", line);
                    let fields : Vec<&str> = line.split_whitespace().collect();
                    if fields.len() < 2 {
                        println!("Couldn't find ip address in line: {}", line);
                        return;
                    }
                    ip_address = fields[1];
                    break;
                }
            }
            if ip_address.len() == 0 {
            	println!("Couldn't locate droplet in output:\n\n{}", output_str);
                return;
            }
            // Create a DNS record to point to the droplet
            let record_str = format!("doctl compute domain records create one.haus --record-type=A --record-data={} --record-name={}", ip_address, name);
            println!("Creating DNS record:\n\t\t{}", record_str);
            Command::new("sh")
                    .arg("-c")
                    .arg(record_str)
                    .output()
                    .expect("doctl create record failed failed!");
	    // Configure the droplet
	}
    }
}

fn main() {
    let matches = clap_app!(myapp =>
        (version: "0.0.1")
        (author: "Michael Xu <michaeljxu11@gmail.com>")
        (about: "One stop shop for common command line goodness")
        (@arg verbose: -v ... "Enable verbose output")
        (@subcommand do =>
            (about: "Doctl wrapper")
            (version: "0.0.0")
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
        )
    ).get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("do") {
	sc_do(matches);
    }
}
