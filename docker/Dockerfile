# To run manually from project root use 'docker build -f docker/Dockerfile .'

# Base image selection
FROM manjarolinux/base

# Setting work directory
WORKDIR /app

# Labeling the image for better organization and tracking
LABEL maintainer="EANSim@bsu.by" \
     description="9K33M Simulator written with Tauri (Svelte + Vite)" \
     version="1.0.0"


# Installing necessary dependencies
RUN pacman -Syu --noconfirm && \
  pacman -Qi base-devel || pacman -S --noconfirm base-devel && \
  pacman -Qi curl || pacman -S --noconfirm curl && \
  pacman -Qi cmake || pacman -S --noconfirm cmake && \
  pacman -Qi nodejs || pacman -S --noconfirm nodejs && \
  pacman -Qi npm || pacman -S --noconfirm npm && \
  pacman -Qi rust || pacman -S --noconfirm rust && \
  pacman -Qi cargo || pacman -S --noconfirm cargo && \
  pacman -Qi webkit2gtk || pacman -S --noconfirm webkit2gtk && \
  pacman -Qi wget || pacman -S --noconfirm wget && \
  pacman -Qi file || pacman -S --noconfirm file && \
  pacman -Qi openssl || pacman -S --noconfirm openssl && \
  pacman -Qi appmenu-gtk-module || pacman -S --noconfirm appmenu-gtk-module && \
  pacman -Qi gtk3 || pacman -S --noconfirm gtk3 && \
  pacman -Qi libappindicator-gtk3 || pacman -S --noconfirm libappindicator-gtk3 && \
  pacman -Qi librsvg || pacman -S --noconfirm librsvg && \
  pacman -Qi libvips || pacman -S --noconfirm libvips

# Installing Tauri CLI
RUN cargo install tauri-cli

# Copying source code into the container
COPY . /app

# Installing npm dependencies
RUN npm install

# Building the project with Tauri
RUN cargo tauri build

# Setting the entrypoint to an interactive shell
CMD ["/bin/bash"]
