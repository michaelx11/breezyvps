use std::process::Command;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn install_nginx(host: &str) {
    let nginx_install_command = format!("ssh root@{} 'apt-get update && apt-get install -y nginx'", host);
    println!("Running:\n\t\t{}", nginx_install_command);
    let result = Command::new("sh")
                         .arg("-c")
                         .arg(nginx_install_command)
                         .output()
                         .expect("Failed to install nginx");
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
    let result = Command::new("sh")
                         .arg("-c")
                         .arg(conf_create_command)
                         .output()
                         .expect("Failed to copy over conf!");
}

pub fn install_letsencrypt_cert(host: &str) {
    let install_certbot_cmd = format!("ssh root@{} 'add-apt-repository ppa:certbot/certbot && apt-get update && apt-get install -y python-certbot-nginx'", host);
    println!("Running:\n\t\t{}", install_certbot_cmd);
    let install_result = Command::new("sh")
                         .arg("-c")
                         .arg(install_certbot_cmd)
                         .output()
                         .expect("Failed to install certbot");
    let run_certbot_command = format!("ssh root@{} 'certbot --nginx -d {}'", host, host);
    println!("Please run:\n\t{}", run_certbot_command);
//    let certbot_result = Command::new("sh")
//                         .arg("-c")
//                         .arg(run_certbot_command)
//                         .output()
//                         .expect("Failed to run certbot!");
}
