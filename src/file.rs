use anyhow::Result;
use compact_str::CompactString;
use parking_lot::RwLock;
use std::fs::{create_dir_all, remove_dir_all, remove_file, File, OpenOptions};
use std::io::{BufReader, Read, Write};
use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

///#本机文件操作
///#pub struct LocalFileOperations<const RX: usize, const RS: usize>(pub FileOperations<RS>; RX);
pub struct LocalFileOperations<const RX: usize, const RS: usize>(pub [FileOperations<RS>; RX]);
///#文件操作|路径/文件(./SF,8964.txt)
pub enum FileOperations<const RX: usize> {
    //创建
    Establish([(CompactString, Vec<CompactString>); RX]),
    //删除
    Delete {
        file: [(CompactString, Vec<CompactString>); RX],
        //连目录删除
        better_to_delete: bool,
    },
    //读取
    Read([(CompactString, Vec<CompactString>); RX]),
    //写入
    Write {
        file: [(CompactString, Vec<CompactString>); RX],
        //覆盖
        coverage: bool,
        //内容
        content: CompactString,
    },
}

impl<const RX: usize, const RS: usize> From<[FileOperations<RS>; RX]>
    for LocalFileOperations<RX, RS>
{
    fn from(value: [FileOperations<RS>; RX]) -> Self {
        return LocalFileOperations(value);
    }
}
pub trait FileOperation {
    type Data;
    //运行
    fn run(self) -> Result<Self::Data>;
}
impl<const RX: usize, const RS: usize> FileOperation for LocalFileOperations<RX, RS> {
    type Data = Vec<(PathBuf, RwLock<CompactString>)>;
    ///#type Data = Vec<(PathBuf, RwLock<CompactString>)>;
    ///#fn run(self) -> Result<Self::Data>
    ///#错误去除
    fn run(self) -> Result<Self::Data> {
        let mut r = vec![];
        self.0
            .map(|x| -> Result<<FileOperations<RS> as FileOperation>::Data> { Ok(x.run()?) })
            .into_iter()
            .map(|x| -> <FileOperations<RS> as FileOperation>::Data {
                x.unwrap_or_else(|b| {
                    eprintln!("IO Error{:?}", b);
                    <FileOperations<RS> as FileOperation>::Data::default()
                })
            })
            .for_each(|mut x| r.append(&mut x));
        return Ok(r);
    }
}

impl<const RX: usize> FileOperation for FileOperations<RX> {
    type Data = Vec<(PathBuf, RwLock<CompactString>)>;
    ///type Data = Vec<(PathBuf, RwLock<CompactString>)>;
    ///fn run(self) -> Result<Self::Data>
    fn run(self) -> Result<Self::Data> {
        let mut v: Vec<(PathBuf, RwLock<CompactString>)> = Vec::new();
        match self {
            FileOperations::Establish(x) => {
                for i in x.map(|(x, y)| -> Result<()> {
                    create_dir_all(x.as_str())?;
                    for x in y.into_iter().map(|y| -> Result<()> {
                        File::create([x.as_str(), y.as_str()].iter().collect::<PathBuf>())?;
                        Ok(())
                    }) {
                        x?
                    }
                    Ok(())
                }) {
                    i?
                }
            }
            FileOperations::Delete {
                file,
                better_to_delete,
            } => {
                for i in file.map(|(x, y)| -> Result<()> {
                    if better_to_delete {
                        remove_dir_all(x.as_str())?;
                    } else {
                        for i in y.into_iter().map(|y| -> Result<()> {
                            remove_file(y.as_str())?;
                            Ok(())
                        }) {
                            i?
                        }
                    }
                    Ok(())
                }) {
                    i?
                }
            }
            FileOperations::Read(x) => {
                for i in x.map(|(x, y)| -> Result<()> {
                    for i in y.into_iter().map(|y| -> Result<()> {
                        let mut f = String::default();
                        let e = [x.as_str(), y.as_str()].iter().collect();
                        BufReader::new(File::open(&e)?).read_to_string(&mut f)?;
                        v.push((e, RwLock::new(CompactString::new(f))));
                        Ok(())
                    }) {
                        i?
                    }
                    Ok(())
                }) {
                    i?
                }
            }
            FileOperations::Write {
                file,
                coverage,
                content,
            } => {
                for i in file.map(|(x, y)| -> Result<()> {
                    for i in y.into_iter().map(|y| -> Result<()> {
                        let mut f = OpenOptions::new()
                            .write(true)
                            .create(true)
                            .append(coverage)
                            .open([x.as_str(), y.as_str()].iter().collect::<PathBuf>())?;
                        f.write_all(content.as_bytes())?;
                        f.flush()?;
                        Ok(())
                    }) {
                        i?
                    }
                    Ok(())
                }) {
                    i?
                }
            }
        }
        return Ok(v);
    }
}

impl<const RX: usize, const RS: usize> Deref for LocalFileOperations<RX, RS> {
    type Target = [FileOperations<RS>; RX];
    fn deref(&self) -> &Self::Target {
        return &self.0;
    }
}
impl<const RX: usize, const RS: usize> DerefMut for LocalFileOperations<RX, RS> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        return &mut self.0;
    }
}
///#异步版
pub mod file_async {
    use super::*;
    use crate::file::{FileOperations, LocalFileOperations};
    use crate::special_type::AsynchronousIterator;
    use arc_swap::ArcSwapAny;
    use async_trait::async_trait;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::vec::IntoIter;

    impl<const RX: usize, const RS: usize> IntoIterator for LocalFileOperations<RX, RS> {
        type Item = FileOperations<RS>;
        type IntoIter = IntoIter<Self::Item>;
        fn into_iter(self) -> Self::IntoIter {
            return self.0.into_iter().collect::<Vec<_>>().into_iter();
        }
    }
    impl<const RX: usize, const RS: usize> From<LocalFileOperations<RX, RS>>
        for AsynchronousIterator<IntoIter<FileOperations<RS>>>
    {
        fn from(value: LocalFileOperations<RX, RS>) -> Self {
            return AsynchronousIterator(deluge::iter(value.into_iter()));
        }
    }
    ///#异步操作
    ///#支持单参数 if (详细,Vec(0)) else (目录,文件)
    pub enum FileAsynchronousOperation<const RX: usize> {
        //创建
        Establish([(PathBuf, Vec<PathBuf>); RX]),
        //删除
        Delete {
            file: [(PathBuf, Vec<PathBuf>); RX],
            //连目录删除
            better_to_delete: bool,
        },
        //读取
        Read([(PathBuf, Vec<PathBuf>); RX]),
        //写入
        Write {
            file: [(PathBuf, Vec<PathBuf>); RX],
            //覆盖
            coverage: bool,
            //内容
            content: String,
        },
    }
    #[async_trait]
    pub trait FileAsynchronously {
        type Data = Vec<(PathBuf, ArcSwapAny<Arc<String>>)>;
        async fn file_async(self) -> Result<Self::Data>;
    }
    #[async_trait]
    impl<const RX: usize> FileAsynchronously for FileAsynchronousOperation<RX> {
        ///#type Data = Vec<(PathBuf, ArcSwapAny<Arc<String>>)>;
        ///#async fn file_async(self) -> Result<Self::Data>
        async fn file_async(self) -> Result<Self::Data> {
            return Ok(match self {
                FileAsynchronousOperation::Establish(s) => {
                    for s in s.map(|(x, y)| -> Result<()> {
                        if y.is_empty() {
                            File::create(x)?;
                        } else {
                            create_dir_all(&x)?;
                            for x in y.iter().map(|y| -> Result<()> {
                                File::create(
                                    [x.as_path(), y.as_path()].iter().collect::<PathBuf>(),
                                )?;
                                Ok(())
                            }) {
                                x?
                            }
                        }
                        return Ok(());
                    }) {
                        s?
                    }
                    Vec::<(PathBuf, ArcSwapAny<Arc<String>>)>::default()
                }
                FileAsynchronousOperation::Delete {
                    file,
                    better_to_delete,
                } => {
                    for x in file.map(|(x, y)| -> Result<()> {
                        if y.is_empty() {
                            if remove_dir_all(&x).is_err() {
                                remove_file(x)?;
                            }
                        } else {
                            if better_to_delete {
                                remove_dir_all(x)?;
                            } else {
                                for i in y.into_iter().map(|y| -> Result<()> {
                                    remove_file(
                                        [x.as_path(), y.as_path()].iter().collect::<PathBuf>(),
                                    )?;
                                    Ok(())
                                }) {
                                    i?
                                }
                            }
                        }
                        Ok(())
                    }) {
                        x?;
                    }
                    Vec::<(PathBuf, ArcSwapAny<Arc<String>>)>::default()
                }
                FileAsynchronousOperation::Read(x) => {
                    let mut r: Vec<(PathBuf, ArcSwapAny<Arc<String>>)> = Vec::new();
                    for x in x.map(|(x, y)| -> Result<()> {
                        if y.is_empty() {
                            let mut f = String::default();
                            BufReader::new(File::open(&x)?).read_to_string(&mut f)?;
                            r.push((x, ArcSwapAny::new(Arc::new(f))));
                        } else {
                            for i in y.into_iter().map(|y| -> Result<()> {
                                let mut f = String::default();
                                let e = [x.as_path(), y.as_path()].iter().collect::<PathBuf>();
                                BufReader::new(File::open(&e)?).read_to_string(&mut f)?;
                                r.push((e, ArcSwapAny::new(Arc::new(f))));
                                Ok(())
                            }) {
                                i?
                            }
                        }
                        Ok(())
                    }) {
                        x?
                    }
                    r
                }
                FileAsynchronousOperation::Write {
                    file,
                    coverage,
                    content,
                } => {
                    for i in file.map(|(x, y)| -> Result<()> {
                        if y.is_empty() {
                            let mut f = OpenOptions::new()
                                .write(true)
                                .create(true)
                                .append(coverage)
                                .open(x)?;
                            f.write_all(content.as_bytes())?;
                            f.flush()?;
                        } else {
                            for i in y.into_iter().map(|y| -> Result<()> {
                                let mut f = OpenOptions::new()
                                    .write(true)
                                    .create(true)
                                    .append(coverage)
                                    .open([x.as_path(), y.as_path()].iter().collect::<PathBuf>())?;
                                f.write_all(content.as_bytes())?;
                                f.flush()?;
                                Ok(())
                            }) {
                                i?
                            }
                        }
                        Ok(())
                    }) {
                        i?;
                    }
                    Vec::<(PathBuf, ArcSwapAny<Arc<String>>)>::default()
                }
            });
        }
    }
}
