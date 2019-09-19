use super::schema::map_values;

#[derive(Queryable, Insertable)]
#[table_name = "map_values"]
pub struct MappedValue {
    pub key_value: String,
    pub value: Option<String>,
}
