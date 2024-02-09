![cdlogo](https://carefuldata.com/images/cdlogo.png)

# ichorsurf

Ichorsurf is a simple reference template for security and performance optimizations.

Key features:

- uses hyper-rs performance and secure TLS defaults
- uses openssl PKCS12 password protected TLS identity
- uses flume unbounded for async io with tokio hyper
- transactional UUIDs and UTC time data
- cloud native design

Use cases:

- processing large data files and unknown inputs
- microservice template for high performance compute backend

## Required environment variables

```
PKCSPATH is the path to the PKCS12 file
PKCSPASWORD is the password to the PKCS12 file
```
Also note that the template uses the environment variable "txid" for telemetry/logging purposes.
The txid variable is overwritten as events occur and does not need to be set. The other two
do need to be set, such as from a kubernetes secret object.

### The demo 

The demo simply returns the data sent in the request body 
in a JSON byte vector (byte array). Some people might
find just this functionality interesting, but most uses
would replace the demo with whatever backend functionality
is desired. Ichorsurf is designed for fast encryption
and cryptography operations, or otherwise data
processing tasks triggered over a network connection.


