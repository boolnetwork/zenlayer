
# IMAGE ZenLayer-NODE
FROM ubuntu:20.04 as zenlayer
LABEL maintainer "BoolNetwork Team"

RUN apt-get update && \
	apt-get install -y --no-install-recommends curl dstat

COPY  ./zenlayer /usr/local/bin
COPY  ./specs/zenlayer-testnet.json /specs/zenlayer-testnet.json
COPY  ./specs/zenlayer-mainnet.json /specs/zenlayer-mainnet.json

RUN	useradd -m -u 1000 -U -s /bin/sh -d /zenlayer zenlayer && \
	mkdir -p /zenlayer/.local/share/zenlayer && \
	chown -R zenlayer:zenlayer /zenlayer/.local && \
	ln -s /zenlayer/.local/share/zenlayer /data

USER zenlayer
EXPOSE 30333 9933 9944
VOLUME ["/data"]

ENTRYPOINT ["/usr/local/bin/zenlayer"]


