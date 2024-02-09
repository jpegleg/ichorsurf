![cdlogo](https://carefuldata.com/images/cdlogo.png)

# ichorsurf

Ichorsurf is a simple reference template for security and performance optimizations.

Key features:

- uses hyper-rs performance and secure TLS defaults
- uses openssl PKCS12 password protected TLS identity
- uses flume unbounded for async io with tokio hyper
- stateful UUID and UTC time data
- cloud native design

Use cases:

- processing large data files and unknown inputs
- microservice template for high performance compute backend

#### Example server log, showing sticky UUID logging

```
[ 2024-02-09 06:40:59.418781157 UTC INFO ] - INIT - Server running on https://0.0.0.0:3459
[ 2024-02-09 06:41:06.795726693 UTC INFO ] - e89817cd-76cf-4163-a9e7-3cf24429fb66 - processing handle_request of client data...
[ 2024-02-09 06:41:07.074435861 UTC INFO ] - Ok("e89817cd-76cf-4163-a9e7-3cf24429fb66") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:41:14.123156100 UTC INFO ] - - connection ended, last in ID: "e89817cd-76cf-4163-a9e7-3cf24429fb66"
[ 2024-02-09 06:41:27.418877138 UTC INFO ] - 4aeb1fd4-bb5a-4523-b897-a657053b7b40 - processing handle_request of client data...
[ 2024-02-09 06:41:29.190245373 UTC INFO ] - 4935e792-a8d8-43ed-b7e5-8cc94e05e66c - processing handle_request of client data...
[ 2024-02-09 06:41:27.674503638 UTC INFO ] - Ok("4935e792-a8d8-43ed-b7e5-8cc94e05e66c") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:41:29.467649810 UTC INFO ] - Ok("4935e792-a8d8-43ed-b7e5-8cc94e05e66c") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:41:34.952433778 UTC INFO ] - - connection ended, last in ID: "4935e792-a8d8-43ed-b7e5-8cc94e05e66c"
[ 2024-02-09 06:42:05.546014251 UTC INFO ] - - connection ended, last in ID: "4935e792-a8d8-43ed-b7e5-8cc94e05e66c"

```

We can tell when a ichorsurf is processing more than one at a time because the UID is stateful as an environment variable rather than
only representing a single transaction. If multiple requests are being processed at the same time, the logging reflect is by having the
"last in" UUID get picked up by the other threads. Even though the threads are sticky, every initial request will get a UUID generated
and inserted to the state. When multiple requetss are being processed at the same time, the last request UUID will stick to the threads
as they close out. This value UUID state value further changes as more new requests come in.

The first example logs show several requests to process HTTP bodies over 100MB in size.

This next example shows many requests to proecss smaller HTTP bodies.

```
[ 2024-02-09 06:57:33.102609164 UTC INFO ] - Ok("8133e519-a44a-4c5e-865d-5849a17522ee") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:57:33.103838340 UTC INFO ] - d29bee92-70c7-4bfa-ba8c-945684b0db87 - processing handle_request of client data...
[ 2024-02-09 06:57:33.103999285 UTC INFO ] - Ok("d29bee92-70c7-4bfa-ba8c-945684b0db87") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:57:33.104386876 UTC INFO ] - - connection ended, last in ID: "d29bee92-70c7-4bfa-ba8c-945684b0db87"
[ 2024-02-09 06:57:33.104650147 UTC INFO ] - eb2e43e2-8b3f-4349-b2a2-c0aa4875fd35 - processing handle_request of client data...
[ 2024-02-09 06:57:33.104656801 UTC INFO ] - c43365d7-4c4a-4148-b1a9-c4fe971abf01 - processing handle_request of client data...
[ 2024-02-09 06:57:33.104767964 UTC INFO ] - Ok("c43365d7-4c4a-4148-b1a9-c4fe971abf01") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:57:33.104768018 UTC INFO ] - Ok("c43365d7-4c4a-4148-b1a9-c4fe971abf01") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:57:33.108928978 UTC INFO ] - - connection ended, last in ID: "c43365d7-4c4a-4148-b1a9-c4fe971abf01"
[ 2024-02-09 06:57:33.109157170 UTC INFO ] - - connection ended, last in ID: "c43365d7-4c4a-4148-b1a9-c4fe971abf01"
[ 2024-02-09 06:57:33.114371536 UTC INFO ] - 987ee1f1-e393-4be9-ad29-d092f7b7b09a - processing handle_request of client data...
[ 2024-02-09 06:57:33.114404341 UTC INFO ] - e3a458a3-82de-4d21-a4f0-78049f865f61 - processing handle_request of client data...
[ 2024-02-09 06:57:33.114786944 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.114943258 UTC INFO ] - Ok("e3a458a3-82de-4d21-a4f0-78049f865f61") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:57:33.114943165 UTC INFO ] - Ok("e3a458a3-82de-4d21-a4f0-78049f865f61") - Successfully opened unbounded, now reading body to bytes...
[ 2024-02-09 06:57:33.115453716 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.116391831 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.117328528 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.118645366 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.119084885 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.119415479 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.120489398 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.128636131 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.134998077 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.138154169 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.138744172 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.139683352 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.139926322 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.142058840 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.146752185 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.146752233 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.147178587 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"
[ 2024-02-09 06:57:33.154677885 UTC INFO ] - - connection ended, last in ID: "e3a458a3-82de-4d21-a4f0-78049f865f61"

```

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

```
$ time curl --cacert internal_bundle.pem -d "hello world" https://ichorsurf.local:3459
{"data":"[104, 101, 108, 108, 111, 32, 119, 111, 114, 108, 100]"}
real    0m0.061s
user    0m0.008s
sys     0m0.007s
```


### Warning: no memory usage limit within

This application has an unbounded queue open, meaning data of
any size can be inserted, until the server can't hold anything 
else in RAM, at which point the ichorsurf will get `killed`.
If in a container orchestration system, that might then
trigger a new container to be created.

Limits on resource consumption can be put in place, but
the killing behavior intentionally remains. The design is to allow long
and large data streams without trying to prevent completion, 
at all costs.

This can result in long TCP streams of large data files,
as long as there is enough RAM to load the files.

<b>Without sufficient protections to restrict access,
this service can cause memory exhaustion if users
send large data files or hold streams open with
continuous data.

If more refined and normal HTTP services are needed, I suggest Actix, see https://github.com/jpegleg/morpho-web

Front-ends would typically want to be more refined than this. Actix, and others, have all the tooling built out, no need to re-invent the wheel.
But ichorsurf is intentionally simplistic, such as so a custom service might benefit from the streaming and performance properties.
</b>
