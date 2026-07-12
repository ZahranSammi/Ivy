FROM golang:1.21-alpine AS builder

RUN apk add --no-cache git
RUN go install -v github.com/owasp-amass/amass/v4/...@latest

FROM alpine:3.19
RUN apk add --no-cache bash jq ca-certificates bind-tools
COPY --from=builder /go/bin/amass /usr/local/bin/amass

# Create non-root user
RUN addgroup -S ivy && adduser -S ivy -G ivy
USER ivy

WORKDIR /app
COPY entrypoint.sh /app/entrypoint.sh
# Ensure executable
# RUN chmod +x /app/entrypoint.sh (would be done via git/host)

ENTRYPOINT ["/bin/bash", "/app/entrypoint.sh"]
