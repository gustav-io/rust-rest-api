_build:
	docker build -t rust_api_local_dev -f Dockerfile.localdevelopment .

_run:
	docker run --rm -it -p 8000:8000 --mount type=bind,source="$(shell pwd)",target=/app --name rust_api rust_api_local_dev cargo watch -x 'run --bin api'

_stop:
	docker stop rust_api