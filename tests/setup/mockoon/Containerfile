FROM node:18-alpine

RUN npm install -g @mockoon/cli@5.1.0
COPY ./mockoon_definition.json ./mockoon_definition.json

# Install curl for healthcheck and tzdata for timezone support.
RUN apk --no-cache add curl tzdata

# Do not run as root.
RUN adduser --shell /bin/sh --disabled-password --gecos "" mockoon
RUN chown -R mockoon ./mockoon_definition.json
USER mockoon

EXPOSE 7777

ENTRYPOINT ["mockoon-cli","start","--disable-log-to-file","--data","./mockoon_definition.json","--port","7777"]

# Usage: docker run -p <host_port>:<container_port> mockoon-test