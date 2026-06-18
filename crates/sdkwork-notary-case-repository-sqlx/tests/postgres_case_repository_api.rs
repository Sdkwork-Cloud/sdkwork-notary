use sdkwork_notary_case_repository_sqlx::PostgresNotaryCaseRepository;

#[test]
fn postgres_notary_case_repository_api_is_publicly_constructible() {
    let _: fn(sqlx::PgPool, String, String) -> PostgresNotaryCaseRepository =
        PostgresNotaryCaseRepository::new;
    let _ = PostgresNotaryCaseRepository::insert_case;
    let _ = PostgresNotaryCaseRepository::delete_case;
    let _ = PostgresNotaryCaseRepository::list_cases;
}
