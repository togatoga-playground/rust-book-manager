use kernel::{
    model::{
        book::list::PaginatedList,
        id::{BookId, UserId},
        user::BookOwner,
    },
    repository::book::MockBookRepository,
};
use rstest::rstest;
use std::sync::Arc;

#[rstest]
#[case("/books", 20, 0)]
#[case("/books?limit=50", 50, 0)]
#[case("/books?limit=50&offset=20", 50, 20)]
#[case("/books?offset=20", 20, 20)]
#[tokio::test]
async fn show_book_list_with_query_200(
    mut fixture: registry::MockAppRegistryExt,
    #[case] path: &str,
    #[case] expected_limit: i64,
    #[case] expected_offset: i64,
) -> anyhow::Result<()> {
    use kernel::model::book::Book;

    let book_id = BookId::new();
    fixture.expect_book_repository().returning(move || {
        let mut mock = MockBookRepository::new();
        mock.expect_find_all().returning(move |opt| {
            let items = vec![Book {
                id: book_id,
                title: "RustによるWebアプリケーション開発".to_string(),
                author: "Yuki Toyoda".to_string(),
                isbn: "978-4-297-11101-4".to_string(),
                description: "RustによるWebアプリケーション開発".to_string(),
                owner: BookOwner {
                    id: UserId::new(),
                    name: "Yuki Toyoda".to_string(),
                },
                checkout: None,
            }];
            Ok(PaginatedList {
                total: 1,
                limit: opt.limit,
                offset: opt.offset,
                items,
            })
        });
        Arc::new(mock)
    });
    Ok(())
}
