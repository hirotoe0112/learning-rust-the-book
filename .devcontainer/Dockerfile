FROM rust:1.92

# Locale settings (English with UTF-8 for Japanese input support)
RUN apt-get update && apt-get install -y locales vim \
    && sed -i '/en_US.UTF-8/s/^# //g' /etc/locale.gen \
    && locale-gen
ENV LANG=en_US.UTF-8
ENV LC_ALL=en_US.UTF-8

# Git
ARG GIT_NAME
ARG GIT_EMAIL
RUN git config --global user.name $GIT_NAME
RUN git config --global user.email $GIT_EMAIL
RUN git config --global core.editor "vim"

WORKDIR /usr/src