# circles-server
[![Build Status](https://travis-ci.org/DevHeap/circles-server.svg?branch=master)](https://travis-ci.org/DevHeap/circles-server)

## Deployment

### Database
##### Setup postgresql
Install PostgreSql and run a service
```
systemctl start postgresql
```

Setup a dotenv file with database URI:
```
echo "DATABASE_URL=postgres://USER:PASSWORD@HOST/circles" > .env
```

##### Initialize DB schema
Install a diesel_cli tool
```
cargo install diesel_cli
```

Setup database and run migrationa
```
diesel setup
diesel migration run
```

If you encountered any problems during the last step, check that .env file contains URI with right credentials and your USER all necessary rights.


### Graylog
If you already have an /etc/graylog directory, remove it.

Then run a delpoy.sh script that will setup /etc/graylog symlink for you
```
sudo ./deploy.sh
```

##### And now it's a Docker time!

Make sure you have `docker` and `docker-compose` installed, start the docker service
```
systemctl start docker
```

And start a Graylog instance
```
cd graylog/config
docker-compose up
```
