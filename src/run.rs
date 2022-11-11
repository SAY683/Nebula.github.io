use anyhow::Result;
use async_backtrace::framed;
use crate::{HTTP_HOME, SETTING};

///#运行
#[framed]
pub async fn start() -> Result<()> {
	match SETTING.get().unwrap().web_enabled {
		true => {
			opener::open(HTTP_HOME.get().unwrap())?;
		}
		false => {}
	}
	return Ok(());
}