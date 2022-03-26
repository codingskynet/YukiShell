use ysh_lib::ysh_main;

fn echo(text: String) -> Result<String, ()> {
    Ok(text)
}

ysh_main!();
