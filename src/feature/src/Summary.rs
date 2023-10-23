pub trait summary {
    fn sumamarize(&self)->String;
}

pub struct Post{
    pub tile:String,
    pub author:String,
    pub content:String,
}

impl summary for Post{
    fn sumamarize(&self) -> String {
        format!("文章{},作者是{}",self.tile,self.author)
    }
}

pub struct weibo{
    pub username:String,
    pub content:String,
}

impl summary for weibo{
    fn sumamarize(&self) -> String {
        format!("{}发表了微博{}",self.username,self.content)
    }
}

