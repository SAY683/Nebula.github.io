use crate::env::Environment;
use crate::node::{Master, NodeService, Slave};
use crate::{CERT, HDFS, HTTP_HOME, IP, KEY, LOGS, MASTER, MYSQL, MYSQL_ULR, NODE, PORT, REDIS, REDIS_DRIVE, REDIS_ULR, RedisServer, RedisUlr, SETTING, SETTING_UP, SLAVE, TRANSCRIPT};
use anyhow::Result;
use crate::mysql::MysqlUlr;
use async_backtrace::framed;

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
	if Master::ping_lot(REDIS_DRIVE.as_ref().unwrap())? {}
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