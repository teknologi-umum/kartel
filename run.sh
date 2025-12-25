echo "pulling docker image..."
docker compose pull

echo "tearing down current running container..."
docker compose down

echo "starting up from new image..."
docker compose up -d --remove-orphans
sleep 3

echo "docker status"
docker compose ps

echo "docker logs"
docker compose logs --tail=30 kartel

echo "clean up old images"
docker image prune -f
