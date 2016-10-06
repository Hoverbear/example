use schema::tests;

#[derive(Queryable)]
#[insertable_into(tests)]
pub struct Test {
    pub id: i32,
}
