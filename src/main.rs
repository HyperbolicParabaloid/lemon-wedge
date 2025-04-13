use lemon_wedge;

fn main() {
    match lemon_wedge::run() {
        Ok(_) => {},
        Err(err) => {
            eprintln!("Trying initalizing Window for program: \n{}", err)
        }
    }
}
