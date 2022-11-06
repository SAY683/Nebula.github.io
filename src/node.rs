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
pub trait SlimeNode: Sized {
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
    use std::ops::{Deref, DerefMut};
    impl Environment for Master {}
    impl SlimeNode for Master {
        fn new() -> Result<Self> {
            return Ok(Master {
                local: format!(
                    "{}:{}",
                    Master::local_var("IP")?,
                    Master::local_var("PORT")?
                )
                .parse()?,
                hdfs: PathBuf::from(Master::local_var("HDFS")?),
                logs: PathBuf::from(Master::local_var("LOGS")?),
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
            let x = Master::new().unwrap();
            return Master {
                local: value,
                hdfs: x.hdfs,
                logs: x.logs,
            };
        }
    }
}
