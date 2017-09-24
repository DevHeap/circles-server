#!/bin/sh
ln -s $(pwd)/graylog /etc/graylog
chmod +x /etc/graylog
chmod g+w /etc/graylog -R
