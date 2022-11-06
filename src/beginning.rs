use crate::env::Environment;
use crate::node::{Master, Slave, SlimeNode};
use crate::{MASTER, SETTING, SETTING_UP, SLAVE};
use anyhow::Result;

///#初始化
pub async fn init() -> Result<()> {
	the_initial_data().await?;
	return Ok(());
}

///#数据初始
pub async fn the_initial_data() -> Result<()> {
	let x = Master::setting_xml(SETTING_UP.as_ref().unwrap())?;
	SETTING.get_or_init(|| x);
	let x = Master::new()?;
	MASTER.get_or_init(|| x);
	let x = Slave::new()?;
	SLAVE.get_or_init(|| x);
	return Ok(());
}
