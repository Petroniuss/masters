fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "gen-proto")]
    {
        tonic_build::configure()
            .out_dir("src/transport/grpc/")
            .compile(&["proto/command.proto"], &["proto/"])?;

        print!("tonic build!");
    }

    Ok(())
}
