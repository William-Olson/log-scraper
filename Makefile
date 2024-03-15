
dev: build
	@COMMENT='Running Service Now . . .' make -s flower-box
	cargo run

tag: CURRENT_VERSION="$(shell yq '.package.version' ./Cargo.toml)"

tag: build-container
	@COMMENT='Tagging Docker Image . . .' make -s flower-box
	@if [[ -z "$(TAG)" ]]; then \
		echo "Error: Please specifiy the Docker tag with the TAG environment variable. " ;\
		echo "i.e. \n\t TAG=$(CURRENT_VERSION) make tag"; echo; exit 1; \
	else \
		echo "Tagging image willko/log-scraper:"$(TAG) ; \
		docker tag willko/log-scraper:latest willko/log-scraper:$(TAG) ; \
	fi

with-docs: build docs
	@COMMENT='Running Service Now . . .' make -s flower-box
	cargo run

start-container: clean-containers build-container
	@COMMENT='Starting Docker Containers' make -s flower-box
	./scripts/docker_dev_run.sh

compose-start: compose-cleanup build-containers
	@COMMENT='Starting Compose Containers' make -s flower-box
	docker-compose -f dev.docker-compose.yml up -d

compose-cleanup:
	@COMMENT='Cleaning Up Any Old Containers' make -s flower-box
	docker-compose -f dev.docker-compose.yml kill || true
	docker-compose -f dev.docker-compose.yml rm -y || true

compose-scraper-restart:
	@COMMENT='Restarting Scraper Container' make -s flower-box
	docker-compose stop -t 1 log-scraper
	./scripts/docker_build.sh
	docker-compose up --no-start log-scraper
	docker-compose start log-scraper

clean-containers:
	@COMMENT='Cleaning Containers' make -s flower-box
	./scripts/docker_cleanup.sh

build-containers: build-container build-container-ui
	@COMMENT='Buildign log-scraper Docker Container' make -s flower-box
	./scripts/docker_build.sh

build-container-ui:
	@COMMENT='Building Web Container' make -s flower-box
	./scripts/docker_web_build.sh

build-container: lint web-build
	@COMMENT='Building log-scraper Docker Container' make -s flower-box
	./scripts/docker_build.sh

lint:
	@COMMENT='Linting Project' make -s flower-box
	@cargo clippy

docs: clean
	@COMMENT='Building Docs' make -s flower-box
	@cargo doc --document-private-items

	@mv ./target/doc ./docs

build: lint web-build
	@COMMENT='Building Rust Service' make -s flower-box
	cargo build

web-build: clean
	@COMMENT='Building Web UI' make -s flower-box
	@cd ./web && npm run pre-dev

clean:
	@COMMENT='Cleaning Workspace' make -s flower-box
	@mkdir -p ./build
	rm -rf ./build
	@mkdir -p ./web/build
	rm -rf ./web/build
	@mkdir -p ./docs
	rm -rf ./docs

flower-box:
	@echo
	@echo '#---------------------------------------------#'
	@echo '#      ${COMMENT}'
	@echo '#---------------------------------------------#'
	@echo
