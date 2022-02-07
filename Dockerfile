FROM rust:latest

ARG USER_ID
ARG GROUP_ID
RUN echo "root:root" | chpasswd
RUN groupadd -g $GROUP_ID salty
RUN useradd -m -r -u $USER_ID -g $GROUP_ID salty

WORKDIR /home/salty

USER salty
CMD ["/bin/bash"]
