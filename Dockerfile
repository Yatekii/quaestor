# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM ekidd/rust-musl-builder:stable AS cargo-build

# Copy the source
COPY src/ src/
COPY Cargo.toml Cargo.toml

# Build the application.
RUN cargo build --release

# ------------------------------------------------------------------------------
# Node Build Stage
# ------------------------------------------------------------------------------

FROM node:lts-alpine AS node-build

# make the 'app' folder the current working directory
WORKDIR /app

# copy project files and folders to the current working directory (i.e. 'app' folder)
COPY ./frontend .

# install project dependencies
RUN yarn install

# build app for production with minification
RUN yarn build

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM alpine:3.13 as runtime

# Install weasyprint dependencies.
RUN apk \
    --update \
    --upgrade \
    --no-cache \
    add \
    cairo-dev \
    pango-dev \
    gdk-pixbuf-dev \
    ttf-dejavu

ADD requirements.txt requirements.txt

RUN set \
    -ex \
    && apk \
    --no-cache \
    --virtual .build-deps \
    add \
    gcc \
    musl-dev \
    jpeg-dev \
    zlib-dev \
    libffi-dev \
    python3-dev \
    py3-virtualenv \
    && python3 -m virtualenv --python=/usr/bin/python3 venv \
    && . venv/bin/activate \
    && pip3 \
    install \
    --no-cache-dir \
    -r requirements.txt \
    && apk \
    del .build-deps

WORKDIR /app

COPY --from=cargo-build /home/rust/src/target/x86_64-unknown-linux-musl/release/quaestor .
COPY --from=node-build /app/dist ./dist
COPY templates/ ./templates/

CMD cd /app && quaestor
