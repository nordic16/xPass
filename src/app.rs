
pub struct Login<'a> {
    username: &'a str,
    password: &'a str,   
}


impl<'a> Login<'a> {
    pub fn new(username: &'a str, password: &'a str) -> Result<Self, &'a str> {
        if !username.is_empty() && !password.is_empty() {
            return Ok(Login { username, password});
        }
        
        Err("Invalid username or password xD")
    }
}


pub struct Logins<'a> {
    pub logins: Vec<Login<'a>>,
}

impl<'a> Logins<'a> {
    pub fn new() -> Self {
        Logins { logins: Vec::<Login<'a>>::new() }
    
        // deserialize logins later.
    }
}