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
	async fn local_node()->Result<String>{
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
	use hashbrown::{HashMap, HashSet};
	use crate::data_table::AeExam;
	use crate::{HDFS, IP, LOGS, PORT};
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
		async fn redis_set(_: HashMap<String, String>) -> Result<Option<Self::Data>> {
			todo!()
		}
		async fn redis_get(_: HashSet<String>) -> Result<Option<Self::Data>> {
			todo!()
		}
		async fn redis_remove(_: HashSet<String>) -> Result<()> {
			todo!()
		}
	}
	#[async_trait]
	impl MysqlServer for Master {
		async fn mysql_set(_: Vec<Self::Object>) -> Result<()> {
			todo!()
		}
		async fn mysql_get_all() -> Result<Option<Self::Object>> {
			todo!()
		}
		async fn mysql_update(_: Vec<(AeExam, String)>) -> Result<Option<AeExam>> {
			todo!()
		}
		async fn mysql_remove(_: HashSet<String>) -> Result<()> {
			todo!()
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
