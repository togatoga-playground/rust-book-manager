#!/bin/bash

curl -v -X POST "http://localhost:8080/auth/login" \
-H "Content-Type: application/json" \
-d '{"email":"eleazar.fig@example.com","password":"Pa55w0rd"}'
