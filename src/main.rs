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
pub mod env;
pub mod file;
pub mod node;
pub mod view;

use crate::env::Setting;
use crate::view::{Colour, Grade, GUI};
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use tokio::main;

#[main]
pub async fn main() -> anyhow::Result<()> {
    //初始化
    beginning::init().await.unwrap_or_else(|x| {
        eprintln!(
            "{}",
            *GUI::from((
                Colour::Error,
                Grade {
                    explain: vec!["beginning::init().await"],
                    output: vec![vec![format!("{:?}", x).as_str()]],
                },
            ))
        );
        panic!("{}", x)
    });
    return Ok(());
}
/*
运行时调用
 */
lazy_static! {}
//#设置
pub static SETTING: OnceCell<Setting> = OnceCell::new();
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
