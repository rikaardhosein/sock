use sock::Socks5Server;

fn main() {
    println!("Hello World!");
    let s = Socks5Server {
        bind_addr: "127.0.0.1:1337".to_string()
    };
    s.start();

}