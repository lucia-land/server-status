use kube::{Client, api::{Api, ListParams}};
use k8s_openapi::api::core::v1::Pod;

pub async fn collect_pod_status() -> anyhow::Result<Vec<String>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::all(client);
    let list = pods.list(&ListParams::default()).await?; // ✅ pods로 수정 (오타)
    let mut result = vec![];

    for pod in list.items {
        let name = pod.metadata.name.unwrap_or_default();
        let phase = pod.status.and_then(|s| s.phase).unwrap_or_default();
        result.push(format!("{name}: {phase}"));
    }

    Ok(result) // ✅ 세미콜론 제거 (return 문 아님)
}
