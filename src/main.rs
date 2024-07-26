pub fn main() {
    let result = seajob_api::start();

    if let Some(err) = result.err() {
        print!("Error: {err}");
    }
}