# Storage Invariants

This document defines the correctness contract for the single-node storage
engine. If any invariant below is broken, the database may return stale data,
lose data after restart, or rebuild an invalid keydir.

## Record Format

Each log record is encoded as:

```text
[crc32 | timestamp | key_size | value_size | key | value]
```

Field layout:

| Field | Size | Encoding | Meaning |
|---|---:|---|---|
| `crc32` | 4 bytes | little-endian `u32` | CRC-32/CKSUM of the record body |
| `timestamp` | 8 bytes | little-endian `u64` | Unix timestamp in seconds |
| `key_size` | 4 bytes | little-endian `u32` | number of key bytes |
| `value_size` | 4 bytes | little-endian `u32` | number of value bytes |
| `key` | `key_size` bytes | UTF-8 | key bytes |
| `value` | `value_size` bytes | UTF-8 | value bytes |

The record body is everything after `crc32`:

```text
[timestamp | key_size | value_size | key | value]
```

## Core Invariants

1. Every valid record's stored `crc32` must match the CRC-32/CKSUM of the
   record body.

2. `load()` must only apply a record to the in-memory keydir after the full
   record has been read and its CRC has been verified.

3. `load()` must never create, update, or remove a keydir entry from a corrupt
   or partial record.

4. After `load()`, the keydir must point to the newest valid non-tombstone
   record for each key.

5. `GET key` must read from the value offset stored in the keydir and return
   exactly `value_size` bytes from that offset.

6. If `SET key value` returns success and the record is fully written, then
   after restart `GET key` must return `value`.

7. If a key has no valid non-tombstone record after replay, `GET key` must
   return `NOT FOUND`.

## Tombstones

A delete is represented as a tombstone record:

```text
value_size = 0
value = empty bytes
```

Tombstone invariants:

1. A valid tombstone removes the key from the keydir during replay.

2. A tombstone must be CRC-verified before it removes anything from the keydir.

3. If the newest valid record for a key is a tombstone, the key must not exist
   after `load()`.

4. A corrupt or partial tombstone must not delete a previously valid value.

## Corruption And Truncation

The storage engine must handle invalid trailing data defensively.

Required behavior:

1. If `load()` reaches clean EOF between records, replay is complete.

2. If `load()` reaches EOF in the middle of a record header, key, or value, the
   partial record must be ignored.

3. If a record's CRC does not match, that record must be ignored.

4. No record after the first corrupt or partial record should be trusted unless
   the storage format later gains an explicit resynchronization mechanism.

5. Valid records before the first corrupt or partial record may still be used
   to rebuild the keydir.

## Keydir

The keydir is an in-memory index from key to the latest value location in the
append-only log.

For each keydir entry:

1. `value_pos` must point to the first byte of the value, not the record header.

2. `value_size` must equal the number of bytes that belong to the value.

3. The referenced record must be the newest valid non-tombstone record for that
   key.

4. Deleted keys must not have keydir entries after replay.

## Current Implementation Gaps

These are the next storage-correctness tasks:

1. `load()` currently applies tombstones before CRC verification. A corrupt
   tombstone should not be able to delete a valid value.

2. `load()` still uses `unwrap()` for parts of record replay. Truncated logs
   should not panic during startup.

3. CRC mismatch handling should be covered by tests for normal values and
   tombstones.

4. Truncated header, truncated key, and truncated value recovery should be
   covered by tests.

## Next Tests

Start with these tests before changing behavior:

1. A valid record survives restart and rebuilds the keydir.

2. A record with a corrupted CRC is ignored during `load()`.

3. A corrupt tombstone does not delete the previous valid value.

4. A log truncated in the middle of a header does not panic.

5. A log truncated in the middle of a value does not update the keydir.
