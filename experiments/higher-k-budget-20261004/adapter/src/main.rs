use std::io::{self,BufRead};
fn main(){for line in io::stdin().lock().lines(){let v=match line{Ok(s)=>higher_k_player::handle(&s,|_|{}),Err(e)=>serde_json::json!({"error":e.to_string()})};println!("{v}");}}
