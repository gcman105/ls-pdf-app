use std::{fs, env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    //let args: Vec<_> = env::args().collect();

    // We expect 1 argument (the path), return if not.
    if args.len() != 2 {
        println!("Usage: {} \"PDF folder path\"", args[0]);
        return;
    }

    // The first argument is the path that was used to call the program.
    println!("Program path is {}.", args[0]);

    let the_path: String = ::std::env::args().nth(1).unwrap();

    println!("The PATH argument given is:{:?}", the_path);

    let res = pdfs(the_path);

    println!("The result is:{:?}", res);
}


fn pdfs(path: String) -> io::Result<()> {

    let entries = fs::read_dir(&path)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;

    for entry in entries {
        println!("{:?}", entry);
    }

    Ok(())
}
