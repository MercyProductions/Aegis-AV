# Signed Update Architecture

## Required Checks

1. Fetch manifest over TLS.
2. Verify the manifest signature against a pinned Aegis public key.
3. Reject manifests with a rollback counter lower than installed state.
4. Download the package to a staging folder.
5. Verify package SHA256 against the signed manifest.
6. Verify package signature for signature packs and application updates.
7. Apply atomically with a rollback copy.
8. Record version, channel, package hash, and rollback counter.

## Release Channels

- `stable`: default user channel.
- `beta`: opt-in preview channel.
- `dev`: local development channel only.

## Non-Goals

- Do not run unsigned executable updates automatically.
- Do not silently downgrade signature packs.
- Do not fetch update instructions from remote-control style endpoints.
