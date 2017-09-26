#!/bin/sh
ln -s $(pwd)/ /etc/graylog
chmod +x /etc/graylog
chmod g+w /etc/graylog -R
