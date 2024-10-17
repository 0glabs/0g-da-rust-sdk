fn main() {
    tonic_build::compile_protos("proto/disperser.proto").unwrap();
}
