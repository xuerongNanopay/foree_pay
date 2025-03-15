use nano_foam::foam_struct_proc;

foam_struct_proc!{
    name: "Aaaa",
    features: [JSON, XML],
    sql: {
        table_name: "users",
    },
    
    properties: [
        {
            name: "foo",
            r#type: "aaa",
            sql: {
                table_name: "foo",
                transient: true,
            }
        }
    ]
}

fn main() {
    println!("Hello, world!");
}