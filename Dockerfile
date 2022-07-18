FROM rust:slim AS builder

WORKDIR /app
COPY . .

RUN set -eux; \
		export DEBIAN_FRONTEND=noninteractive; \
	  apt update; \
		apt install --yes --no-install-recommends pkg-config libssl-dev curl; \
		apt clean autoclean; \
		apt autoremove --yes; \
		rm -rf /var/lib/{apt,dpkg,cache,log}/; \
		echo "Installed base utils!"

ENV VOLTA_VERSION="v1.0.8"
ENV VOLTA_HOME="/volta"
ENV PATH="$VOLTA_HOME/bin:$PATH"
ENV IS_DOCKER=1

RUN --mount=type=cache,target=/app/target \
		--mount=type=cache,target=/app/node_modules \
		--mount=type=cache,target=/usr/local/cargo/registry \
		--mount=type=cache,target=/usr/local/cargo/git \
		--mount=type=cache,target=/usr/local/cargo/bin \
		--mount=type=cache,target=/usr/local/rustup \
		--mount=type=cache target=${VOLTA_HOME} \
		set -eux; \
		rustup install stable; \
		mkdir -p $VOLTA_HOME; \
    curl -sSL https://get.volta.sh | bash -s -- --skip-setup; \
		which volta; \
		volta install node@16; \
    volta install npm@8; \
		which node; \
		which npm; \
	 	NODE_ENV="production" cargo build -p awc-server --release; \
		objcopy --compress-debug-sections target/release/awc-server ./awc

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

COPY --from=builder /app/awc ./awc
COPY --from=builder /app/crates/awc-server/public ./public
CMD ["app/awc"]