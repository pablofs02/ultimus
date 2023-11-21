use ultimus::prompt::Prompt;

fn main() {
    let mut p = Prompt::iniciar(">>> ");
    loop {
        let f = p.preguntar();
        println!("Lo dicho: '{f}'");
        if f == "q" {
            break;
        }
    }
}
