use serde::{Serialize, Deserialize};
use async_trait::async_trait;
use deadpool_redis::redis::{Client, ConnectionLike};
use deadpool_redis::{Connection, Pool};
use anyhow::Result;
use futures::executor::block_on;
use hashbrown::{HashSet,HashMap};
use crate::node::NodeService;
use crate::REDIS;

///#Redis_Ulr
#[derive(Debug, Serialize, Deserialize)]
pub struct RedisUlr {
	pub name: Option<String>,
	pub password: Option<String>,
	pub ip: String,
	pub port: String,
	pub database: String,
}

impl NodeService for RedisUlr {
	fn new() -> Result<Self> {
		let x = block_on(RedisUlr::async_target(REDIS.get().unwrap()))?;
		let x = &*x.get(0).unwrap();
		return Ok(serde_json::from_str(x.1.load().as_str())?);
	}
	type Data = String;
	///#产生
	///#redis://[<username>][:<password>@]<hostname>[:port][/<db>]
	fn handle(&self) -> anyhow::Result<Self::Data> {
		Ok(if self.name.is_some() || self.password.is_some() {
			format!(
				"redis://{}:{}@{}:{}/{}",
				self.name.as_ref().unwrap().as_str(),
				self.password.as_ref().unwrap().as_str(),
				self.ip.as_str(),
				self.port.as_str(),
				self.database.as_str()
			)
		} else {
			format!("redis://{}:{}", self.ip.as_str(), self.port.as_str())
		})
	}
}

///#redis服务
#[async_trait]
pub trait RedisServer<Gx = String> {
	///#redis(e: &str) -> Result<Client>
	fn redis(e: &str) -> Result<Client> {
		return Ok(Client::open(e)?);
	}
	///#fn ping_lot(e: &Client) -> Result<bool>
	fn ping_lot(e: &Client) -> Result<bool> {
		return Ok(e.get_connection()?.is_open());
	}
	///#async fn redis_connection_async(e: Pool) -> Result<Connection>
	async fn redis_connection_async(e: Pool) -> Result<Connection> {
		return Ok(e.get().await?);
	}
	type Data = Vec<Gx>;
	///#async fn redis_set(_: &Gx) -> Result<Self::Data>;
	async fn redis_set(_: HashMap<Gx, Gx>) -> Result<Option<Self::Data>>;
	///#async fn get_redis_get(_: &Gx)->Result<Self::Data>;
	async fn redis_get(_: HashSet<Gx>) -> Result<Option<Self::Data>>;
	///#async fn get_redis_remove(_: &Gx) -> Result<()>;
	async fn redis_remove(_: HashSet<Gx>) -> Result<()>;
}