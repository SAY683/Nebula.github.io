use crate::file::{FileOperation, FileOperations, LocalFileOperations};
use anyhow::Result;
use compact_str::CompactString;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;

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
pub trait SlimeNode: Sized {
    fn new() -> Result<Self>;
    //产生
    fn target(dir: &str, file: &str) -> Result<Vec<(PathBuf, RwLock<CompactString>)>> {
        return Ok(LocalFileOperations([FileOperations::Read([(
            CompactString::new(dir),
            vec![CompactString::new(file)],
        )])])
        .run()?);
    }
    type Data;
    //处理
    fn handle(&self) -> Result<Self::Data>;
}
