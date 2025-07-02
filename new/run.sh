

    source shell_utils/utils.sh

    DOCKER_NAME=$(generate_docker_name)

    clear &&\
        echo "building $DOCKER_NAME" &&\
        build_docker_fn "$DOCKER_NAME" || exit 1

    run_docker_fn \
        \
        "\
            bash \
        "\
        \
        "\
            -v '${PWD}/src':/src \
            --rm \
            --privileged \
            --name new \
        "\
        \
        "$DOCKER_NAME" \
        \
        -n -x