.PHONY: all re build down debug

all:
	docker-compose up -d

re:
	sudo docker-compose down
	sudo docker-compose up --build -d

build:
	sudo docker-compose up --build -d

down:
	sudo docker-compose down

debug:
	sudo docker-compose up --build
