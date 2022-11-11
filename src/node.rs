use crate::file::{
	file_async::{FileAsynchronousOperation, FileAsynchronously},
	FileOperation, FileOperations, LocalFileOperations,
};
use anyhow::Result;
use async_trait::async_trait;
use compact_str::CompactString;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use tokio::net::UdpSocket;
use uuid::fmt::Urn;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Master {
	pub local: SocketAddr,
	pub hdfs: PathBuf,
	pub logs: PathBuf,
}

///#节点数据
#[derive(Debug, Serialize, Deserialize)]
pub struct Slave {
	//节点
	pub slave: Vec<Node>,
	//slave_hdfs统一配置
	pub hdfs: PathBuf,
	//slave守护节点
	pub guard: Node,
}

///#节点
#[derive(Debug, Serialize, Deserialize)]
pub struct Node {
	//名称
	pub name: String,
	//host
	pub host: String,
}

#[async_trait]
pub trait NodeService: Sized {
	async fn local_node() -> Result<String> {
		let x = UdpSocket::bind("0.0.0.0:0").await?;
		x.connect("8.8.8.8:80").await?;
		return Ok(x.local_addr()?.ip().to_string());
	}
	fn uid() -> String {
		return Urn::from_uuid(Uuid::new_v4()).to_string();
	}
	fn new() -> Result<Self>;
	//#读取产生
	fn target(dir: &str, file: &str) -> Result<Vec<(PathBuf, RwLock<CompactString>)>> {
		return Ok(LocalFileOperations([FileOperations::Read([(
			CompactString::new(dir),
			vec![CompactString::new(file)],
		)])])
			.run()?);
	}
	//#读取产生
	async fn async_target(
		dir: &str,
	) -> Result<<FileAsynchronousOperation<0> as FileAsynchronously>::Data> {
		return Ok(
			FileAsynchronousOperation::Read([(PathBuf::from(dir), vec![])])
				.file_async()
				.await?,
		);
	}
	type Data;
	//处理
	fn handle(&self) -> Result<Self::Data>;
}

pub mod master_node {
	use super::*;
	use crate::env::Environment;
	use crate::node::Master;
	use crate::view::GUI;
	use std::ops::{Deref, DerefMut};
	use deadpool_redis::redis::cmd;
	use hashbrown::{HashMap, HashSet};
	use crate::data_table::AeExam;
	use crate::{HDFS, IP, LOGS, MYSQL_DRIVE, PORT, REDIS_DRIVE};
	use crate::mysql::MysqlServer;
	use crate::redis::RedisServer;
	
	impl Environment for Master {}
	
	impl NodeService for Master {
		fn new() -> Result<Self> {
			return Ok(Master {
				local: format!(
					"{}:{}",
					IP.get().unwrap(),
					PORT.get().unwrap()
				)
					.parse()?,
				hdfs: PathBuf::from(HDFS.get().unwrap()),
				logs: PathBuf::from(LOGS.get().unwrap()),
			});
		}
		
		type Data = <Master as Deref>::Target;
		///#type Data = Lazy<fn()-><Master as Deref>::Target>;
		fn handle(&self) -> Result<Self::Data> {
			return Ok(self.local.to_string().parse::<SocketAddr>()?);
		}
	}
	
	impl Into<String> for Master {
		fn into(self) -> String {
			return self.local.to_string();
		}
	}
	
	impl Deref for Master {
		type Target = SocketAddr;
		fn deref(&self) -> &Self::Target {
			return &self.local;
		}
	}
	
	impl DerefMut for Master {
		fn deref_mut(&mut self) -> &mut Self::Target {
			return &mut self.local;
		}
	}
	
	impl<Rx: Sized> AsRef<Rx> for Master
		where
			<Master as Deref>::Target: AsRef<Rx>,
	{
		fn as_ref(&self) -> &Rx {
			return self.deref().as_ref();
		}
	}
	
	impl<Rx: Sized> AsMut<Rx> for Master
		where
			<Master as Deref>::Target: AsMut<Rx>,
	{
		fn as_mut(&mut self) -> &mut Rx {
			return self.deref_mut().as_mut();
		}
	}
	
	impl From<SocketAddr> for Master {
		///#首先文件否则默认
		fn from(value: SocketAddr) -> Self {
			let x = GUI::numerical_treatment("Master\r\nfrom", Master::new());
			return Master {
				local: value,
				hdfs: x.hdfs,
				logs: x.logs,
			};
		}
	}
	
	#[async_trait]
	impl RedisServer for Master {
		type Data = HashMap<String, Option<String>>;
		async fn redis_set(e: HashMap<String, String>) -> Result<()> {
			let mut z = REDIS_DRIVE.as_ref().unwrap().get_tokio_connection().await?;
			for (x, y) in e.into_iter() {
				cmd("SET").arg(x).arg(y).query_async(&mut z).await?;
			}
			return Ok(());
		}
		async fn redis_get(e: HashSet<String>) -> Result<Self::Data> {
			let mut z = REDIS_DRIVE.as_ref().unwrap().get_connection()?;
			let mut x = HashMap::new();
			for i in e.into_iter() {
				x.insert(i.as_str().to_string(), cmd("GET").arg(i).query::<Option<String>>(&mut z)?);
			}
			return Ok(x);
		}
		async fn redis_remove(e: HashSet<String>) -> Result<()> {
			let mut z = REDIS_DRIVE.as_ref().unwrap().get_tokio_connection().await?;
			for i in e.into_iter() {
				cmd("DEL").arg(i).query_async(&mut z).await?;
			}
			return Ok(());
		}
	}
	
	#[async_trait]
	impl MysqlServer for Master {
		async fn mysql_set(e: Self::Object) -> Result<()> {
			let mut x = MYSQL_DRIVE.as_ref().unwrap();
			let mut v: HashMap<String, String> = HashMap::new();
			for i in e.into_iter() {
				AeExam::insert(&mut x, &i).await?;
				v.insert(String::from(i.name), String::from(i.id.unwrap()));
			};
			Master::redis_set(v).await?;
			return Ok(());
		}
		
		async fn mysql_get_all() -> Result<Self::Object> {
			return Ok(AeExam::select_all(
				&mut MYSQL_DRIVE.as_ref().unwrap()
			).await?.into_iter().collect::<Vec<_>>());
		}
		async fn mysql_update(e: Vec<(AeExam, String)>) -> Result<()> {
			let mut i = MYSQL_DRIVE.as_ref().unwrap();
			//y=name
			for (x, y) in e.into_iter() {
				let r = Master::redis_get(HashSet::from([y.to_string()])).await?;
				let r = r.get(&y).unwrap().as_ref().unwrap();
				if x.name != r.as_str() {
					AeExam::update_id(&mut i, &x, r).await?;
					let _ = Master::redis_cmd::<String>("rename", HashSet::from([y, x.name]))?;
				} else {
					AeExam::update_id(&mut i, &x, r).await?;
				}
			}
			return Ok(());
		}
		async fn mysql_remove(e: HashSet<String>) -> Result<()> {
			let mut x = MYSQL_DRIVE.as_ref().unwrap();
			for i in e.into_iter() {
				let u = Master::redis_get(HashSet::from([i.to_string()])).await?;
				AeExam::delete_by_column(&mut x, "id", u.get(i.as_str()).unwrap().as_ref().unwrap()).await?;
				Master::redis_remove(HashSet::from([i.to_string()])).await?;
			}
			return Ok(());
		}
	}
}

pub mod node_manager {
	use anyhow::Result;
	use futures::executor::block_on;
	use crate::{Environment, NODE};
	use hashbrown::HashMap;
	use super::*;
	
	impl Environment for Slave {}
	
	impl NodeService for Slave {
		fn new() -> Result<Self> {
			let x = block_on(FileAsynchronousOperation::Read([(PathBuf::from(NODE.get().unwrap()), vec![])]).file_async())?;
			let (_, y) = x.get(0).unwrap();
			return Ok(serde_json::from_str(&*y.load().as_str())?);
		}
		type Data = HashMap<CompactString, CompactString>;
		///#type Data = HashMap::<CompactString,CompactString>;(name,host)
		///#fn handle(&self) -> Result<Self::Data>
		fn handle(&self) -> Result<Self::Data> {
			let mut r = hashbrown::HashMap::<CompactString, CompactString>::new();
			self.slave.iter().for_each(|x| {
				r.insert(CompactString::new(x.name.as_str()), CompactString::new(x.host.as_str()));
			});
			return Ok(r);
		}
	}
}
