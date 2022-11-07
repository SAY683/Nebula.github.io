use crate::env::Environment;
use crate::node::{Master, NodeService, Slave};
use crate::{CERT, Colour, Grade, HDFS, HTTP_HOME, IP, KEY, LOGS, MASTER, MYSQL, MYSQL_ULR, MysqlServer, NODE, PORT, REDIS, REDIS_DRIVE, REDIS_ULR, RedisServer, RedisUlr, SETTING, SETTING_UP, SLAVE, TRANSCRIPT};
use anyhow::Result;
use crate::mysql::MysqlUlr;
use async_backtrace::framed;
use crate::view::GUI;

///#初始化
#[framed]
pub async fn init() -> Result<()> {
	the_initial_data().await?;
	if SETTING.get().unwrap().heat_enabled {
		check().await?;
	} else {}
	return Ok(());
}

///#全面检查
pub async fn check() -> Result<()> {
	let mut x = Master::conn(&Master::get_pool(MYSQL_ULR.get().unwrap())).await?;
	let r = Master::ping(&mut x).await?;
	x.disconnect().await?;
	println!("{}", *GUI::from((Colour::Output, Grade {
		explain: vec!["REDIS", "MYSQL"],
		output: vec![
			vec![format!("REDIS VERSION[{}]", Master::ping_lot(REDIS_DRIVE.as_ref().unwrap())?).as_str(),
			     format!("MYSQL VERSION[{}.{}{}]", r.0, r.1, r.2).as_str()]
		],
	})));
	return Ok(());
}

///#数据初始
pub async fn the_initial_data() -> Result<()> {
	//#++++++++++++++++++++++++++++设置文件
	let x = Master::local_var("SETTING_UP")?;
	SETTING_UP.get_or_init(|| x);
	let x = Master::local_var("PORT")?;
	PORT.get_or_init(|| x);
	let x = Master::local_var("IP")?;
	IP.get_or_init(|| x);
	let x = Master::local_var("KEY")?;
	KEY.get_or_init(|| x);
	let x = Master::local_var("CERT")?;
	CERT.get_or_init(|| x);
	let x = Master::local_var("MYSQL")?;
	MYSQL.get_or_init(|| x);
	let x = Master::local_var("REDIS")?;
	REDIS.get_or_init(|| x);
	let x = Master::local_var("NODE")?;
	NODE.get_or_init(|| x);
	let x = Master::local_var("HTTP_HOME")?;
	HTTP_HOME.get_or_init(|| x);
	let x = Master::local_var("LOGS")?;
	LOGS.get_or_init(|| x);
	let x = Master::local_var("HDFS")?;
	HDFS.get_or_init(|| x);
	let x = Master::local_var("TRANSCRIPT")?;
	TRANSCRIPT.get_or_init(|| x);
	//+++++++++++++++++++++++++++++++++++++++++++++++
	let x = Master::setting_xml(SETTING_UP.get().unwrap())?;
	SETTING.get_or_init(|| x);
	let x = Master::new()?;
	MASTER.get_or_init(|| x);
	let x = Slave::new()?;
	SLAVE.get_or_init(|| x);
	let x = RedisUlr::new()?.handle()?;
	REDIS_ULR.get_or_init(|| x);
	let x = MysqlUlr::new()?.handle()?;
	MYSQL_ULR.get_or_init(|| x);
	return Ok(());
}