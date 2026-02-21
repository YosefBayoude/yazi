use mlua::{ExternalError, FromLua, IntoLua, Lua, Value};
use yazi_shared::{SStr, event::CmdCow};

#[derive(Debug)]
pub struct FilterCmdOpt {
	pub cmd: SStr,
}

impl From<CmdCow> for FilterCmdOpt {
	fn from(mut c: CmdCow) -> Self { Self { cmd: c.take_first().unwrap_or_default() } }
}

impl FromLua for FilterCmdOpt {
	fn from_lua(_: Value, _: &Lua) -> mlua::Result<Self> { Err("unsupported".into_lua_err()) }
}

impl IntoLua for FilterCmdOpt {
	fn into_lua(self, _: &Lua) -> mlua::Result<Value> { Err("unsupported".into_lua_err()) }
}
