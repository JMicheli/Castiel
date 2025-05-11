# Stage 1: Build the frontend
FROM node:lts-alpine as frontend_builder

WORKDIR /app

COPY frontend/ /app
RUN npm install
RUN npm run build

# Stage 2: Build the Rust application
FROM rust:latest as rust_builder

WORKDIR /app

COPY . .
RUN cargo build --release

# Stage 3: Final image
FROM debian:buster-slim

WORKDIR /app

COPY --from=frontend_builder /app/dist ./dist
COPY --from=rust_builder /app/target/release/castiel .

# Define the entrypoint
CMD ["./castiel"]
