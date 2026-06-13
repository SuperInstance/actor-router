# Actor Router

An **actor router** directs incoming messages to one of several routees (actor references) based on a routing logic — broadcast, random, consistent-hash, or scatter-gather.

## Why It Matters

Routers are the primary scaling mechanism in actor systems. They abstract away the fan-out pattern so callers send one message and the router handles distribution. Used heavily in Akka/Pekko for horizontal scaling.

## How It Works

Implements multiple routing strategies with configurable route tables. Each strategy has different latency, fairness, and ordering guarantees. The router itself is an actor, enabling hierarchical routing trees.

## Usage

```toml
[dependencies]
actor-router = "0.1.0"
```

```rust
use actor_router;

// See examples/ directory for detailed usage
```

## API

- `RoutingStrategy` (lib.rs)
- `Router` (lib.rs)

## Architecture

This crate is part of the **[SuperInstance](https://github.com/SuperInstance)** ecosystem — a conservation-law-based framework for fleet coordination, ternary computation, and distributed agent systems.

### Related Crates

- [`superinstance-core`](https://github.com/SuperInstance/superinstance-core) — Core conservation law (γ + η = C)
- [`superinstance-harness`](https://github.com/SuperInstance/superinstance-harness) — Build harness and self-improving loop
- [`fleet-coordinator`](https://github.com/SuperInstance/fleet-coordinator) — Fleet-level coordination

## References

- [SuperInstance Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md)
- [Conservation Law Paper](https://github.com/SuperInstance/SuperInstance/blob/main/docs/conservation-law.md)

## License

MIT
