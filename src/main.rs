use log;
use std::{fs, env, io};
use simple_logger::SimpleLogger;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    SimpleLogger::new().init().unwrap();

    let args: Vec<_> = env::args().collect();

    // We expect 1 argument (the path), return if not.
    if args.len() != 2 {
        log::error!("Usage: {} \"PDF folder path\"", args[0]);
        return;
    }

    // The first argument is the path that was used to call the program.
    log::info!("Program path is {}.", args[0]);

    let path = ::std::env::args().nth(1).unwrap();

    // Create a channel to receive the events.
    let (sender, receiver) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(sender, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        match receiver.recv() {
           Ok(event) => println!("{:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    }

}

//struct PdfFolder {
    //path: String,
    //pdfs: Vec<String>,

//fn main() {
    //SimpleLogger::new().init().unwrap();

    //let args: Vec<_> = env::args().collect();

    //// We expect 1 argument (the path), return if not.
    //if args.len() != 2 {
        //log::error!("Usage: {} \"PDF folder path\"", args[0]);
        //return;
    //}

    //// The first argument is the path that was used to call the program.
    //log::info!("Program path is {}.", args[0]);

    //let folder = PdfFolder {
        //path: ::std::env::args().nth(1).unwrap(),
        //pdfs: Vec::new(),
    //};

    //log::info!("The PATH argument given is:{:?}", folder.path);

    //let res = pdfs(folder);

    ////for file in folder {
        ////log::trace!("{:?}", entry);
    ////}

    //log::info!("The result is:{:?}", res);
//}


//fn pdfs(mut folder: PdfFolder) -> io::Result<()> {

    //let entries = fs::read_dir(&folder.path)?
        //.map(|res| res.map(|e| e.path()))
        //.collect::<Result<Vec<_>, io::Error>>()?;

    //for entry in entries {
        //folder.pdfs.push(format!("{:?}", entry));
        //log::info!("{:?}", entry);
        ////log::trace!("{:?}", entry);
        ////println!("{:?}", entry);
    //}

    //for file in folder.pdfs {
        //log::trace!("{:?}", file);
    //}

    //Ok(())
//}
