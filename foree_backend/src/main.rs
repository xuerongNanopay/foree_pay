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
                column_name: "foo",
                transient: true,
            }
        },
        {
            name: "bar",
            r#type: "aaa",
            sql: {
                column_name: "bar",
                transient: true,
            }
        }
    ]
}

fn main() {
    println!("Hello, world!");
}