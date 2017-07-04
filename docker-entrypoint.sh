#!/bin/bash
echo "DEBUG = False" > /srv/djangoserver/server/setdebug.py
echo "ALLOWED_HOSTS = ['0.0.0.0', 'localhost', 'groep16.cammaert.me', '193.190.127.140']" >> /srv/djangoserver/server/setdebug.py
echo "NUM_THREADS = 1" >> /srv/djangoserver/server/setdebug.py

# curl https://sh.rustup.rs -sSf | sh -s -- -y
# rustup install stable
cargo build --release

python manage.py migrate                  # Apply database migrations
python manage.py collectstatic --noinput  # Collect static files
# coverage run --source='.' manage.py test  # Run tests
# coverage xml -i
python manage.py test --with-xunit --with-coverage --cover-xml
pylint --load-plugins pylint_django server -r n --msg-template="{path}:{line}: [{msg_id}({symbol}), {obj}] {msg}" > pylint.txt

# Prepare log files and start outputting logs to stdout
touch /srv/logs/gunicorn.log
touch /srv/logs/access.log
tail -n 0 -f /srv/logs/*.log &


NUM_WORKERS=1
TIMEOUT=300
# Start Gunicorn processes
echo Starting Gunicorn.
exec gunicorn server.wsgi:application \
    --name drig \
    --bind 0.0.0.0:8000 \
    --workers $NUM_WORKERS \
    --timeout $TIMEOUT \
    --log-level=info \
    --log-file=/srv/logs/gunicorn.log \
    --access-logfile=/srv/logs/access.log \
    "$@"

# echo Starting the server
# python manage.py runserver 0.0.0.0:8000
