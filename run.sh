CONTAINER_VERSION=rust:1.39

docker run -it --rm -w="/app/${PWD##*/}" -v $(pwd)/..:/app --user="1000:1000" -e "USER=dsthode" $CONTAINER_VERSION $*

