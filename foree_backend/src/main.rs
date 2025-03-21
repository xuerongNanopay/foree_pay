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
            r#type: i32,
            sql: {
                column_name: "foo",
                transient: true,
            }
        },
        {
            name: "bar",
            r#type: String,
            sql: {
                column_name: "bar",
                transient: true,
            }
        }
    ],

    sql_queries: [
        {
            fn_name: "getById",
            query: " SELECT #foo, #bar from #Aaaa WHERE #foo=?foo; ",
        }
    ]
}

fn main() {
    println!("Hello, world!");
}