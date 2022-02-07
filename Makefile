USER_ID=$(shell id -u)
GROUP_ID=$(shell id -g)

.DEFAULT_GOAL := default

image:
	docker build --build-arg USER_ID=${USER_ID} --build-arg GROUP_ID=${GROUP_ID} -t salty  - < Dockerfile
run:
	docker run -v $(PWD):/home/salty -it salty:latest

default: image run
