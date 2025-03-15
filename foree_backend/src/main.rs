use nano_foam::foam_struct_proc;

foam_struct_proc!{
    name: "Aaaa",
    features: [JSON, XML],
    sql_config: {
        table_name: "users",
    },
    
    properties: {

    }
}

fn main() {
    println!("Hello, world!");
}