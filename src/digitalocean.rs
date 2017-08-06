use std::process::Command;

fn get_all_sshkey_ids() -> String {
    let sshkey_ids = Command::new("sh")
                                      .arg("-c")
                                      .arg("doctl compute ssh-key list --no-header --format=ID")
                                      .output()
                                      .expect("Failed to list all ssh keys!");
    let output_raw = sshkey_ids.stdout;
    let output_str = String::from_utf8(output_raw).expect("Found invalid UTF-8 in sshkey ids");
    let ids : Vec<&str> = output_str.lines().collect();
    return ids.join(",");
}

pub fn create_droplet_by_name(name: &str) {
    let create_str = format!("doctl compute droplet create {} --image=ubuntu-16-04-x64 --region=sfo1 --size=512mb --ssh-keys=\"{}\" --wait", name, get_all_sshkey_ids());
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
                                      .expect("doctl list droplets failed!");
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
            .expect("doctl create record failed!");
}

pub fn destroy_droplet_by_name(name: &str) {
    let create_str = format!("doctl compute droplet delete -f {}", name);
    println!("Running command:\n\t\t{}", create_str);
    // Create the actual droplet
    Command::new("sh")
            .arg("-c")
            .arg(create_str)
            .output()
            .expect("doctl delete failed!");
    // Get A record and delete it!
    let droplet_list_output = Command::new("sh")
                                      .arg("-c")
                                      .arg("doctl compute domain records list one.haus --format Name,ID --no-header")
                                      .output()
                                      .expect("doctl list failed!");
    let output_raw = droplet_list_output.stdout;
    let output_str = String::from_utf8(output_raw).expect("Found invalid UTF-8 in droplet list output");
    let lines = output_str.lines();
    let mut record_to_delete = "";
    for line in lines {
        let fields : Vec<&str> = line.split_whitespace().collect();
        if fields.len() < 2 {
            println!("Invalid output line: {}", line);
            continue;
        }
        if fields[0].starts_with(name) {
            println!("Found: {}", line);
            record_to_delete = fields[1];
            break;
        }
    }
    if record_to_delete.len() == 0 {
        println!("Couldn't locate record for [{}] in output:\n\n{}", name, output_str);
        return;
    }
    // Create a DNS record to point to the droplet
    let record_str = format!("doctl compute domain records delete one.haus {}", record_to_delete);
    println!("Deleting DNS record:\n\t\t{}", record_str);
    Command::new("sh")
            .arg("-c")
            .arg(record_str)
            .output()
            .expect("doctl delete record failed!");
}

pub fn create_sshkey(name: &str) {
    // By default, always attempt to add a new key with [name] mapping to ~/.ssh/id_rsa.pub
    let create_key_str = format!("doctl compute ssh-key create {} --public-key=\"$(cat ~/.ssh/id_rsa.pub)\"", name);
    println!("Running command:\n\t\t{}", create_key_str);
    // Create the actual droplet
    Command::new("sh")
            .arg("-c")
            .arg(create_key_str)
            .output()
            .expect("Failed to create new key, it may already exist");
}
