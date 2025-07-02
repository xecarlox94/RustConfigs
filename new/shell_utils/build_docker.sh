

build_docker_fn () {

    DOCKER_NAME=$1

    #sudo docker \
        #system prune \
            #-a \
            #--filter "until=4w"

    # sudo docker rmi $(docker images -f dangling=true)
    # sudo docker volume rm $(sudo docker volume ls -q -f dangling=true)


    sudo \
        docker build . \
            -t "$DOCKER_NAME"

            #
            #
            #  I need to add ARG environments to extend further things, such as:
            #
            #  adding user to container user group
            #
            #--build-arg BUILD_ENV=dev

}

            