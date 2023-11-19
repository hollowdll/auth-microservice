fn main() -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::configure()
        .build_server(false)
        .compile(
            &[
                "../AuthService/Proto/auth.proto",
                "../AuthService/Proto/user.proto"
            ],
            &["../AuthService/Proto"],
        )?;
   Ok(())
}