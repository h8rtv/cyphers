#! /bin/bash

docker run -v $(pwd):/home --rm shamelesscookie/openssl:1.1.1 rc4 -d -k 123456 -nosalt -in /home/data/secret-rc4.rc4
