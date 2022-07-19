# base layer for rust builds
# https://github.com/LukeMathWalker/cargo-chef
FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

# layer that plans xtask dependencies to cache
FROM chef AS xtask-planner
COPY ./.xtask/Cargo.toml ./Cargo.toml
COPY ./xtask/Cargo.toml ./xtask/Cargo.toml
COPY ./saucer/Cargo.toml ./saucer/Cargo.toml
RUN set -eux; \
		cargo chef prepare --recipe-path recipe.xtask.json; \
		echo "Prepared xtask dependency plan!"

# layer that plans web dependencies to cache
FROM chef AS web-planner

COPY ./Cargo.toml .
COPY ./Cargo.lock  .
COPY ./awc-cli/Cargo.toml ./awc-cli/Cargo.toml
COPY ./awc-web/Cargo.toml ./awc-web/Cargo.toml
COPY ./awc-lib/Cargo.toml ./awc-lib/Cargo.toml
COPY ./saucer/Cargo.toml ./saucer/Cargo.toml
COPY ./xtask/Cargo.toml ./xtask/Cargo.toml
RUN set -eux; \
		cargo chef prepare --recipe-path recipe.web.json; \
		echo "Prepared web dependency plan!"

# layer that installs node
FROM debian:11.3-slim AS volta-installer

ENV VOLTA_VERSION="v1.0.8"
ENV VOLTA_HOME="/volta"
ENV PATH="$VOLTA_HOME/bin:$PATH"	

RUN set -eux; \
		export DEBIAN_FRONTEND=noninteractive; \
	  apt update; \
		apt install --yes --no-install-recommends ca-certificates curl; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/; \
		echo "Installed base utils!"

RUN set -eux; \
		mkdir -p $VOLTA_HOME; \
    curl -sSL https://get.volta.sh | bash -s -- --skip-setup; \
		which volta; \
		volta install node@16; \
    volta install npm@8; \
		which node; \
		which npm; \
		echo "Installed Node!"

# layer that builds xtask
FROM chef AS xtask-builder

ENV VOLTA_VERSION="v1.0.8"
ENV VOLTA_HOME="/volta"
ENV PATH="$VOLTA_HOME/bin:$PATH"	
ENV IS_DOCKER=1

COPY --from=volta-installer $VOLTA_HOME $VOLTA_HOME

WORKDIR /app
RUN set -eux; \
		export DEBIAN_FRONTEND=noninteractive; \
	  apt update; \
		apt install --yes --no-install-recommends pkg-config libssl-dev; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/; \
		echo "Installed base utils!"

COPY --from=xtask-planner /app/recipe.xtask.json recipe.xtask.json

RUN set -eux; \
		cargo chef cook --release --recipe-path recipe.xtask.json; \
		echo "Compiled xtask Rust dependencies!"

COPY ./.xtask/Cargo.toml ./Cargo.toml
COPY ./xtask ./xtask
COPY ./saucer ./saucer

RUN set -eux; \
		cargo build -p xtask --release; \
		echo "Compiled 'xtask'!"

FROM chef as awc-web-builder

WORKDIR /app

COPY --from=web-planner /app/recipe.web.json ./recipe.web.json

RUN set -eux; \
		cargo chef cook --release --recipe-path recipe.web.json; \
		echo "Compiled 'awc-web' Rust dependencies!"

COPY . .

RUN set -eux; \
		cargo build -p awc-web --release; \
		echo "Compiled 'awc-web'!"

RUN set -eux; \
		NODE_ENV="production" ./target/release/xtask web bundle all; \
		echo "Built static assets!"

################################################################################
FROM debian:11.3-slim


RUN set -eux; \
		export DEBIAN_FRONTEND=noninteractive; \
	  apt update; \
		apt install --yes --no-install-recommends bind9-dnsutils iputils-ping iproute2 curl ca-certificates htop; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/; \
		echo "Installed base utils!"

WORKDIR /app

# keep the public directory in the same place so that relative paths
# in awc.prod.json still look correct in source code
RUN set -eux; \
		mkdir ./awc-web

CMD ["./awc-bin"]
EXPOSE 8080

COPY --from=awc-web-builder /app/target/release/awc-web ./awc-bin
COPY --from=awc-web-builder /app/awc-web/public ./awc-web/public
