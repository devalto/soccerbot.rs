FROM devalto/soccerbot_base

RUN apt-get update && \
    DEBIAN_FRONTEND=noninteractive apt-get install -y --no-install-recommends curl

RUN curl -sSL https://get.docker.io/builds/Linux/x86_64/docker-1.2.0 -o /tmp/docker && \
    echo "540459bc5d9f1cac17fe8654891814314db15e77 /tmp/docker" | sha1sum -c - && \
    mv /tmp/docker /usr/local/bin/docker && \
    chmod +x /usr/local/bin/docker

ADD . /soccerbot

WORKDIR /soccerbot

RUN cargo build --release

WORKDIR /soccerbot/target/release

ADD Dockerfile.final /soccerbot/target/release/Dockerfile

CMD docker build -t devalto/soccerbot .
