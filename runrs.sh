CONTAINER_VERSION=rust:1.66

docker run \
	-it \
	--rm \
	-w="/app/${PWD##*/}" \
	-v $(pwd)/..:/app \
	--user="1000:1000" \
	-e "USER=dsthode" \
	-e "RUST_BACKTRACE=1" \
	$CONTAINER_VERSION \
	$*

