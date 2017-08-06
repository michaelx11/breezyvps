#[macro_use]
extern crate clap;
extern crate breezyvps;

fn sc_do(do_matches: &clap::ArgMatches) {
    let mut verbose = false;
    if do_matches.is_present("verbose") {
        verbose = true;
    }
    if let Some(create_droplet_matches) = do_matches.subcommand_matches("create_droplet") {
        if let Some(name) = create_droplet_matches.value_of("name") {
            breezyvps::digitalocean::create_droplet_by_name(name);
        } else {
            println!("Missing required name parameter!");
        }
        return;
    }
    if let Some(destroy_droplet_matches) = do_matches.subcommand_matches("destroy_droplet") {
        if let Some(name) = destroy_droplet_matches.value_of("name") {
            breezyvps::digitalocean::destroy_droplet_by_name(name);
        } else {
            println!("Missing required name parameter!");
        }
        return;
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
