use std::io;
use std::process::ExitCode;

fn codec_string() -> String {
    std::env::var("ENV_AVRO_CODEC")
        .ok()
        .unwrap_or_else(|| "null".into())
}

fn sub() -> Result<(), io::Error> {
    let codec: String = codec_string();
    rs_avro_transcode::enc::stdin2stdout(&codec)
}

fn main() -> ExitCode {
    sub().map(|_| ExitCode::SUCCESS).unwrap_or_else(|e| {
        eprintln!("{e}");
        ExitCode::FAILURE
    })
}
