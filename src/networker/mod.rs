use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct NetOpt {
    #[structopt(short, long, help = "URL to mount to", default_value = "/stream")]
    url: String,

    #[structopt(short, long, help = "Port to bind to", default_value = "1181")]
    port: u16,
}

impl NetOpt {
    pub fn get_url(&self) -> &String {
        &self.url
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }
}
