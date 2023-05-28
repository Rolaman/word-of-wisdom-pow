# Word of Wisdom Service with Proof Of Work DDOS Protection

## Task

**Design and implement “Word of Wisdom” tcp server**

- TCP server should be protected from DDOS attacks with the Prof of Work, the challenge-response protocol should be used.
- The choice of the POW algorithm should be explained
- After Prof Of Work verification, server should send one of the quotes from “word of wisdom” book or any other collection of the quotes
- Docker file should be provided both for the server and for the client that solves the POW challenge

## How to run

Run server
```
docker-compose up -d server
// You can set env HOST. Default - 0.0.0.0:8001
```

Run client
```
docker-compose up -d client
// You can set env HOST. Default - 0.0.0.0:8001
```

## Implementation

- Client established a new tcp connection with a server
- Server generated a new challenge(time-based) with a difficulty(=2)
- Server sent the message to the client
- Client parsed the challenge and the difficulty
- Client calculated the right nonce to solve it
- Client send the solution with the challenge
- If nonce was valid, the server sent a quote from word of wisdom book
- If nonce was invalid, the server closed the stream

## PoW Algo

**I used Hashcash with SHA256 hashing PoW algorithm to set DDOS protection**

- It is fast to implement and a lot of libs provide sha256 hashing
- Easy to check the solution
- Do not consume much memory, so even clients with small memory can use this service
- In this and many cases sha256 is enough to protect from ddos attacks

**Tests and benchmarks is in test.rs and benches/pow_benchmark.rs files**

| Difficulty(leading zeros) | Average time to solve |
|---------------------------|-----------------------|
| 1                         | 69.346 µs             |
| 2                         | 16.921 ms             |
| 3                         | 3.3267 s              |
