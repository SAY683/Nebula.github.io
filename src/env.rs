use anyhow::Result;
use dotenv::dotenv;
use dotenv::var;
use quick_xml::de::from_str as from_str_serializer;
use serde::{Deserialize, Serialize};
use std::env::{current_dir, set_var};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};
///#环境
pub trait Environment<G = Setting>
where
    for<'life> G: Serialize + Sized + Deserialize<'life>,
{
    ///#返回设置
    ///#fn setting(e: &str, f: &mut String) -> Result<G>
    fn setting_xml(e: &str) -> Result<G> {
        let mut f = String::new();
        BufReader::new(File::open(e)?).read_to_string(&mut f)?;
        return Ok(from_str_serializer(&f)?);
    }
    //#环境变量读取 fn local_var(e: &str) -> Result<String>
    fn local_var(e: &str) -> Result<String> {
        dotenv().ok();
        return Ok(var(e)?);
    }
    //#本机数据 fn local_data() -> SystemData
    fn local_data() -> SystemData {
        return SystemData::default();
    }
    //#环境路径 fn environment_variable()
    fn environment_variable() -> Result<PathBuf> {
        return Ok(current_dir()?);
    }
    //#添加环境变量 fn add_variable(k: &str, y: &str)
    fn add_variable(k: &str, y: &str) {
        set_var(k, y);
    }
    //#文件查询 fn file_query(j: &str, i: usize) -> Vec<DirEntry>
    fn file_query(j: &str, i: usize) -> Vec<DirEntry> {
        return WalkDir::new(j)
            .min_depth(1)
            .max_depth(i)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
            .collect::<Vec<_>>();
    }
}
///#系统设置
#[derive(Debug, Serialize, Deserialize)]
pub struct Setting {
    //日志
    pub logs: bool,
    //集群
    pub colony: bool,
    //副本
    pub backups: i64,
    //热启动
    pub heat_enabled: bool,
	//web自驱动
	pub web_enabled:bool
}
///#系统数据
pub struct SystemData {
    ///#真实名称
    pub rename: String,
    ///#用户名称
    pub username: String,
    ///#用户语言选择
    pub lang: Vec<String>,
    ///#设备名称
    pub devices: String,
    ///#主机名称
    pub hosts: String,
    ///#平台
    pub platform: String,
    ///#系统名称
    pub system_name: String,
    ///#桌面环境
    pub desktop_environment: String,
}
impl Default for SystemData {
    fn default() -> Self {
        return SystemData {
            rename: whoami::realname(),
            username: whoami::username(),
            lang: whoami::lang().collect::<Vec<String>>(),
            devices: whoami::devicename(),
            hosts: whoami::hostname(),
            platform: whoami::platform().to_string(),
            system_name: whoami::distro(),
            desktop_environment: whoami::desktop_env().to_string(),
        };
    }
}
