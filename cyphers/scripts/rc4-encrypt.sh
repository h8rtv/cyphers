#! /bin/bash

docker run -v $(pwd):/home --rm shamelesscookie/openssl:1.1.1 rc4 -k 123456 -nosalt -in /home/data/da-vinci.txt > data/secret-rc4.rc4
