fn main() {
    let serch_term = "picture";
    let quote = "\
    Every face, every shop, bedroom window, public-house, and
    dark square is a picture feverishly turned--in search of what?
    It is the same with books. 7 What do we seek through millions of pages?";


    for (i,line) in quote.lines().enumerate(){
        if line.contains(serch_term){
            let line_num = i + 1;
            println!("{}:{}",line_num ,line);
        }
    }
}
