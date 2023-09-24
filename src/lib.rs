use std::{fs::File, io::Write, path::PathBuf};

use mlua::prelude::*;

fn init(l: &Lua, _: ()) -> LuaResult<bool> {
	let log_file = PathBuf::from(".")
		.join("amax")
		.join("log")
		.join("blur_lua_discord_rpc.log");
	let Ok(mut f) = File::create(log_file) else {
		return Ok(false);
	};
	let x = l.used_memory();
	let Ok(_) = write!(
		f,
		"Hi from Blur_Discord_Rich_Presence_Client! Used memory: {x} bytes."
	) else {
		return Ok(false);
	};
	Ok(true)
}

#[mlua::lua_module]
fn blur_discord_rpc(lua: &Lua) -> LuaResult<LuaTable> {
	let exports = lua.create_table()?;

	let info = lua.create_table()?;
	info.set("name", "Blur_Discord_Rich_Presence_Client")?;
	info.set("version", 0.1)?;
	exports.set("info", info)?;

	exports.set("init", lua.create_function(init)?)?;
	Ok(exports)
}
