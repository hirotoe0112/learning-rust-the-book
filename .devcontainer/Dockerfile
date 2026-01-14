FROM rust:1.92

# Git
ARG GIT_NAME
ARG GIT_EMAIL
RUN git config --global user.name $GIT_NAME
RUN git config --global user.email $GIT_EMAIL

WORKDIR /usr/src