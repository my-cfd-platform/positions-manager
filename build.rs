fn main() {
    tonic_build::compile_protos("proto/positions_manager_grpc_service.proto").unwrap();
}
