use crate::error::ManagerError;
use k8s_openapi::api::batch::v1::Job;
use serde_json::json;

pub fn create_job_template(
    name: &str,
    image: &str,
    input_channel: &str,
    output_channel: &str,
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
                    "volumes": [
                        {"name": "data-volume", "emptyDir": {}}
                    ],
                    "containers": [{
                        "name": "sidecar",
                        "image": "sidecar:1",
                        "env": [
                            {"name": "BROKER_URL", "value": "amqp://iterum:sinaasappel@iterum-mq-rabbitmq-ha:5672"},
                            {"name": "INPUT_QUEUE", "value": input_channel},
                            {"name": "OUTPUT_QUEUE", "value": output_channel},
                            {"name": "MINIO_URL", "value": "iterum-minio:9000"},
                            {"name": "MINIO_ACCESS_KEY", "value": "iterum"},
                            {"name": "MINIO_SECRET_KEY", "value": "banaanappel"},
                            {"name": "MINIO_OUTPUT_BUCKET", "value": "outputbucket"},
                            {"name": "DATA_VOLUME_PATH", "value": "/data-volume"},
                        ],
                        "volumeMounts": [{
                            "name": "data-volume",
                            "mountPath": "/data-volume"
                        }]
                    },
                    {
                        "name": "transformation-step",
                        "image": image,
                        "env": [
                            {"name": "DATA_VOLUME_PATH", "value": "/data-volume"},
                        ],
                        "volumeMounts": [{
                            "name": "data-volume",
                            "mountPath": "/data-volume"
                        }]
                    }],
                    "restartPolicy": "OnFailure"
                }
            }
        }
    }))?;
    Ok(job)
}
