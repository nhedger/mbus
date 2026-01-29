# M-Bus Frame

`mbus-frame` defines link-layer frame structs for M-Bus (EN 13757-2).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
mbus-frame = "0.1"
```

## Usage

The following example demonstrates how to create different types of M-Bus
frames.

```rust
use mbus_frame::{AckFrame, LongFrame, NackFrame, ShortFrame};

fn example() {
    let ack = AckFrame::new();
    let nack = NackFrame::new();
    let short = ShortFrame::new(0x40, 0x01);
    let long = LongFrame::new(0x44, 0x01, vec![0x01, 0x02, 0x03]);
}
```

## Byte layouts

This section describes the byte layouts of the different M-Bus link-layer frames.

### ShortFrame

Byte layout (5 bytes total)

| Offset | Field        | Size   | Description                               |
|--------|--------------|--------|-------------------------------------------|
| 0      | `start_byte` | 1 byte | Start byte (`0x10`).                      |
| 1      | `control`    | 1 byte | Control byte for the link layer.          |
| 2      | `address`    | 1 byte | Secondary address.                        |
| 3      | `checksum`   | 1 byte | `control + address` (wrapping 8-bit sum). |
| 4      | `stop_byte`  | 1 byte | Stop byte (`0x16`).                       |

### LongFrame

Byte layout (8 + N bytes total)

| Offset | Field               | Size    | Description                                              |
|--------|---------------------|---------|----------------------------------------------------------|
| 0      | `start_byte`        | 1 byte  | Start byte (`0x68`).                                     |
| 1      | `length`            | 1 byte  | `2 + payload.len()` (control + address + payload).       |
| 2      | `length_repeat`     | 1 byte  | Repeated length (`length`).                              |
| 3      | `start_byte_repeat` | 1 byte  | Repeated start byte (`0x68`).                            |
| 4      | `control`           | 1 byte  | Control byte for the link layer.                         |
| 5      | `address`           | 1 byte  | Secondary address.                                       |
| 6..    | `payload`           | N bytes | Application payload (opaque bytes).                      |
| 6+N    | `checksum`          | 1 byte  | `control + address + sum(payload)` (wrapping 8-bit sum). |
| 7+N    | `stop_byte`         | 1 byte  | Stop byte (`0x16`).                                      |

### AckFrame

Byte layout (1 byte total)

| Offset | Field      | Size   | Description                    |
|--------|------------|--------|--------------------------------|
| 0      | `ACK_BYTE` | 1 byte | Acknowledgement byte (`0xE5`). |

### NackFrame

Byte layout (1 byte total)

| Offset | Field       | Size   | Description                             |
|--------|-------------|--------|-----------------------------------------|
| 0      | `NACK_BYTE` | 1 byte | Negative acknowledgement byte (`0xA2`). |

## Scope

This crate provides data structures representing M-Bus link-layer frames,
but intentionally excludes application-layer fields and parsing logic.