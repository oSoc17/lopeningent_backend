# #LopenInGent
![oSoc17 #LopenInGent Banner](https://raw.githubusercontent.com/oSoc17/lopeningent_backend/develop/assets/banner.jpeg "#LopenInGent Banner")

The **#LopenInGent project** is an effort to create an *enjoyable, customizable and dynamic running app* for the city of **Ghent**. Here's a showcase of our **major features**: 

- Running **route generation** based on a **distance or an amount of time** (eg. I want to run 5 km or I want to run for 30 minutes)

- Route **recalculation** if you go off the given route, the recalculation will be within the distance/time constraint.

- Routes always go **back to the starting point**, it will never take the same route to go back.

- It's possible to **add distance or time while you're running**.

- Users can select **multiple attributes for a given run**. (eg. I want to run on soft ground or I want to run next to parks)

- **Audio guidance** while you are running (Heartrate feedback, directions, ...)

- Keep track of your **running statistics** and previous runs.

## Installation

The HTTP server is written in Python, the routing algorithm is written in Rust

Make sure you have `python2.7`,  a recent `Rust` & `PostgreSQL` version and some flavour of `GNU/Linux`.

First off, create a python virtualenv using [virtualenvwrapper](https://virtualenvwrapper.readthedocs.io):
```
$ mkproject lopeningent_backend
```

Now, you can clone the development branch
```
$ git clone git@github.com:oSoc17/lopeningent_backend
```

After the clone has finished, move into the directory and install the python third-party packages:
```
$ cd lopeningent_backend
$ pip intall requirements.txt
```

the packages depend on a list of Linux shared libraries, a comprehensive list will follow later.
```
Coming soon
```

Load the `data/lopeningent_schema.sql` into your `PostgreSQL` server and update the settings in `data/migrate_config.py`.
To load the data into the database run: 
```
$ cd data
$ ./update.sh
```

To start the server, go into the `server` directory and run the command:
```
python manage.py runserver
```

You can find detailed installation instructions on the repository's [wiki page](https://github.com/oSoc17/lopeningent_backend/wiki).

## Documentation
You can find detailed documentation on the [wiki page](https://github.com/oSoc17/lopeningent_backend/wiki).

## MIT License
This project is released as an open-source project under the [MIT License](https://github.com/oSoc17/lopeningent_backend/blob/develop/LICENSE).
