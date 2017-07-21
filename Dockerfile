# Debian testing is like the stable version of most distributions,
# we're using this because it allows for more recent packages. 
FROM debian:testing

# Hi, nice to meet you!
MAINTAINER Tim Baccaert 

# We've got to make sure everything is upgraded.
RUN apt-get update && apt-get upgrade -yqq

# Now we can install some dependencies. 
RUN apt-get install -yqq \
	libncurses5-dev \
	python2.7-dev \
	libffi-dev \
	libgeos-dev \
	libspatialindex-dev \
	python-pip \
	osmosis \
	curl

# Install the Rust programming language environment
ENV RUST_VERSION stable
ENV CARGO_HOME /cargo
ENV PATH $CARGO_HOME/bin:/root/.cargo/bin:$PATH
ENV SRC_PATH /src
RUN curl -sSf https://sh.rustup.rs | env -u CARGO_HOME sh \
	-s -- -y --default-toolchain  "$RUST_VERSION" \
	&& rustc --version && cargo --version \
	&& mkdir -p "$CARGO_HOME" "$SRC_PATH"

# Just putting our application into the proper directories.
ADD server /opt/lig-server
ADD data /opt/lig-data
WORKDIR /opt/lig-server

# Install the python dependencies.
RUN pip install -r requirements.txt

# Now we can start running the server.
# This could be changed to an nginx+letsencrypt configuration later.
EXPOSE 8000