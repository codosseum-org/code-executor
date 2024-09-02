use sandkasten_client::{
    schemas::programs::{BuildRequest, BuildRunRequest, MainFile},
    SandkastenClient,
};

#[tokio::main]
async fn main() {
    let client = SandkastenClient::new("https://sandkasten.bootstrap.academy/".parse().unwrap());
    let result = client
        .build_and_run(&BuildRunRequest {
            build: BuildRequest {
                environment: "python".into(),
                main_file: MainFile {
                    name: Some("test.py".into()),
                    content: "print(6 * 7, end='')".into(),
                },
                ..Default::default()
            },
            run: Default::default(),
        })
        .await
        .unwrap();
    assert_eq!(result.run.stdout, "42");
}
