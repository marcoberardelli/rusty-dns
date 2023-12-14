# rusty-dns

Update your DNS when not having a static IP addres

## Usage

$ ```rusty-dns --url youtDnsProviderEndpoint --token yourBearerToken```

### Available arguments

```
-u, --url <URL>        API endpoint to update DNS
-t, --token <TOKEN>    Bearer token
-i, --ip-api <IP_API>  API endpoint to retreive public IP address
-p, --period <PERIOD>  How often the IP address should be checked in minutes
-h, --help             Print help
-V, --version          Print version
```

### Default values

```
--ip-api=http://api.ipify.org?format=json
--period=1
```
