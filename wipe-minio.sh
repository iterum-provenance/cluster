
docker run --network host --name miniocli -d -it --entrypoint=/bin/sh minio/mc
docker exec miniocli mc config host add microk8s http://localhost:9000 iterum banaanappel
docker exec miniocli mc rb --force --dangerous microk8s
docker kill miniocli
docker rm miniocli
