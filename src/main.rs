use leptos::*;
use leptos_struct_table::*;

#[derive(TableRow, Clone, Debug)]
#[table(impl_vec_data_provider, 
    sortable,
)]
pub struct Item {
    #[table(skip)]
    id: u32,
    name: String,
}

#[derive(TableRow, Clone, Debug)]
#[table(impl_vec_data_provider, 
    sortable,
)]
pub struct ItemSafe {
    id: u32,
    name: String,
}

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let rows = vec![
            Item { id: 1, name: "One".to_string() },
            Item { id: 2, name: "Two".to_string()},
            Item { id: 3, name: "Three".to_string()},
        ];
        let rows_safe = vec![
            ItemSafe { id: 1, name: "One".to_string() },
            ItemSafe { id: 2, name: "Two".to_string()},
            ItemSafe { id: 3, name: "Three".to_string()},
        ];

        view! {
            <div>
                <p>Trying to sort the following table will panic:</p>
                <table>
                    <TableContent rows=rows />
                </table>
                <p>Trying to sort the following table will not panic:</p>
                <table>
                    <TableContent rows=rows_safe />
                </table>
            </div>
        }
    });
}