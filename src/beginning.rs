use crate::env::Environment;
use crate::mysql::MysqlUlr;
use crate::node::{Master, NodeService, Slave};
use crate::view::GUI;
use crate::{
	Colour, Grade, MysqlServer, RedisServer, RedisUlr, CERT, HDFS, HTTP_HOME, IP, KEY, LOGS,
	MASTER, MYSQL, MYSQL_ULR, NODE, PORT, REDIS, REDIS_DRIVE, REDIS_ULR, SETTING, SETTING_UP,
	SLAVE, TRANSCRIPT,
};
use anyhow::Result;
use async_backtrace::framed;
use crate::data_table::AE_EXAM;

///#初始化
#[framed]
pub async fn init() -> Result<()> {
	the_initial_data().await?;
	match SETTING.get().unwrap().heat_enabled {
		true => {
			check().await?;
			data_establish(AE_EXAM).await?;
		}
		false => {}
	}
	return Ok(());
}

///#全面检查
pub async fn check() -> Result<()> {
	let mut x = Master::conn(&Master::get_pool(MYSQL_ULR.get().unwrap())).await?;
	let (r1, r2, r3) = Master::ping(&mut x).await?;
	x.disconnect().await?;
	Master::ping_lot(REDIS_DRIVE.as_ref().unwrap())?;
	println!(
		"{}",
		*GUI::from((
			Colour::Output,
			Grade {
				explain: vec!["REDIS", "MYSQL"],
				output: vec![vec![
					format!("REDIS VERSION[OK]").as_str(),
					format!("MYSQL VERSION[{r1}.{r2}{r3}]").as_str(),
				]],
			}
		))
	);
	return Ok(());
}

///#数据建立
pub async fn data_establish(e: &str) -> Result<()> {
	Master::quote(e, MYSQL_ULR.get().unwrap()).await?;
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
