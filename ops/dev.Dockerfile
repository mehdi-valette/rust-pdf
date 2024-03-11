FROM rust:1.76-bullseye

RUN \
    apt-get update && apt-get install -y openssh-server sudo vim && \
    mkdir -p /var/run/sshd && \
    echo "PermitEmptyPasswords yes" >> /etc/ssh/sshd_config && \
    echo "PermitRootLogin yes" >> /etc/ssh/sshd_config \
    echo "StrictHostKeyChecking no" >> /etc/ssh/ssh_config

RUN adduser rust && usermod -a -G sudo rust && passwd -d rust

WORKDIR /home/rust

EXPOSE 22

CMD ["/usr/sbin/sshd", "-D", "-o", "ListenAddress=0.0.0.0"]