#!/bin/bash

# login
resp=$(curl -s -X POST "http://localhost:8080/auth/login" \
-H "Content-Type: application/json" \
-d '{"email":"eleazar.fig@example.com","password":"Pa55w0rd"}')

# logout
accessToken=$(echo $resp | jq -r '.accessToken')

curl -v -X POST "http://localhost:8080/auth/logout"
