# HRC Root Finance lending market blueprint

Forked from Weft Finance lending market blueprint
visit https://docs.weft.finance for more information

This project is licensed under the [Apache License version 2.0](http://www.apache.org/licenses/LICENSE-2.0) - see the [LICENSE](LICENSE) file for details.


#### Building

```
DOCKER_DEFAULT_PLATFORM=linux/amd64 docker pull radixdlt/scrypto-builder:v1.1.2
DOCKER_DEFAULT_PLATFORM=linux/amd64 docker run --name scrypto_builder --rm -v $(pwd):/src --network=host radixdlt/scrypto-builder:v1.1.2
```