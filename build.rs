use std::path::PathBuf;

fn main() {
    let proto = std::fs::read_dir("proto")
        .and_then(|x| x.collect::<Result<Vec<_>, _>>())
        .map(|x| {
            x.into_iter()
                .filter(|x| x.file_type().unwrap().is_file())
                .filter(|x| x.file_name().to_str().unwrap().ends_with(".proto"))
                .map(|x| x.file_name().into_string().unwrap())
                .collect::<Vec<_>>()
        })
        .unwrap_or(vec![]);
    tonic_build::configure()
        .build_client(true)
        .build_server(true)
        .build_transport(true)
        .out_dir(PathBuf::from("libs/rpc"))
        .compile_protos(&proto, &["proto"])
        .unwrap();
}
