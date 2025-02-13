/*
Sentinel by Alyx Shang.
Licensed under the FSL v1.
*/

/// Importing the 
/// runner function.
use sentinel::run_app;

/// The main point
/// of entry for
/// the Rust compiler
/// and Actix Web.
#[actix_web::main]
async fn main(){
    match run_app().await{
        Ok(_feedback) => {},
        Err(e) => eprintln!("{}", e.to_string())
    };
}