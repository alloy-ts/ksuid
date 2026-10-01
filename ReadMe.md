# @lib/ksuid

A native Node.js addon providing high-performance KSUID (K-Sortable Unique Identifier) implementation in Rust using `svix-ksuid` and `NAPI-RS`.

## Features

- **Standard 20-byte KSUID**: 32-bit (4-byte) timestamp in seconds + 16-byte payload.
- **Millisecond Resolution KSUID**: 48-bit (6-byte) or 64-bit (8-byte) timestamp in milliseconds (`KsuidMs`).
- **Configurable `timestampSize`**: Choose `"32bit"` (4 bytes), `"48bit"` (6 bytes), or `"64bit"` (8 bytes) timestamp size in constructor options.
- **Multiple Encodings**:
  - **Base62** (default standard 27-character KSUID string).
  - **Crockford Base32** (32-character string using Crockford alphabet).
  - **Base36** (32-character string for 160-bit or 25-character string for 128-bit data).
- **CrockfordBase32 Utility**: Custom 32-character alphabets and seed-based deterministic alphabet shuffling.
- **Base36 Utility**: Arbitrary byte array and 128-bit Base36 encoding/decoding.

## Installation

```bash
npm install @lib/ksuid
```

## Usage Examples

### Creating and Converting KSUIDs

```javascript
import { Ksuid, KsuidMs, CrockfordBase32, Base36 } from "@lib/ksuid";

// Generate a new KSUID with current timestamp
const ksuid = Ksuid.now();

// Base62 representation (27 characters)
console.log(ksuid.toBase62()); // "1srOrx2ZWZBpBUvZwXKQmoEYga2"
console.log(ksuid.toString()); // Same as toBase62() by default

// Raw 20 bytes Buffer
console.log(ksuid.bytes()); // Buffer of 20 bytes

// Raw payload Buffer
console.log(ksuid.payload()); // Buffer of 16 bytes

// Extract timestamp in seconds
console.log(ksuid.timestampSeconds()); // 1621627443
```

### Constructor Options & Encoding Config

```javascript
// Configure default encoding to Crockford Base32
const ksuidB32 = new Ksuid({ enc: "base32" });
console.log(ksuidB32.toString()); // 32-character Crockford Base32 string

// Configure custom shuffled Crockford alphabet
const shuffledAlph = CrockfordBase32.shuffleAlphabet("my-seed");
const ksuidCustom = new Ksuid({
  enc: "base32",
  alphabet: shuffledAlph,
  timestampSize: "48bit",
});
console.log(ksuidCustom.toString()); // Encoded using shuffled alphabet
```

### `timestampSize` Options

Supported values:

- `"32bit"` / `32` / `4`: 4-byte timestamp (seconds resolution, 16-byte payload).
- `"48bit"` / `48` / `6`: 6-byte timestamp (milliseconds resolution, 15-byte payload).
- `"64bit"` / `64` / `8`: 8-byte timestamp (milliseconds resolution, 15-byte payload).

```javascript
const k32 = new Ksuid({ timestampSize: "32bit" });
console.log(k32.timestampSize); // "32bit"
console.log(k32.payloadBytes); // 16

const k48 = new Ksuid({ timestampSize: "48bit" });
console.log(k48.timestampSize); // "48bit"
console.log(k48.payloadBytes); // 15
```

### CrockfordBase32 Utility

```javascript
// Default Crockford Base32 alphabet
console.log(CrockfordBase32.defaultAlphabet()); // "0123456789ABCDEFGHJKMNPQRSTVWXYZ"

// Deterministically shuffle alphabet using a seed
const shuffled = CrockfordBase32.shuffleAlphabet("seed123");

const cb32 = new CrockfordBase32(shuffled);
const encodedNum = cb32.encodeNumber(12345);
const decodedNum = cb32.decodeNumber(encodedNum); // 12345
```

### Base36 Utility

```javascript
// Encode and decode 128-bit (16-byte) data
const bytes128 = Buffer.alloc(16);
const encoded128 = Base36.encode128(bytes128); // 25 digits
const decoded128 = Base36.decode128(encoded128); // 16 bytes Buffer
```

## Comparison and Ordering

```javascript
const k1 = Ksuid.fromSeconds(1555555555);
const k2 = Ksuid.fromSeconds(1777777777);

console.log(k1.compare(k2)); // -1
console.log(k1.equals(k1)); // true
```

## Development

```bash
# Build native addon
npm run build

# Run unit tests
npm test

# Format code
npm run fmt

# Lint and check
npm run check
```
