pub fn inner_pub() {
    println!("inner::inner_pub()");
}

pub(crate) fn inner_public_create() {
    println!("inner::inner_pub_crate() – visible trong crate");
}

pub(super) fn inner_public_super() {
    println!("inner::inner_pub_super() – visible cho outer");
}

fn inner_private() {
    println!("inner::inner_private() – chỉ dùng trong inner");
}
