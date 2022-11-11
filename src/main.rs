#![feature(
type_alias_impl_trait,
atomic_from_mut,
inline_const,
const_mut_refs,
associated_type_defaults,
array_zip,
box_syntax,
let_chains,
unboxed_closures,
async_closure,
type_ascription,
never_type,
impl_trait_in_fn_trait_return,
const_try
)]

mod beginning;
mod end;
pub mod env;
pub mod file;
pub mod mysql;
pub mod node;
pub mod redis;
mod run;
pub mod view;

pub use crate::env::Setting;
use crate::mysql::MysqlServer;
pub use crate::node::Slave;
use crate::redis::{RedisServer, RedisUlr};
pub use crate::view::GUI;
use crate::view::{Colour, Grade};
use deadpool_redis::redis::Client;
pub use env::Environment;
use lazy_static::lazy_static;
pub use node::Master;
use once_cell::sync::OnceCell;
use rbatis::Rbatis;
use std::net::UdpSocket;
use tokio::main;

#[main]
pub async fn main() -> anyhow::Result<()> {
	GUI::dispose("beginning::init()", beginning::init().await, true);
	GUI::dispose("run::start()", run::start().await, true);
	GUI::dispose("end::over()", end::over().await, true);
	return Ok(());
}
/*
运行时调用
 */
lazy_static! {
    //Redis驱动
    pub static ref REDIS_DRIVE:anyhow::Result<Client>=Ok(Master::redis(REDIS_ULR.get().unwrap())?);
    //Mysql驱动
    pub static ref MYSQL_DRIVE:anyhow::Result<Rbatis>=Ok(Master::orm(MYSQL_ULR.get().unwrap())?);
    //#本机ip
    pub static ref LOCAL_IP: anyhow::Result<String>={
        let x = UdpSocket::bind("0.0.0.0:0")?;
        x.connect("8.8.8.8:80")?;
        return Ok(x.local_addr()?.ip().to_string());
    };
}
//#设置
pub static MASTER: OnceCell<Master> = OnceCell::new();
pub static SLAVE: OnceCell<Slave> = OnceCell::new();
pub static REDIS_ULR: OnceCell<String> = OnceCell::new();
pub static MYSQL_ULR: OnceCell<String> = OnceCell::new();
pub static SETTING: OnceCell<Setting> = OnceCell::new();
//设置文件位置
pub static SETTING_UP: OnceCell<String> = OnceCell::new();
pub static PORT: OnceCell<String> = OnceCell::new();
pub static IP: OnceCell<String> = OnceCell::new();
pub static KEY: OnceCell<String> = OnceCell::new();
pub static CERT: OnceCell<String> = OnceCell::new();
pub static MYSQL: OnceCell<String> = OnceCell::new();
pub static REDIS: OnceCell<String> = OnceCell::new();
pub static NODE: OnceCell<String> = OnceCell::new();
pub static HTTP_HOME: OnceCell<String> = OnceCell::new();
pub static LOGS: OnceCell<String> = OnceCell::new();
pub static HDFS: OnceCell<String> = OnceCell::new();
pub static TRANSCRIPT: OnceCell<String> = OnceCell::new();

///#特殊类型
pub mod special_type {
	use deluge::Iter;
	use serde::{Deserialize, Serialize};
	use std::future::Future;
	use std::pin::Pin;
	use thiserror::Error;
	
	///#异步闭包[Future]
	///#pub struct AsyncDriver<'life, Rx: Sized>(
	///#pub Pin<Box<dyn Future<Output = anyhow::Result<Rx>> + Send + Sync + 'life>>,
	///#);
	pub struct AsyncDriver<'life, Rx: Sized>(
		pub Pin<Box<dyn Future<Output = anyhow::Result<Rx>> + Send + Sync + 'life>>,
	);
	
	///#异步脚本解析成为结构执行[serde] pub struct AsyncTheScript<'life, Rx: Sized + Serialize + Deserialize<'life>, Re: Sized + Serialize + Deserialize<'life>>(pub Box<dyn FnOnce(Rx) -> AsyncDriver<'life, Re> + Send + Sync + 'life>);
	pub struct AsyncTheScript<
		'life,
		Rx: Sized + Serialize + Deserialize<'life>,
		Re: Sized + Serialize + Deserialize<'life>,
	>(pub Box<dyn FnOnce(Rx) -> AsyncDriver<'life, Re> + Send + Sync + 'life>);
	
	///#异步迭代器[deluge]实现 pub struct AsynchronousIterator<G: Sized + IntoIterator>(Iter<G>);
	pub struct AsynchronousIterator<G: Sized + IntoIterator>(pub Iter<G>);
	
	///#通用异常
	#[derive(Debug, Error)]
	pub enum NebulaError {
		///过程异常
		#[error("ProcessException")]
		ProcessException,
		///数据异常
		#[error("DataException[IP:{name:?}|Error:{error:?}]")]
		DataException { name: String, error: String },
	}
}

///#数据表
pub mod data_table {
	use rbatis::{crud, impl_select, impl_update};
	use rbdc::datetime::FastDateTime;
	
	///#默认数据表
	#[derive(Hash, Clone, Debug, Serialize, Deserialize)]
	pub struct AeExam {
		//其他时间表接口 Option特殊情况不用写
		pub id: Option<String>,
		//分布式虚拟文件名称
		pub name: String,
		//hash文件验证值
		pub hash: Option<String>,
		//存储位置jsonNode
		pub location: Option<String>,
		//时间记录
		pub time: Option<FastDateTime>,
	}
	//依据实现
	crud!(AeExam {});
	//查询id
	impl_select!(AeExam {select_id(id:&str)=>"`where id = #{name}`"});
	//查询名称
	impl_select!(AeExam{select_name(name:&str)=>"`where name = #{name}`"});
	//基于id更新
	impl_update!(AeExam{update_id(id:&str)=>"`where id = #{id}`"});
	///#默认数据表格
	pub const AE_EXAM: &str = r"
create table if not exists ae_exam
(
	id varchar(1989) not null,
	name varchar(1989) not null,
	hash text null,
	location longtext null,
	time datetime null
)engine=InnoDB,charset=utf8mb4;
";
	
	///#数据库
	#[macro_use]
	pub mod database_strap;
	//#索引数据
	database_src! {
		indexes,
		id,i64,
		name,String
	}
	crud!(indexes{});
}
