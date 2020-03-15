use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Config {
    helloworld_url: String,
}

// Host information structure returned at /hostinfo
#[derive(Deserialize, Debug)]
struct HostInfo {
    hostname: String,
    pid: u32,
    uptime: u64,
}

// Name your user agent after your app
pub static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
    " (+https://github.com/Xe/helloworld)",
);

fn main() -> reqwest::Result<()> {
    let cfg: Config;

    match envy::from_env::<Config>() {
        Ok(my_cfg) => cfg = my_cfg,
        Err(why) => panic!("{:?}", why),
    }

    println!("{:?}", cfg);

    let client = reqwest::blocking::Client::builder()
        .user_agent(APP_USER_AGENT)
        .build()?;

    let msg = client.get(&cfg.helloworld_url).send()?.text()?;
    println!("{}", msg);

    let hostinfo: HostInfo = client
        .get(&format!("{}/{}", &cfg.helloworld_url, "hostinfo"))
        .send()?
        .json()?;

    println!("{:?}", hostinfo);

    Ok(())
}
