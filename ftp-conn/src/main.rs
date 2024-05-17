use colored::Colorize;
use ftp::FtpStream;

fn main() {
    // replace ip to ftp server
    match FtpStream::connect("44.241.66.173:21") {
        Ok(mut ftp_stream) => {
            match ftp_stream.login("ftp_base", "r5TgKiP18L1VwuX0K0JZEX7V4j31wh") {
                Ok(_) => {
                    println!("{} {}", "[+]".green(), "Successful credentials found.");
                }
                Err(_) => {
                    println!("{} {}", "[-]".yellow(), "Failed to login");
                }
            }

            let _ = ftp_stream.quit();
        }
        Err(_) => {
            println!("Failed to connect to server [-]");
        }
    }
}

