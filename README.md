# circles-server

### Deploy and start Graylog
If you already have an /etc/graylog directory, remove it.

Then run a delpoy.sh script that will setup /etc/graylog symlink for you
```
sudo ./deploy.sh
```

##### And now it's a Docker time!

Make sure you have `docker` and `docker-compose` installed

And start a Graylog instance
```
cd graylog/config
docker-compose up
```
