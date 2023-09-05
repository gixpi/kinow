fn main(){
    tonic_build::configure()
    .build_client(false)
    .out_dir("proto/api")
    .compile(&["proto/protos/ticket.proto",],&["path"])
    .expect("failed to compile protos");
}