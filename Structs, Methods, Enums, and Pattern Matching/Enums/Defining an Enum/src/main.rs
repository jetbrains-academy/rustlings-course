enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: IpAddrKind) {}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    route(four);
    route(six)
}
