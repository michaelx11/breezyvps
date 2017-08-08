use super::command;

fn get_all_sshkey_ids() -> String {
    let get_sshkeys_cmd = "doctl compute ssh-key list --no-header --format=ID";
    println!("Running command:\n\t\t{}", get_sshkeys_cmd);
    let result = command::run_host_cmd(&get_sshkeys_cmd);
    if !result.success {
        println!("Failed to get sshkeys with stderr:\n\n{}", result.stderr);
    }
    let ids : Vec<&str> = result.stdout.lines().collect();
    return ids.join(",");
}

pub fn create_droplet_by_name(name: &str) {
    let create_str = format!("doctl compute droplet create {} --image=ubuntu-16-04-x64 --region=sfo1 --size=512mb --ssh-keys=\"{}\" --wait", name, get_all_sshkey_ids());
    println!("Running command:\n\t\t{}", create_str);
    // Create the actual droplet
    let result = command::run_host_cmd(&create_str);
    if !result.success {
        println!("Failed, with stderr:\n\n{}", result.stderr);
        return;
    }

    let list_command = "doctl compute droplet list --format Name,PublicIPv4,PublicIPv6,Status";
    println!("Running command:\n\t\t{}", list_command);
    let droplet_result = command::run_host_cmd(&list_command);
    if !droplet_result.success{
        println!("Failed, with stderr:\n\n{}", droplet_result.stderr);
        return;
    }
    let lines = droplet_result.stdout.lines();
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
        println!("Couldn't locate droplet in output:\n\n{}", droplet_result.stdout);
        return;
    }
    // Create a DNS record to point to the droplet
    let record_str = format!("doctl compute domain records create one.haus --record-type=A --record-data={} --record-name={}", ip_address, name);
    println!("Creating DNS record:\n\t\t{}", record_str);
    let create_record_result = command::run_host_cmd(&record_str);
    if !create_record_result.success{
        println!("Failed, with stderr:\n\n{}", create_record_result.stderr);
    }
}

pub fn destroy_droplet_by_name(name: &str) {
    let create_str = format!("doctl compute droplet delete -f {}", name);
    println!("Running command:\n\t\t{}", create_str);
    // Create the actual droplet
    let result = command::run_host_cmd(&create_str);
    if !result.success {
        println!("Failed with stderr:\n\n{}", result.stderr);
        return
    }
    let get_record_cmd = "doctl compute domain records list one.haus --format Name,ID --no-header";
    // Get A record and delete it!
    let record_result = command::run_host_cmd(&get_record_cmd);
    if !record_result.success {
        println!("Failed with stderr:\n\n{}", record_result.stderr);
        return
    }
    let lines = record_result.stdout.lines();
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
        println!("Couldn't locate record for [{}] in output:\n\n{}", name, record_result.stdout);
        return;
    }
    // Create a DNS record to point to the droplet
    let delete_record_cmd = format!("doctl compute domain records delete one.haus {}", record_to_delete);
    println!("Deleting DNS record:\n\t\t{}", delete_record_cmd);
    let delete_result = command::run_host_cmd(&delete_record_cmd);
    if !delete_result.success {
        println!("Failed with stderr:\n\n{}", delete_result.stderr);
        return
    }
}

pub fn create_sshkey(name: &str) {
    // By default, always attempt to add a new key with [name] mapping to ~/.ssh/id_rsa.pub
    let create_key_str = format!("doctl compute ssh-key create {} --public-key=\"$(cat ~/.ssh/id_rsa.pub)\"", name);
    println!("Running command:\n\t\t{}", create_key_str);
    // Create the actual droplet
    let result = command::run_host_cmd(&create_key_str);
    if !result.success {
        println!("Failed with stderr:\n\n{}", result.stderr);
    }
}
