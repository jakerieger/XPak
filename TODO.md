# TODO

## For the build cache:

The cache itself will just be a YAML file containing a list of asset names and SHA-256 checksum values. When building,
XPak will check if
the checksum of an asset source listed in the manifest matches an existing checksum in the cache file. If so, the build
process skips the asset. If it doesn't match, the checksum is updated and the asset rebuilt.

### Cache File Example

```yaml
local_cache:
  - source: sprites/idle.png
    checksum: a4924ee9d693d2518eeedb0ccb5d5f97bcf660c7960b9a89344577534180e4dd
  - source: sprites/blink.png
    checksum: 764f3991bdebf32082fda9af28784cd259815fabf853515df41d294dc0f21b59
```