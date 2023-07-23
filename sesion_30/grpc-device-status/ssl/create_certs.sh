#!/bin/sh
mkdir -p keys
cd keys/

# source: https://users.rust-lang.org/t/use-tokio-tungstenite-with-rustls-instead-of-native-tls-for-secure-websockets/90130

# Create unencrypted private key and a CSR (certificate signing request)
openssl req -newkey rsa:2048 -nodes -subj "/C=FI/CN=vahid" -keyout ssl/key.pem -out ssl/key.csr

# Create self-signed certificate (`cert.pem`) with the private key and CSR
openssl x509 -signkey key.pem -in ssl/key.csr -req -days 365 -out ssl/cert.pem

# Create a self-signed root CA
openssl req -x509 -sha256 -nodes -subj "/C=FI/CN=vahid" -days 1825 -newkey rsa:2048 -keyout ssl/rootCA.key -out ssl/rootCA.crt


# Create file localhost.ext with the following content:
cat <<'EOF' >> localhost.ext
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
subjectAltName = @alt_names
[alt_names]
DNS.1 = server
IP.1 = 127.0.0.1
EOF

# Sign the CSR (`cert.pem`) with the root CA certificate and private key
# => this overwrites `cert.pem` because it gets signed
openssl x509 -req -CA ssl/rootCA.crt -CAkey ssl/rootCA.key -in ssl/key.csr -out ssl/cert.pem -days 365 -CAcreateserial -extfile localhost.ext
