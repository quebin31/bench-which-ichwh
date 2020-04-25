fn main() {
    println!("{:?}", futures::executor::block_on(ichwh::which("git")));
}
