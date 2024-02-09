FROM scratch
COPY target/x86_64-unknown-linux-musl/release/ichorsurf /ichorsurf
EXPOSE 3459
CMD ["/ichorsurf"]
