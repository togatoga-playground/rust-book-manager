#!/bin/bash

# login
resp=$(curl -s -X POST "http://localhost:8080/auth/login" \
-H "Content-Type: application/json" \
-d '{"email":"eleazar.fig@example.com","password":"Pa55w0rd"}')

accessToken=$(echo $resp | jq -r '.accessToken')

# get users
curl -v "http://localhost:8080/api/v1/users" \
-H "Authorization: Bearer $accessToken"

# register
curl -v -X POST "http://localhost:8080/api/v1/users" \
-H "Authorization: Bearer $accessToken" \
-H "Content-Type: application/json" \
-d '{"name":"yamada", "email":"yamada@example.com", "password":"hogehoge"}'


# get users
curl -v "http://localhost:8080/api/v1/users" \
-H "Authorization: Bearer $accessToken"

# register book
curl -v -X POST "http://localhost:8080/api/v1/books" \
-H "Authorization: Bearer $accessToken" \
-H "Content-Type: application/json" \
-d '{"title": "Rust book", "author": "me", "isbn": "1234567890", "description": ""}'


# list books
curl -v "http://localhost:8080/api/v1/books" \
-H "Authorization: Bearer $accessToken" \
