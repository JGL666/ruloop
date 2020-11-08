
#[derive(Default,Debug)]
pub struct Token(pub usize);
impl Token{
    pub fn new(n:usize)->Self{
        Token(n)
    }
}