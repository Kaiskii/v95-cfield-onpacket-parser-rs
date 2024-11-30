# Straightforward v95 Mushroom Game SendOpcode Parser

## Usage

```bash
cargo run -- --help
```

## Example

```bash
cargo run -- --opcode 0x3e
```

## Output

```bash
CWvsContext::OnPartyResult(this, iPacket)
```

## Build

```bash
cargo build --release
```

## Checksum

```bash
Get-FileHash target\release\v95-on-packet-parser.exe -Algorithm SHA256
```
