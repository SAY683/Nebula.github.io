use crate::data_table::AeExam;
use crate::node::NodeService;
use crate::{MYSQL, RedisServer};
use anyhow::Result;
use async_trait::async_trait;
use futures::executor::block_on;
use hashbrown::{HashMap, HashSet};
use mysql_async::prelude::{Query, Queryable};
use mysql_async::{Conn as AsyncConn, Pool as AsyncPool};
use rbatis::Rbatis;
use rbdc_mysql::driver::MysqlDriver;
use serde::{Deserialize, Serialize};

///#Mysql_Ulr
#[derive(Debug, Serialize, Deserialize)]
pub struct MysqlUlr {
	pub name: String,
	pub password: String,
	pub host: String,
	pub database: String,
}

impl NodeService for MysqlUlr {
	fn new() -> Result<Self> {
		let x = block_on(MysqlUlr::async_target(MYSQL.get().unwrap()))?;
		let x = &*x.get(0).unwrap();
		return Ok(serde_json::from_str(x.1.load().as_str())?);
	}
	type Data = String;
	fn handle(&self) -> anyhow::Result<Self::Data> {
		return Ok(format!(
			"mysql://{}:{}@{}/{}",
			self.name.as_str(),
			self.password.as_str(),
			self.host.as_str(),
			self.database
		));
	}
}

#[async_trait]
pub trait MysqlServer<Gx = AeExam>: RedisServer {
	///#async_mysql get fn get(e: &str) -> AsyncPool
	fn get_pool(e: &str) -> AsyncPool {
		return AsyncPool::new(e);
	}
	///#async_mysql pool async fn conn(e: &AsyncPool) -> Result<Conn>
	async fn conn(e: &AsyncPool) -> Result<AsyncConn> {
		return Ok(e.get_conn().await?);
	}
	///#AsyncPool move AsyncPool to disconnect<断开>
	///#async_mysql pool async fn async fn quote(e: &str, r: ulr: &str) -> Result<()>
	async fn quote(e: &str, ulr: &str) -> Result<()> {
		let mut r = AsyncPool::new(ulr).get_conn().await?;
		e.ignore(&mut r).await?;
		r.disconnect().await?;
		return Ok(());
	}
	///#async_mysql ping async fn ping(r:&mut AsyncConn) -> Result<(u16, u16, u16)>
	async fn ping(r: &mut AsyncConn) -> Result<(u16, u16, u16)> {
		r.ping().await?;
		return Ok(r.server_version());
	}
	///#生成orm_get async fn orm_get(e: &str) -> Result<Rbatis>
	fn orm(e: &str) -> Result<Rbatis> {
		let rb = Rbatis::new();
		rb.init(MysqlDriver {}, e)?;
		return Ok(rb);
	}
	type Object = Vec<Gx>;
	///#async fn mysql_set(_: Self::Object) -> Result<()>;
	async fn mysql_set(_: Self::Object) -> Result<HashMap<<Self as RedisServer>::GX, <Self as RedisServer>::GX>>;
	///#async fn mysql_get_all() -> Result<Self::Object>;
	async fn mysql_get_all() -> Result<Self::Object>;
	///#async fn mysql_update(_: HashSet<(Gx, String)>) -> Result<Option<Gx>>;
	async fn mysql_update(_: Vec<(Gx, String)>) -> Result<Option<Gx>>;
	///#async fn mysql_remove(_: HashSet<String>) -> Result<()>;
	async fn mysql_remove(_: HashSet<String>) -> Result<()>;
}
