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
never_type
)]

mod beginning;
mod end;
pub mod env;
pub mod file;
pub mod node;
mod run;
pub mod view;

pub use crate::env::Setting;
pub use crate::view::GUI;
pub use env::Environment;
use lazy_static::lazy_static;
pub use node::Master;
use once_cell::sync::OnceCell;
use tokio::main;
pub use crate::node::Slave;

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
    //#env设置
    pub static ref SETTING_UP: anyhow::Result<String> = Ok(Master::local_var("SETTING_UP")?);
    pub static ref PORT: anyhow::Result<String> = Ok(Master::local_var("PORT")?);
    pub static ref IP: anyhow::Result<String> = Ok(Master::local_var("IP")?);
    pub static ref KEY: anyhow::Result<String> = Ok(Master::local_var("KEY")?);
    pub static ref CERT: anyhow::Result<String> = Ok(Master::local_var("CERT")?);
    pub static ref MYSQL: anyhow::Result<String> = Ok(Master::local_var("MYSQL")?);
    pub static ref REDIS: anyhow::Result<String> = Ok(Master::local_var("REDIS")?);
    pub static ref NODE: anyhow::Result<String> = Ok(Master::local_var("NODE")?);
    pub static ref HTTP_HOME: anyhow::Result<String> = Ok(Master::local_var("HTTP_HOME")?);
    pub static ref LOGS: anyhow::Result<String> = Ok(Master::local_var("LOGS")?);
    pub static ref HDFS: anyhow::Result<String> = Ok(Master::local_var("HDFS")?);
    pub static ref TRANSCRIPT: anyhow::Result<String> = Ok(Master::local_var("TRANSCRIPT")?);
}
//#设置
pub static SETTING: OnceCell<Setting> = OnceCell::new();
pub static MASTER: OnceCell<Master> = OnceCell::new();
pub static SLAVE: OnceCell<Slave> = OnceCell::new();

///#特殊类型
pub mod special_type {
	use deluge::Iter;
	use serde::{Deserialize, Serialize};
	use std::future::Future;
	use std::pin::Pin;
	
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
}
