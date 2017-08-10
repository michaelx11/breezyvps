use super::command;
use super::chain;

pub fn create_droplet_by_name(name: &str) {

    let ssh_key_mapping_func = |res: &command::Result, cmd_str: String| -> String {
        let ids : Vec<&str> = res.stdout.lines().collect();
        let new_cmd = str::replace(&cmd_str, "%ssh_keys%", &ids.join(","));
        new_cmd.to_string()
    };

    let ip_address_mapping_func = |res: &command::Result, cmd_str: String| -> String {
        let new_cmd = str::replace(&cmd_str, "%ip_address%", &res.stdout);
        new_cmd.to_string()
    };

    // Get ip address from stdout
    let ip_address_extractor = |res: &command::Result| -> command::Result {
        let mut ip_address : Option<String> = None;
        let res_stdout = res.stdout.clone();
        let lines = res_stdout.lines();
        for line in lines {
            if line.starts_with(name) {
                debug!("Found: {}", line);
                let fields : Vec<&str> = line.split_whitespace().collect();
                if fields.len() < 2 {
                    warn!("Couldn't find ip address in line: {}", line);
                }
                ip_address = Some(fields[1].to_string());
                break;
            }
        }
        if ip_address == None {
            error!("Couldn't locate droplet in output: {}", res_stdout);
            return command::Result {
                exit_code: None,
                success: false,
                stdout: "".to_string(),
                stderr: res.stderr.clone()
            }
        }

        command::Result {
            exit_code: res.exit_code,
            success: res.success,
            // We check for None above
            stdout: ip_address.unwrap().trim().to_string(),
            stderr: res.stderr.clone()
        }
    };

    let create_str = format!("doctl compute droplet create {} --image=ubuntu-16-04-x64 --region=sfo1 --size=512mb --ssh-keys=\"%ssh_keys%\" --wait", name);
    let record_str = format!("doctl compute domain records create one.haus --record-type=A --record-data=%ip_address% --record-name={}", name);

    let chain = chain::CommandChain::new()
        .cmd("doctl compute ssh-key list --no-header --format=ID")
        .result_mapped_cmd(&ssh_key_mapping_func, &create_str)
        .cmd("doctl compute droplet list --format Name,PublicIPv4,PublicIPv6,Status")
        .result_proc(&ip_address_extractor)
        .result_mapped_cmd(&ip_address_mapping_func, &record_str)
        .execute();
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
