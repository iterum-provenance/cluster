use crate::error::ManagerError;
use crate::pipelines::pipeline::PipelineJob;
use k8s_openapi::api::batch::v1::Job;
use serde_json::json;
use std::env;

pub fn create_combiner_template(pipeline_job: &PipelineJob) -> Result<Job, ManagerError> {
    let hash = format!("{}-combiner", &pipeline_job.pipeline_hash);

    let job: Job = serde_json::from_value(json!({
        "apiVersion": "batch/v1",
        "kind": "Job",
        "metadata": { "name": hash, "labels": {"pipeline_hash": pipeline_job.pipeline_hash} },
        "spec": {
            "parallelism": 1,
            "template": {
                "metadata": {
                    "name": hash
                },
                "spec": {
                    "containers": [{
                        "name": "combiner",
                        "image": env::var("COMBINER_IMAGE").unwrap(),
                        "env": [
                            {"name": "DATA_VOLUME_PATH", "value": "/data-volume"},
                            {"name": "ITERUM_NAME", "value": &hash},
                            {"name": "PIPELINE_HASH", "value": &pipeline_job.pipeline_hash},

                            {"name": "DAEMON_URL", "value": env::var("DAEMON_URL").unwrap()},
                            {"name": "DAEMON_DATASET", "value": &pipeline_job.input_dataset},
                            {"name": "DAEMON_COMMIT_HASH", "value": &pipeline_job.input_dataset_commit_hash},


                            {"name": "MANAGER_URL", "value": env::var("MANAGER_URL").unwrap()},


                            {"name": "MINIO_URL", "value": env::var("MINIO_URL").unwrap()},
                            {"name": "MINIO_ACCESS_KEY", "value": env::var("MINIO_ACCESS_KEY").unwrap()},
                            {"name": "MINIO_SECRET_KEY", "value": env::var("MINIO_SECRET_KEY").unwrap()},
                            {"name": "MINIO_USE_SSL", "value": env::var("MINIO_USE_SSL").unwrap()},
                            {"name": "MINIO_OUTPUT_BUCKET", "value": "INVALID"},

                            {"name": "MQ_BROKER_URL", "value": env::var("MQ_BROKER_URL").unwrap()},
                            {"name": "MQ_OUTPUT_QUEUE", "value": "INVALID"},
                            {"name": "MQ_INPUT_QUEUE", "value": &pipeline_job.combiner_input_channel},

                            {"name": "TRANSFORMATION_STEP_INPUT", "value": "tts.sock"},
                            {"name": "TRANSFORMATION_STEP_OUTPUT", "value": "fts.sock"},

                        ]
                    }],
                    "restartPolicy": "OnFailure"
                }
            }
        }
    }))?;
    Ok(job)
}