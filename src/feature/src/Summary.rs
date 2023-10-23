pub trait summary {
    fn summarize(&self) ->String;
}

pub struct Post{
    pub tile:String,
    pub author:String,
    pub content:String,
}

impl summary for Post{
    fn summarize(&self) -> String {
        format!("文章{},作者是{}",self.tile,self.author)
    }
}

pub struct weibo{
    pub username:String,
    pub content:String,
}

impl summary for weibo{
    fn summarize(&self) -> String {
        format!("{}发表了微博{}",self.username,self.content)
    }
}

