# Use the latest postgres image
FROM postgres

# Hi, nice to meet you!
MAINTAINER Tim Baccaert

# Database is called lopeningent.
ENV POSTGRES_DB lopeningent

# Initialise the database with our schema.
COPY ./lopeningent_schema.sql /docker-entrypoint-initdb.d/