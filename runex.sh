$CONTAINER_VERSION=elixir:1.12

docker run \
 -it \
 --rm \
 -w="/app/${PWD##*/}" \
 -v $(pwd)/..:/app \
 --user="1000:1000" \
 $CONTAINER_VERSION \
 $*