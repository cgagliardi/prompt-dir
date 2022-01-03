use std::env;

fn main() {
  let home = get_env_var("HOME");
  let pwd = get_env_var("PWD");

  let home_replaced = replace_home(&home, pwd);
  let final_path = to_last_slashes(&home_replaced, 4);

  println!("{}", final_path)
}

fn get_env_var(name: &str) -> String {
  env::var(name).unwrap_or_else(|_| panic!("Cannot find env variable {}", name))
}

fn replace_home(home: &str, pwd: String) -> String {
  if let Some(home_removed) = pwd.strip_prefix(home) {
    return "~".to_owned() + home_removed;
  }
  pwd
}

fn to_last_slashes(str: &str, n: usize) -> &str {
  let slashes = str.rmatch_indices('/');
  let mut i: usize = 0;
  for (slash_i, _) in slashes {
    i += 1;
    if i == n {
      return &str[(slash_i + 1)..];
    }
  }
  str
}
