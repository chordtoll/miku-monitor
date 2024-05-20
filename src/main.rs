fn main() {
    println!("{}",String::from_utf8(libsqlite3_sys::SQLITE_VERSION.to_vec()).unwrap());
    println!("Hello, world!");
    MainWindow::new().unwrap().run().unwrap();
}

slint::slint! {
    export component MainWindow inherits Window {
        Text {
            text: "hello world";
            color: green;
        }
    }
}
