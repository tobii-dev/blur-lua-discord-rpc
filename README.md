# blur-lua-discord-rpc
Discord Rich Presence for Blur, but as a Lua plugin, but implemented in Rust, but loaded as a Lua module.

```nu
LUA_LIB_NAME=lib/lua5.1 LUA_LINK=cdylib cargo build --release --target=i686-pc-windows-msvc
cp "target/i686-pc-windows-msvc/release/blur_discord_rpc.dll" "<BLUR_DIR>/amax/plugins/discord_rpc/lib-blur_discord_rpc.dll"
```
