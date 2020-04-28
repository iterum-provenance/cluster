use crate::error::ManagerError;
use k8s_openapi::api::batch::v1::Job;
use serde_json::json;

pub fn create_combiner_template(
    name: &str,
    image: &str,
    input_channel: &str,
    dataset_name: &str,
    pipeline_hash: &str,
    daemon_url: &str,
) -> Result<Job, ManagerError> {
    let job: Job = serde_json::from_value(json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": { "name": name },
        "spec": {
            "completions": 1,
            "parallelism": 1,
            "template": {
                "metadata": {
                    "name": name
                },
                "spec": {
                    "containers": [{
                        "name": "combiner",
                        "image": "localhost:32000/combiner:1",
                        "env": [
                            {"name": "BROKER_URL", "value": "amqp://iterum:sinaasappel@iterum-mq-rabbitmq-ha:5672"},
                            {"name": "INPUT_QUEUE", "value": input_channel},
                            {"name": "MINIO_URL", "value": "iterum-minio:9000"},
                            {"name": "MINIO_ACCESS_KEY", "value": "iterum"},
                            {"name": "MINIO_SECRET_KEY", "value": "banaanappel"},
                            {"name": "PIPELINE_HASH", "value": pipeline_hash},
                            {"name": "DAEMON_URL", "value": daemon_url},
                            {"name": "DATASET_NAME", "value": dataset_name},

                        ]
                    }],
                    "restartPolicy": "OnFailure"
                }
            }
        }
    }))?;
    Ok(job)
}
