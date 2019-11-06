// read the content to std_out.
fn main() {
    let mut args = std::env::args();
    let _ = args.next();
    let filename = args.next().expect("need file name");
    let content = std::fs::read_to_string(format!("/proc/{}", filename)).expect("read error");
    println!("{:#?}", content);
}
