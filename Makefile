
image-transformation-step:
	docker build -t transformation-step:1 ./transformation-step
	kind load docker-image transformation-step:1 --name kind

image-sidecar:
	kind load docker-image iterum-sidecar:1 --name kind

image-data-producer:
	docker build -t data-producer:1 ./data-producer
	kind load docker-image data-producer:1 --name kind

job:
	kubectl delete -f job.yaml
	kubectl apply -f job.yaml