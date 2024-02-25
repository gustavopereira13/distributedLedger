## Nó
- Normal
	- IP - string
	- Port - int (u16)
	- ID - string(maybe bytes) = sha256 de ip:porto
- Bootstrap
	- Who knows
## UDP
- std::net::UdpSocket
	- Usar duas sockets para dar bind ao nosso IP (0.0.0.0:0)
	- Conectar ao porto do outro nó
## lib.rs
- Declarar modulos 
- testes eventualmente xDDD

## Routing e KBuckets
- KBucket:
	- Vetor de nós
	- Tamanho
- RoutingTable
	- Vetor de KBuckets?
	- Nó (perhaps apontador)
## Network
- RPCs
	- Ping
	- Store
	- find_node
	- find_value
