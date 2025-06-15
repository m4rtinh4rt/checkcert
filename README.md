# checkcert

checkcert is a command-line tool that verifies the fingerprints of TLS
certificates used by specific targets by comparing them to a reference list.

## usage

```
$ checkcert 
Usage: checkcert FILENAME
```

The target file should be formatted as follows:

```
# domain:port fingerprint (SHA256)
mpah.dev:443 4f4274a9fbbbe9e559234d422d6e6f8ce15709ea062c0318559b407181c3045c
git.mpah.dev:443 9c2a28331078239c0dd45441083216895ed74745f27d2997a93bc8e074621d12
...
```
