# Stokenet resource faucet 

#### Building

```
DOCKER_DEFAULT_PLATFORM=linux/amd64 docker pull radixdlt/scrypto-builder:v1.1.2
DOCKER_DEFAULT_PLATFORM=linux/amd64 docker run --name scrypto_builder --rm -v $(pwd):/src --network=host radixdlt/scrypto-builder:v1.1.2
```