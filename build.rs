fn main() {
    // tonic_build::compile_protos("protos/char.proto").unwrap();s
    tonic_build::configure()
    .out_dir("protos/chat.proto").compile_protos(&["protos/chat.proto"], &[""])
    .unwrap_or_else(|e| eprintln!("Error here: {e:?}"));

}