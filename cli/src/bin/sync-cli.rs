use postman_collection_serializer::serialize;
use runtime::runner_sync;
use std::time::Instant;
use colored::Colorize;


fn main() {
    let args: Vec<String> = std::env::args().collect();
    let file_path = &args[1].to_string();
    let collection = serialize(file_path).unwrap();

    let mut responses = runner_sync(collection);

    for v in responses.iter_mut() {
        if v.response.is_ok() {
            let resp = v.response.as_mut().ok().take().unwrap();
            if resp.status().is_success() {
                let start = Instant::now();
                let status = resp.status().to_string();
                let url = resp.url().to_string();
                let time_taken = start.elapsed().as_millis();

                println!("{} ...{}", "Success".green().bold(), status);
                println!("URL: {url}");
                println!("  ...request completed in {}ms", v.time_taken);
                println!("  ...response serialization completed in {}ms\n", time_taken);
            } else {
                println!("{} ...{}", "Failed".red().bold(), resp.status().as_str());
                println!("URL: {}", resp.url().to_string());
                println!("  ...request completed in {}ms\n", v.time_taken);
            }
        }
    }
}
