set shell := ["powershell.exe", "-c"]

default:
	just --list

docker-build:
	docker image build -t landonwork/aoc2023:latest .

docker-pull:
	docker pull landonwork/aoc2023:latest

docker-run:
	docker run -p 80:80 --init landonwork/aoc2023:latest

docker-run-detached:
	docker run -d -p 80:80 --init landonwork/aoc2023:latest
