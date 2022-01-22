use actix_web::rt::System;
use awc::Client;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// File to process
    #[structopt(name = "URI", parse(from_os_str))]
    uri: PathBuf,
}

fn main() {
    System::new("test").block_on(async {
        let client = Client::default();
        let opt = Opt::from_args();
        let uri = opt.uri.as_path().to_str().unwrap();
        let encoded_uri = urlencoding::encode(uri).to_string();
        let res = client
            .get("http://127.0.0.1:8080/load_file?image_url=".to_owned() + &encoded_uri)
            .header("User-Agent", "Actix-web")
            .send()
            .await;

        println!("Response: {:?}", res.unwrap().body().await);
    });
}
