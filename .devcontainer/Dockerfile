FROM mcr.microsoft.com/devcontainers/rust:1

# Install core utilities
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends \
    ripgrep \
    fd-find \
    jq \
    curl \
    ca-certificates \
    postgresql \
    && apt-get clean -y && rm -rf /var/lib/apt/lists/*

# Install the RTK binary globally into /usr/local/bin (Done once, cached forever)
RUN curl -fsSL https://raw.githubusercontent.com/rtk-ai/rtk/refs/heads/master/install.sh | sh \
    && mv /root/.local/bin/rtk /usr/local/bin/rtk