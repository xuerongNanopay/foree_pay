use nano_foam::foam_struct_proc;

foam_struct_proc!{
    name: "Aaaa",
    features: [JSON, XML],
    sql: {
        table_name: "users",
    }
}

fn main() {
    println!("Hello, world!");
}