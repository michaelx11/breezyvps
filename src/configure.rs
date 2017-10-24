use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use super::command;
use super::chain;

pub fn install_nginx(host: &str) {
    let nginx_install_command = format!("ssh root@{} 'apt-get update && apt-get install -y nginx'", host);
    println!("Running:\n\t\t{}", nginx_install_command);
    let result = command::run_host_cmd(&nginx_install_command);
    if !result.success {
        println!("{}", result.stderr);
    }
}

fn create_host_file(host: &str, port: &str) -> String {
    let filepath = format!("/tmp/{}.conf", host);
    let path = Path::new(&filepath);
    let mut file = File::create(&path).expect("Failed to open file!");

    let template = r#"
server {
    listen 80;

    server_name {1};

    set_real_ip_from 127.0.0.1;
    set_real_ip_from 192.168.2.1;
    real_ip_header X-Forwarded-For;

    location / {
        proxy_pass http://localhost:{2};
        proxy_http_version 1.1;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_cache_bypass $http_upgrade;
    }
}"#;

    // These are hacks LOL, cause format! didn't seem to work with multiline string
    let mut contents = template.replace("{1}", host);
    contents = contents.replace("{2}", port);
    file.write_all(contents.as_bytes());
    return filepath.clone();
}

pub fn add_nginx_host(host: &str, port: &str) {
    let filename = create_host_file(host, port);
    // TODO: cleanup host file after scp
    let conf_create_command = format!("scp {} root@{}:/etc/nginx/conf.d/", filename, host);
    println!("Running:\n\t\t{}", conf_create_command);
    let result = command::run_host_cmd(&conf_create_command);
    if !result.success {
        println!("{}", result.stderr);
    }
}

pub fn install_letsencrypt_cert(host: &str) {
    let install_certbot_cmd = format!("ssh root@{} 'add-apt-repository ppa:certbot/certbot && apt-get update && apt-get install -y python-certbot-nginx'", host);
    println!("Running:\n\t\t{}", install_certbot_cmd);
    let result = command::run_host_cmd(&install_certbot_cmd);
    if !result.success {
        println!("{}", result.stderr);
    }
    let run_certbot_command = format!("ssh root@{} 'certbot --nginx -d {}'", host, host);
    println!("Please run:\n\t{}", run_certbot_command);
}

pub fn install_rust(host: &str) {
    let install_rust_cmd = format!("ssh root@{} 'curl https://sh.rustup.rs -sSf | sh -s -- -y'", host);
    let result = command::run_host_cmd(&install_rust_cmd);
    if !result.success {
        warn!("{}", result.stderr);
    } else {
        info!("{}", result.stdout);
    }
}

pub fn install_python(host: &str) {
    let install_python_cmd = format!("ssh root@{} 'apt-get update && apt-get install -y python'", host);
    let result = command::run_host_cmd(&install_python_cmd);
    if !result.success {
        warn!("{}", result.stderr);
    } else {
        info!("{}", result.stdout);
    }
}

pub fn install_jekyll(host: &str) {
    let _ = chain::CommandChain::new()
        .cmd(&format!("ssh root@{} 'apt-get update && apt-get install -y rubygems build-essential ruby-dev'", host))
        .cmd(&format!("ssh root@{} 'gem install jekyll bundler'", host))
        .execute();
}

pub fn renew_cert(host: &str) {
    let _ = chain::CommandChain::new()
        .cmd(&format!("ssh root@{} 'certbot --nginx renew'", host))
        .execute();
}

pub fn setup_iptables(host: &str) {
    let _ = chain::CommandChain::new()
        .cmd(&format!("ssh root@{} 'iptables -P INPUT ACCEPT'", host)) // First, switch input back to accept
        .cmd(&format!("ssh root@{} 'iptables -F'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -p tcp --tcp-flags ALL NONE -j DROP'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -p tcp ! --syn -m state --state NEW -j DROP'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -p tcp --tcp-flags ALL ALL -j DROP'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -s 127.0.0.1 -j ACCEPT'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -p tcp -m tcp --dport 80 -j ACCEPT'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -p tcp -m tcp --dport 443 -j ACCEPT'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -p tcp -m tcp --dport 22 -j ACCEPT'", host))
        .cmd(&format!("ssh root@{} 'iptables -A INPUT -m state --state ESTABLISHED,RELATED -j ACCEPT'", host))
        .cmd(&format!("ssh root@{} 'iptables -P OUTPUT ACCEPT'", host))
        .cmd(&format!("ssh root@{} 'iptables -P INPUT DROP'", host))
        .execute();
}

pub fn install_sqlite3(host: &str) {
    let _ = chain::CommandChain::new()
        .cmd(&format!("ssh root@{} 'apt-get update'", host))
        .cmd(&format!("ssh root@{} 'apt-get install -y sqlite3 libsqlite3-dev'", host))
        .execute();
}
