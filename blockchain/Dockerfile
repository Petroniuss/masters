FROM ghcr.io/foundry-rs/foundry:latest
WORKDIR usr/src

RUN forge build --skip src --use solc:0.8.17

COPY . .
RUN forge build
RUN forge test

ENTRYPOINT ["sh", "./entrypoint.sh"]