#!/bin/bash

#(cd $1
#git checkout . 
#git pull origin encapsulation
#)

#rustup default 1.20.0

export SCHEMA=lopeningent2
export DATABASE_PASSWORD=idlab_lopeningent cd $1
#( cd data && cat lopeningent_schema.sql | sed 's/$SCHEMA/'"$SCHEMA"'/g' | sudo -u postgres psql)
#(cd data && ./migrate.py)
(cd server/routing_server && cargo clean && cargo build --release)
(cd server/routing_server && cargo run --release)&
#(cd server && python manage.py runserver 8001)&
