use super::command;
use super::chain;

pub fn create_droplet_by_name(name: &str, region: Option<&str>, size: Option<&str>) {

    let ssh_key_mapping_func = |res: &command::Result, cmd_str: String| -> String {
        let ids : Vec<&str> = res.stdout.lines().collect();
        let new_cmd = str::replace(&cmd_str, "%ssh_keys%", &ids.join(","));
        new_cmd.to_string()
    };

    let ip_address_mapping_func = |res: &command::Result, cmd_str: String| -> String {
        let mut ip_address : Option<String> = None;
        let res_stdout = res.stdout.clone();
        for line in res_stdout.lines() {
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
            return "--will fail--".to_string()
        }

        let new_cmd = str::replace(&cmd_str, "%ip_address%", &ip_address.unwrap());
        new_cmd.to_string()
    };

    let create_str = format!("doctl compute droplet create {} --image=ubuntu-16-04-x64 --region={} --size={} --ssh-keys=\"%ssh_keys%\" --wait",
                             name,
                             region.unwrap_or("sfo1"),
                             size.unwrap_or("512mb"));
    let record_str = format!("doctl compute domain records create one.haus --record-type=A --record-data=%ip_address% --record-name={}", name);

    // TODO: check the result heh
    let _ = chain::CommandChain::new()
        .cmd("doctl compute ssh-key list --no-header --format=ID")
        .result_mapped_cmd(&ssh_key_mapping_func, &create_str)
        .cmd("doctl compute droplet list --format Name,PublicIPv4,PublicIPv6,Status")
        .result_mapped_cmd(&ip_address_mapping_func, &record_str)
        .execute();
}

pub fn destroy_droplet_by_name(name: &str) {
    let record_id_extractor = |res: &command::Result, cmd_str: String| -> String {
        let mut record_id : Option<String> = None;
        let res_stdout = res.stdout.clone();
        for line in res_stdout.lines() {
            if line.starts_with(name) {
                debug!("Found: {}", line);
                let fields : Vec<&str> = line.split_whitespace().collect();
                if fields.len() < 2 {
                    warn!("Couldn't find ip address in line: {}", line);
                }
                record_id = Some(fields[1].to_string());
                break;
            }
        }
        if record_id == None {
            error!("Couldn't locate droplet in output: {}", res_stdout);
            return "--will fail--".to_string()
        }

        let new_cmd = str::replace(&cmd_str, "%record_id%", &record_id.unwrap());
        new_cmd.to_string()
    };

    let delete_droplet_cmd = format!("doctl compute droplet delete -f {}", name);
    let delete_record_cmd = format!("doctl compute domain records delete -f one.haus %record_id%");

    // TODO: check the result heh
    let _ = chain::CommandChain::new()
        .cmd(&delete_droplet_cmd)
        .cmd("doctl compute domain records list one.haus --format Name,ID --no-header")
        .result_mapped_cmd(&record_id_extractor, &delete_record_cmd)
        .execute();
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
