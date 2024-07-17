# Feedback system

## What is needed
Docker, Docker Compose, Make

## How to use
### 1. Copy .env.example -> .env
and fill it

### 2. Go to frontend/sveltekit/srcs/utils/utils.js
modify BASE_URL(backend server url/address)

### 3. make
```
make
# start == docker compose up

make build
# -- build

make re
```
### 4. Example
아직 안만듦

## What is included?
Sveltekit with just node server  
Actix web backend  
PostgreSQL Container



