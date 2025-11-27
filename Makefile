COMPOSE_COMMAND=$(shell \
	which docker-compose > /dev/null \
	&& echo 'docker-compose' \
	|| echo 'docker compose')

up: .env dev.env prod.env
	$(COMPOSE_COMMAND) up

upd: .env dev.env prod.env
	$(COMPOSE_COMMAND) up -d

down: .env dev.env prod.env
	$(COMPOSE_COMMAND) down

.env:
	echo "SITE_MODE=dev" > .env

dev.env:
	echo "TUNNEL_TOKEN=fixme" > dev.env
	echo "NO_AUTOUPDATE=1" >> dev.env

prod.env:
	echo "TUNNEL_TOKEN=fixme" > prod.env
	echo "NO_AUTOUPDATE=1" >> prod.env

site:
	uv run main.py

deploy: site
	rsync -a site morgana:~/projects/mm4cc-web/

clean:
	rm -r site
