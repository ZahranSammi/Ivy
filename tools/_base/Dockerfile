FROM alpine:3.19

# Install common utilities needed by all tools
RUN apk add --no-cache \
    bash \
    jq \
    curl \
    ca-certificates

# Create a non-root user for execution
RUN addgroup -S ivy && adduser -S ivy -G ivy
USER ivy

WORKDIR /app
