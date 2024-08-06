use tonic_build::configure;

fn main() {
    configure()
        .compile(
            &[
                "protos/auth.proto",
                "protos/block.proto",
                "protos/block_engine.proto",
                "protos/bundle.proto",
                "protos/packet.proto",
                "protos/relayer.proto",
                "protos/searcher.proto",
                "protos/shared.proto",
            ],
            &["protos"],
        )
    tonic_build::configure()
        .compile_with_config(
            prost_build::Config::new().protoc_arg("--experimental_allow_proto3_optional"),
            &proto_files,
            &proto_includes,
        )
        .unwrap();
}
