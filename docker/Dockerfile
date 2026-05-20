# Build backend
FROM rust:1-bookworm AS rust-builder
WORKDIR /app
COPY backend/ backend/
RUN cd backend && cargo build --release -p api

# Build frontend
FROM node:22-bookworm AS frontend-builder
WORKDIR /app
COPY frontend/package*.json frontend/
RUN cd frontend && npm ci
COPY frontend/ frontend/
RUN cd frontend && npm run build

# Runtime
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=rust-builder /app/backend/target/release/dept-tracker /app/dept-tracker
COPY --from=frontend-builder /app/frontend/build /app/static
ENV DATA_DIR=/data STATIC_DIR=/app/static
EXPOSE 8080
VOLUME ["/data"]
CMD ["/app/dept-tracker"]
