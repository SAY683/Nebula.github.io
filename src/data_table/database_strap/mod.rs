///#[不安全数据]
#[macro_export]
macro_rules! nothing {
    ($a:expr)=>{
        ManuallyDrop::new($a)
    }
}
///#数据数据表
#[macro_export]
macro_rules! database_src {
    ($name:tt,$fie:ident,$typ:ty)=>{
        use serde::{Serialize,Deserialize};
        #[derive(Debug,Hash,Clone,Serialize, Deserialize)]
        pub struct $name{
            pub $fie:$typ
        }
    };
    ($name:tt,$($fie:ident,$typ:ty),*)=>{
        use serde::{Serialize,Deserialize};
        #[derive(Debug,Hash,Clone,Serialize, Deserialize)]
        pub struct $name{
            pub $($fie:$typ),*
        }
    }
}