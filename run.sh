CONTAINER_VERSION=rust:1.39

docker run -it --rm -w="/app" -v $(pwd):/app --user="1000:1000" -e "USER=dsthode" $CONTAINER_VERSION $*

