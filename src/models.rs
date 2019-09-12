#[derive(Queryable)]
#[derive(Insertable)]
#[table_name = "map_values"]
pub struct MappedValue {
    pub key_value: String,
    pub value: Option<String>
}