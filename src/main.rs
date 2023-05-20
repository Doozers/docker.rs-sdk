use std::os::unix::net::UnixStream;
use std::io::prelude::*;
use std::error::Error;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct DockerInfo {
    #[serde(rename = "Platform")]
    platform: Platform,
    #[serde(rename = "Components")]
    components: Vec<Component>,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "ApiVersion")]
    api_version: String,
    #[serde(rename = "MinAPIVersion")]
    min_api_version: String,
    #[serde(rename = "GitCommit")]
    git_commit: String,
    #[serde(rename = "GoVersion")]
    go_version: String,
    #[serde(rename = "Os")]
    os: String,
    #[serde(rename = "Arch")]
    arch: String,
    #[serde(rename = "KernelVersion")]
    kernel_version: String,
    #[serde(rename = "BuildTime")]
    build_time: String,
}

#[derive(Deserialize, Debug)]
pub struct Component {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Details")]
    details: Details,
}

#[derive(Deserialize, Debug)]
pub struct Details {
    #[serde(rename = "ApiVersion")]
    api_version: Option<String>,
    #[serde(rename = "Arch")]
    arch: Option<String>,
    #[serde(rename = "BuildTime")]
    build_time: Option<String>,
    #[serde(rename = "Experimental")]
    experimental: Option<String>,
    #[serde(rename = "GitCommit")]
    git_commit: String,
    #[serde(rename = "GoVersion")]
    go_version: Option<String>,
    #[serde(rename = "KernelVersion")]
    kernel_version: Option<String>,
    #[serde(rename = "MinAPIVersion")]
    min_api_version: Option<String>,
    #[serde(rename = "Os")]
    os: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Platform {
    #[serde(rename = "Name")]
    name: String,
}


fn main() -> Result<(), Box<dyn Error>> {
    let mut stream = UnixStream::connect("/var/run/docker.sock")?;

    let request = b"GET /version HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\n\r\n";
    stream.write_all(request)?;

    let mut response = Vec::new();
    let mut buffer = [0; 4096]; // Buffer size for reading response


    let bytes_read = stream.read(&mut buffer)?;

    response.extend_from_slice(&buffer[..bytes_read]);
    let res_str = String::from_utf8_lossy(&response);
    if let Some(index) = res_str.find("\r\n\r\n") {
        let json_data = &response[index + 4..];
        // print!("{}", std::str::from_utf8(json_data)?);
        let docker_info: DockerInfo = serde_json::from_str(std::str::from_utf8(json_data)?).unwrap();
        println!("{:?}", docker_info);
    }

    Ok(())
}
