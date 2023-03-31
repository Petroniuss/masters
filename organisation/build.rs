fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "gen-proto")]
    {
        println!("Generating protobuf files");
        tonic_build::configure()
            .out_dir("src/transport/grpc/")
            .compile(&["proto/command.proto"], &["proto/"])?;
    }

    Ok(())
}
