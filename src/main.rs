#[derive(Debug)] // Copy не добавлен
struct MyStructPlain {
    field: i32,
}

#[derive(Clone, Copy, Debug)] // Copy требуется для инициализации через new
struct MyStructImpl {
    field: i32,
}

impl MyStructImpl {
    pub fn new() -> Self {
        MyStructImpl { field: 0 }
    }
}

fn main() {
    println!("Hello, world!");
    let mut plain_arr: [MyStructImpl; 3] = [
        MyStructImpl { field: 0 },
        MyStructImpl { field: 0 },
        MyStructImpl { field: 0 },
    ];
    let mut impl_arr: [MyStructImpl; 3] = [MyStructImpl::new(); 3];

    //direct
    plain_arr[0].field += 500;
    plain_arr[0].field += 500;
    impl_arr[0].field += 500;
    impl_arr[0].field += 500;

    //via variable
    let mut var = plain_arr[1];
    var.field += 500;
    var.field += 500;
    let mut var = impl_arr[1];
    var.field += 500; // результат "теряется"
    var.field += 500; // результат "теряется"

    //via ref
    let mut ref_var = &mut plain_arr[2];
    ref_var.field += 500;
    ref_var.field += 500;
    let mut ref_var = &mut impl_arr[2];
    ref_var.field += 500;
    ref_var.field += 500;

    println!("{:?}", plain_arr);
    println!("{:?}", impl_arr);
}
