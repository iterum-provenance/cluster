use crate::pipelines::pipeline::PipelineJob;
use actix::prelude::*;
// use actix::Addr;
use k8s_openapi::api::batch::v1::{Job, JobStatus};
use kube::{api::Api, api::ListParams, Client};
use std::collections::HashMap;
use std::time::Duration;

pub struct KubeJobStatusMessage {
    pub status: bool,
    pub kube_statuses: HashMap<String, JobStatus>,
}

impl Message for KubeJobStatusMessage {
    type Result = bool;
}

pub struct PipelineActor {
    pub pipeline_job: PipelineJob,
    pub statuses: HashMap<String, JobStatus>,
    pub first_node_upstream_map: HashMap<String, String>,
}

impl PipelineActor {}

impl Actor for PipelineActor {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        info!("Pipeline actor is alive");

        ctx.run_interval(Duration::from_millis(5000), |act, context| {
            Arbiter::spawn(get_jobs_status(act.pipeline_job.clone(), context.address()));
        });
    }

    fn stopped(&mut self, _ctx: &mut Context<Self>) {
        info!("Pipeline actor is stopped");
    }
}

impl Handler<KubeJobStatusMessage> for PipelineActor {
    type Result = bool;

    fn handle(&mut self, msg: KubeJobStatusMessage, ctx: &mut Context<Self>) -> Self::Result {
        info!("Current pipeline status:");

        self.statuses = msg.kube_statuses;
        let success: Vec<bool> = self
            .statuses
            .iter()
            .map(|(key, val)| {
                info!("{}:\t{:?}", key, val.succeeded);
                val.succeeded.is_some()
            })
            .filter(|val| !val)
            .collect();
        if success.is_empty() {
            ctx.stop();
        }

        msg.status
    }
}

async fn get_jobs_status(pipeline_job: PipelineJob, actor_addr: Addr<PipelineActor>) {
    info!(
        "Checking status for pipeline with hash {}",
        pipeline_job.pipeline_hash
    );

    let mut statuses: HashMap<String, JobStatus> = HashMap::new();

    let client = Client::try_default().await.expect("create client");
    let jobs_client: Api<Job> = Api::namespaced(client.clone(), "default");
    // let pods_client: Api<Pod> = Api::namespaced(client.clone(), "default");

    let lp = ListParams::default().labels(&format!("pipeline_hash={}", pipeline_job.pipeline_hash));
    let jobs = jobs_client.list(&lp).await.unwrap();
    for job in jobs {
        let metadata = &job.metadata.as_ref().unwrap();
        let name = metadata.name.clone().unwrap();
        let status = job.status.clone().unwrap();
        statuses.insert(name, status);
    }
    actor_addr
        .send(KubeJobStatusMessage {
            status: true,
            kube_statuses: statuses,
        })
        .await
        .unwrap();
}

pub struct JobStatusMessage {
    pub step_name: String,
}

impl Message for JobStatusMessage {
    type Result = Option<bool>;
}

impl Handler<JobStatusMessage> for PipelineActor {
    type Result = Option<bool>;

    fn handle(&mut self, msg: JobStatusMessage, _ctx: &mut Context<Self>) -> Self::Result {
        info!("Retrieving {} from upstream list", msg.step_name);
        info!("Upstream list: {:?}", self.first_node_upstream_map);

        match self.first_node_upstream_map.get(&msg.step_name) {
            Some(step_upstream) => match self.statuses.get(step_upstream) {
                Some(status) => Some(status.succeeded.is_some()),
                None => None,
            },
            None => None,
        }
    }
}
