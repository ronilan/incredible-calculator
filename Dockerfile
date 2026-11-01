FROM ubuntu:24.04

RUN apt-get update \
    && apt-get install -y --no-install-recommends \
        ca-certificates \
        curl \
        unzip \
    && rm -rf /var/lib/apt/lists/*

RUN curl -fL \
    https://github.com/ronilan/calculator/releases/latest/download/incredible_calculator-terminal-linux.zip \
    -o /tmp/incredible_calculator.zip \
    && unzip -o /tmp/incredible_calculator.zip -d /usr/local/bin \
    && rm /tmp/incredible_calculator.zip \
    && mv /usr/local/bin/incredible_calculator /usr/local/bin/calculator \
    && chmod +x /usr/local/bin/calculator

CMD ["/bin/bash", "-i"]