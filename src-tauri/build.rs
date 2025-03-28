fn main() {
    tonic_build::configure().out_dir("./protos")
    .compile_protos(&["protos/chat.proto"], &[""])
    .unwrap_or_else(|e| eprintln!("Error here: {e:?}"));
    tauri_build::build()
}
