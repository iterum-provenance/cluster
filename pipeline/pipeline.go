package pipeline

import (
	"encoding/json"
	"fmt"
	"net/http"

	batchv1 "k8s.io/api/batch/v1"
	apiv1 "k8s.io/api/core/v1"
	v1 "k8s.io/api/core/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	v11 "k8s.io/client-go/kubernetes/typed/batch/v1"
)

type PipelineJob struct {
	Name                string               `json:"name"`
	TransformationSteps []TransformationStep `json:"transformation_steps"`
}

func Create(jobsClient v11.JobInterface) func(w http.ResponseWriter, r *http.Request) {

	return func(w http.ResponseWriter, r *http.Request) {
		var p PipelineJob

		fmt.Println("Received request!")
		// Try to decode the request body into the struct. If there is an error,
		// respond to the client with the error message and a 400 status code.
		err := json.NewDecoder(r.Body).Decode(&p)
		if err != nil {
			http.Error(w, err.Error(), http.StatusBadRequest)
			return
		}
		fmt.Println("Created pipeline:", p)

		for _, step := range p.TransformationSteps {
			fmt.Println("Step:", step.Name)
			inputChannel := p.Name + "-" + step.Name + "_input"
			outputChannel := p.Name + "-" + step.Name + "_output"

			job := createTransformationStep(p.Name+"-"+step.Name, step.Image, inputChannel, outputChannel)
			result, err := jobsClient.Create(&job)
			if err != nil {
				fmt.Println(err)
				panic(err)
			}
			fmt.Printf("Created job %q.\n", result.GetName())

		}

		// Do something with the Person struct...
		fmt.Fprintf(w, "Pipeline: %+v", p)

	}
}

func Get(jobsClient v11.JobInterface) func(w http.ResponseWriter, r *http.Request) {

	return func(w http.ResponseWriter, r *http.Request) {

		jobs, err := jobsClient.List(metav1.ListOptions{})
		if err != nil {
			panic(err.Error())
		}
		fmt.Printf("There are %d jobs in the cluster\n", len(jobs.Items))

		for _, job := range jobs.Items {
			fmt.Fprintf(w, "Job: %+v\n", job.GetName())
			fmt.Fprintf(w, "Job: %+v\n\n\n", job)
		}
	}
}

func createTransformationStep(
	stepName string,
	transformationStepImage string,
	inputQueue string,
	outputQueue string) (job batchv1.Job) {

	sidecarEnvVariables := []v1.EnvVar{
		{Name: "BROKER_URL", Value: "amqp://iterum:sinaasappel@iterum-mq-rabbitmq-ha:5672"},
		{Name: "MINIO_URL", Value: "iterum-minio:9000"},
		{Name: "MINIO_ACCESS_KEY", Value: "iterum"},
		{Name: "MINIO_SECRET_KEY", Value: "banaanappel"},
		{Name: "MINIO_OUTPUT_BUCKET", Value: stepName + "output"},
		{Name: "INPUT_QUEUE", Value: inputQueue},
		{Name: "OUTPUT_QUEUE", Value: outputQueue},
	}

	commonEnvVariables := []v1.EnvVar{
		{Name: "DATA_VOLUME_PATH", Value: "/data-volume"},
	}

	volumeMounts := []v1.VolumeMount{
		{Name: "data-volume", MountPath: "/data-volume"},
	}

	job = batchv1.Job{
		ObjectMeta: metav1.ObjectMeta{
			GenerateName: "iterum-" + stepName + "-",
			Namespace:    "default",
		},
		Spec: batchv1.JobSpec{
			Template: apiv1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{
					GenerateName: "iterum-" + stepName + "-",
				},
				Spec: apiv1.PodSpec{
					Containers: []apiv1.Container{
						{
							Name:         "iterum-sidecar",
							Image:        "iterum-sidecar:1",
							Env:          append(sidecarEnvVariables, commonEnvVariables...),
							VolumeMounts: volumeMounts,
						},
						{
							Name:         "transformation-step",
							Image:        transformationStepImage,
							Env:          commonEnvVariables,
							VolumeMounts: volumeMounts,
						},
					},
					RestartPolicy: apiv1.RestartPolicyOnFailure,
					Volumes: []apiv1.Volume{
						{Name: "data-volume", VolumeSource: v1.VolumeSource{
							EmptyDir: &v1.EmptyDirVolumeSource{},
						}},
					},
				},
			},
		},
	}

	return
}
