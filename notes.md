


1. Install RabbitMQ [https://hub.helm.sh/charts/stable/rabbitmq-ha/1.21.1]
```
helm repo add stable https://kubernetes-charts.storage.googleapis.com/
helm install iterum-mq --set rabbitmqUsername=iterum,rabbitmqPassword=sinaasappel,prometheus.operator.enabled=false stable/rabbitmq-ha
```
2. Install Minio
```
helm install --set accessKey=iterum,secretKey=banaanappel iterum-minio stable/minio
```

3. Port forwards:
RabbitMQ:
```
kubectl port-forward $(kubectl get pods --namespace default -l "app=rabbitmq-ha" -o jsonpath="{.items[0].metadata.name}") --namespace default 5672:5672 15672:15672
```

Minio:
```
kubectl port-forward $(kubectl get pods --namespace default -l "release=iterum-minio" -o jsonpath="{.items[0].metadata.name}") --namespace default 9000
```


* Command to produce some data on a queue:
```
go build -o ./build/ && BROKER_URL=amqp://iterum:sinaasappel@localhost:5672 OUTPUT_QUEUE=job1_input MINIO_URL=localhost:9000 MINIO_ACCESS_KEY=iterum MINIO_SECRET_KEY=banaanappel MINIO_OUTPUT_BUCKET=inputbucket ./build/iterum-data-producer
```