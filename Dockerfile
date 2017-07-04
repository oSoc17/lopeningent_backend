############################################################
# Dockerfile to run a Django-based web application
# Based on an Ubuntu Image
############################################################

# Set the base image to use to Ubuntu
FROM ubuntu:14.04

# Set the file maintainer (your name - the file's author)
MAINTAINER Olivier Cammaert

# Set env variables used in this Dockerfile (add a unique prefix, such as DOCKYARD)
# Local directory with project source
ENV DOCKYARD_SRC=djangoserver
# Directory in container for all project files
ENV DOCKYARD_SRVHOME=/srv
# Directory in container for project source files
ENV DOCKYARD_SRVPROJ=/srv/djangoserver

# Update the default application repository sources list
RUN apt-get update && apt-get -y upgrade
RUN apt-get install -y libncurses5-dev libffi-dev
RUN apt-get install -y python2.7-dev python-pip
# RUN apt-get install -y pypy
RUN apt-get install -y libgeos-dev
RUN apt-get install -y curl

# RUST STUFF
ENV RUST_VERSION stable
ENV CARGO_HOME /cargo
ENV PATH $CARGO_HOME/bin:/root/.cargo/bin:$PATH
ENV SRC_PATH /src
RUN curl -sSf https://sh.rustup.rs | env -u CARGO_HOME sh -s -- -y --default-toolchain "$RUST_VERSION" \
  && rustc --version && cargo --version \
  && mkdir -p "$CARGO_HOME" "$SRC_PATH"

# Create application subdirectories
WORKDIR $DOCKYARD_SRVHOME
RUN mkdir media static logs
VOLUME ["$DOCKYARD_SRVHOME/media/", "$DOCKYARD_SRVHOME/logs/"]

# Copy application source code to SRCDIR
COPY $DOCKYARD_SRC $DOCKYARD_SRVPROJ

# Install Python dependencies
RUN pip install -r $DOCKYARD_SRVPROJ/requirements.txt

# Port to expose
EXPOSE 8000

# Copy entrypoint script into the image
WORKDIR $DOCKYARD_SRVPROJ
COPY ./docker-entrypoint.sh /
ENTRYPOINT ["/docker-entrypoint.sh"]
