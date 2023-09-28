# blur-lua-discord-rpc
Discord Rich Presence for Blur, but as a Lua plugin, but implemented in Rust, but loaded as a Lua module.


## Build

```shell
LUA_LIB_NAME=lib/lua5.1 LUA_LINK=cdylib cargo build --release --target=i686-pc-windows-msvc
cp "target/i686-pc-windows-msvc/release/blur_discord_rpc.dll" "<BLUR_DIR>/amax/plugins/discord_rpc/lib-blur_discord_rpc.dll"
```


## Lua plugin

`amax/plugins/discord_rpc/blur_discord_rpc.lua`:

```lua
local M = {
	lib = nil,
	client = nil,
	info = {
		name = "Dicord_RPC",
		description = "Discord Rich Presence Client for Blur",
		version = 0.1,
	},
}


function M.update_rpc(new_rpc)
	print_api("update_rpc(" .. new_rpc .. ")")
	M.client:update_rpc(new_rpc)
end


function M.update_rpc_pcall(new_rpc)
	if not M.client then
		return
	end
	local client = M.client
	print_api("update_rpc_pcall(" .. new_rpc .. ")")
	local f = function (x)
		client:update_rpc(x)
	end
	local ok, rs = pcall(f, name_next)
	if ok then
		print_api("updated RPC :)")
	else
		print_api("NOT OK :( error: " .. tostring(rs))
	end
end


-- This will be called when the plugin loads
function M.init()
	M.lib = require("discord_rpc.lib-blur_discord_rpc")
	local client = M.lib.start()
	if not client
		return false
	end
	M.client = client
	return true -- true means we loaded correctly
end


-- This will be called when the plugin unloads (game closes)
function M.free()
	return M.client.stop()
end

return M -- Return this plugin
```
