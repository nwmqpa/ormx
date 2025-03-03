use sqlx::{Executor, Postgres};

use crate::User;

pub(crate) async fn query_users(
    db: impl Executor<'_, Database = Postgres>,
    filter: Option<&str>,
    limit: Option<usize>,
) -> anyhow::Result<Vec<User>> {
    let result = ormx::conditional_query_as!(
        User,
        r#"SELECT
            id AS user_id, first_name, last_name, email, disabled, favourite_color as "favourite_color: _",
            role AS "role: _", "group" AS "group: _", type as "ty: _", last_login"#
        "FROM users"
        Some(f) = filter => {
            "WHERE first_name LIKE" ?(f)
            "OR last_name LIKE" ?(f)
        }
        "ORDER BY first_name DESC"
        Some(l) = limit => {
            "LIMIT" ?(l as i64)
        }
    )
    .fetch_all(db)
    .await?;

    Ok(result)
}
