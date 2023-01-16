# `rust-dns`

Good morning! This is a toy for playing with DNS. It can do a few things:

* Run an DNS echo server (that can be queried using `dig`)
* Read DNS packets stored on disk

This is a playground to improve my understanding of DNS at the packet level, so
I can implement an embedded nameserver in another project. I'm making it public
in case it's useful for others :)

On the roadmap:

* DNS server that HTTP clients can resolve against
  * i.e. a (local) authoritative nameserver
* TCP support
* Tests

## Usage

### Reading a packet from file

#### Using an example file

```
$ cargo run --bin cli -- read examples/question_packet
$ ...
```

#### Generating your own DNS packet

First, listen on a port using netcat:

```
$ nc -u -l 3000 > question_packet
```

Then, in a seperate terminal, make a DNS request to it using `dig`:

```
$ dig +retry=0 -p 3000 @127.0.0.1 +noedns example.com
```

When this completes, you can stop the netcat server and parse the datagram:

```
$ cargo run --bin cli -- read question_packet
$ ...
```

### Run a DNS echo server

This will receive UDP packets, parse and print them, and then echo them back to the caller.

```
$ cargo run --bin cli -- launch --addr 127.0.0.1:3000
$ dig +retry=0 -p 3000 @127.0.0.1 +noedns example.com
```

## Project structure

* `/cli` contains the commandline interface and UDP server
* `/core` contains the DNS packet parsing logic

## References
* [Domain names (RFC 1035, 1987)](https://www.ietf.org/rfc/rfc1035.txt)
* [How DNS works (Julia Evans, 2021)](https://wizardzines.com/zines/dns/)
* [DNS Guide (Emil Hernvall, 2017)](https://github.com/EmilHernvall/dnsguide)