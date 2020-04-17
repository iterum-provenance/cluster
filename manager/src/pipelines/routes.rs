use crate::error::ManagerError;
use crate::pipelines::job::create_job_template;
use crate::pipelines::pipeline::PipelineJob;
use actix_web::{delete, get, post, web, HttpResponse, Responder};
use k8s_openapi::api::batch::v1::Job;
use kube::{
    api::Api,
    api::{ListParams, Meta, ObjectList, PostParams},
    Client,
};
use serde_json::json;

async fn get_pod_names() -> Result<Vec<String>, ManagerError> {
    use k8s_openapi::api::core::v1::Pod;

    let client = Client::try_default().await.expect("create client");
    let pods: Api<Pod> = Api::namespaced(client, "default");

    let lp = ListParams::default(); //.labels("app=blog");
    let pods = pods.list(&lp).await.unwrap();

    info!("There are pods.");
    let mut names: Vec<String> = vec![];
    for pod in pods {
        let name = pod.metadata.unwrap().name.unwrap().clone();
        info!("Pod: {:?}", name);
        names.push(name)
    }
    Ok(names)
}

async fn get_job_names() -> Result<Vec<String>, ManagerError> {
    let client = Client::try_default().await.expect("create client");
    let jobs: Api<Job> = Api::namespaced(client, "default");

    let lp = ListParams::default(); //.labels("app=blog");
    let jobs = jobs.list(&lp).await.unwrap();

    info!("There are pods.");
    let mut names: Vec<String> = vec![];
    for job in jobs {
        // job.
        let name = job.metadata.unwrap().name.unwrap();
        info!("Pod: {:?}", name);
        names.push(name)
    }
    Ok(names)
}

#[post("/submit_pipeline")]
async fn submit_pipeline(pipeline: web::Json<PipelineJob>) -> Result<HttpResponse, ManagerError> {
    info!("Submitting a pipeline");

    let pipeline = pipeline.into_inner();

    pipeline.submit().await?;

    Ok(HttpResponse::Ok().json(pipeline))
}

#[get("/get_pods")]
async fn get_pods() -> Result<HttpResponse, ManagerError> {
    info!("Getting pods");
    let pod_names = get_pod_names().await?;

    Ok(HttpResponse::Ok().json(pod_names))
}

#[get("/get_jobs")]
async fn get_jobs() -> Result<HttpResponse, ManagerError> {
    info!("Getting jobs");
    let job_names = get_job_names().await?;

    Ok(HttpResponse::Ok().json(job_names))
}

#[get("/get_pipelines")]
async fn get_pipelines() -> Result<HttpResponse, ManagerError> {
    info!("Getting pipelines");
    let pod_names = get_pod_names().await?;

    Ok(HttpResponse::Ok().json(pod_names))
}

#[delete("/delete_pipelines")]
async fn delete_pipelines() -> Result<HttpResponse, ManagerError> {
    info!("Deleting pipelines");
    PipelineJob::delete_all().await?;

    Ok(HttpResponse::Ok().finish())
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(get_pods);
    cfg.service(get_jobs);
    cfg.service(get_pipelines);
    cfg.service(submit_pipeline);
    cfg.service(delete_pipelines);
}
