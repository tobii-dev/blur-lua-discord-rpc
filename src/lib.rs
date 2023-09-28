use std::sync::mpsc::{Receiver, Sender};

use mlua::prelude::*;
use mlua::UserData;

struct LuaDiscordClient {
	tx: Sender<String>,
}

impl LuaDiscordClient {
	fn new() -> Self {
		let (tx, rx) = std::sync::mpsc::channel::<String>();
		std::thread::spawn(|| {
			client_thread(rx);
		});
		LuaDiscordClient { tx }
	}
}

impl UserData for LuaDiscordClient {
	// fn add_fields<'lua, F: LuaUserDataFields<'lua, Self>>(_fields: &mut F) {}

	fn add_methods<'lua, M: LuaUserDataMethods<'lua, Self>>(methods: &mut M) {
		//TODO: client:update_rpc()
		// - How to handle bad starts?
		methods.add_method("update_rpc", |_, this, state: String| -> LuaResult<()> {
			log::info!("update_rpc( {state} )");
			if this.tx.send(state).is_err() {
				log::info!("tx.send failed");
				return Ok(());
			}
			log::info!("sent update_rpc");
			Ok(())
		});

		//TODO: client:stop() method
		methods.add_method("stop", |_, this, _: ()| -> LuaResult<()> {
			this.tx.send("stfu".to_string()).unwrap();
			Ok(())
		});

		//TODO: client:is_online() method
		methods.add_method("is_online", |_, _this, _: ()| -> LuaResult<bool> {
			Ok(false)
		});
	}
}

//TODO: Move Discord stuff to rpc.rs
fn client_thread(rx: Receiver<String>) {
	use discord_rich_presence::{
		activity::{Activity, Assets},
		DiscordIpc, DiscordIpcClient,
	};
	const CLIENT_ID: u64 = 933390770453499974;
	let logo = Assets::new().large_image("logo").large_text("Blur");
	let act = Activity::default().assets(logo);

	/*
	details = sz_state;

	state = sz_playlist;

	smallImageKey = sz_small_img_key;
	smallImageText = sz_state; //TODO (maybe display better info about the playlist here)

	//largeImageKey = "logo";
	//largeImageText = "Blur"; //TODO (maybe display user:name here?)

	partyId = party_id; // TODO: suppport actual party stuff,
	partySize = party_size; //or, if user is not in a party, display lobby player numbers
	partyMax = party_max;

	dp.startTimestamp = start_time;
	*/

	let Ok(mut client) = DiscordIpcClient::new(&CLIENT_ID.to_string()) else {
		log::info!("Failed to DiscordIpcClient::new()");
		return;
	};
	log::info!("Created client");
	let mut connected = if client.connect().is_ok() {
		client
			.set_activity(act.clone().state("state").details("details"))
			.unwrap();
		true
	} else {
		log::info!("client.connect() failed");
		false
	};
	log::info!("Created activity...");
	while let Ok(state) = rx.recv() {
		log::info!("rx STATE: {state}");
		if !connected {
			log::info!("trying to reconnect");
			let r = client.connect();
			connected = r.is_ok();
		}
		if !connected {
			log::info!("failed to reconnect");
			continue;
		}

		if client.set_activity(act.clone().state(&state)).is_err() {
			log::info!("set_activity error");
			continue;
		}
		log::info!("updated rpc to {state}");
	}
}

#[mlua::lua_module]
fn blur_discord_rpc(lua: &Lua) -> LuaResult<LuaTable> {
	init_logs();

	let exports = lua.create_table()?;

	fn start(l: &Lua, _: ()) -> LuaResult<LuaDiscordClient> {
		let mem = l.used_memory();
		dbg!(mem);
		let client = LuaDiscordClient::new();
		Ok(client)
	}
	exports.set("start", lua.create_function(start)?)?;
	Ok(exports)
}

fn init_logs() {
	use log::LevelFilter;
	use simplelog::{
		ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
	};
	let cfg = ConfigBuilder::new()
		.set_time_offset_to_local()
		.unwrap()
		.build();
	let log_file = blur_plugins_core::create_log_file("blur_discord_rpc.log").unwrap();
	CombinedLogger::init(vec![
		TermLogger::new(
			LevelFilter::Trace,
			cfg,
			TerminalMode::Mixed,
			ColorChoice::Auto,
		),
		WriteLogger::new(LevelFilter::Trace, Config::default(), log_file),
	])
	.unwrap();
	log_panics::init();
}
