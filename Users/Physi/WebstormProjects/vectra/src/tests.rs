use crate::memory::{Db, DirectoryHandle};

#[tokio::test]
async fn store_and_retrieve() {
    let embedding = vec![1.0, 2.0, 3.0];

    let mut Vectra = Db::new(DirectoryHandle::new());

    Vectra.write("hello", embedding.clone(), vec![]).await;

    let result = Vectra
        .find_nearest_neighbors(embedding, vec![], 1)
        .await
        .first()
        .unwrap()
        .content
        .clone();

    assert_eq!(result, "hello".to_string());
}

#[tokio::test]
async fn store_two_and_retrieve() {
    let embedding_1 = vec![1.0, 2.0, 3.0];
    let embedding_2 = vec![-1.0, -2.0, -3.0];

    let mut Vectra = Db::new(DirectoryHandle::new());

    Vectra.write("hello", embedding_1.clone(), vec![]).await;
    Vectra.write("goodbye", embedding_2.clone(), vec![]).await;

    {
        let result = Vectra
            .find_nearest_neighbors(embedding_1, vec![], 1)
            .await
            .first()
            .unwrap()
            .content
            .clone();

        assert_eq!(result, "hello".to_string());
    }
    {
        let result = Vectra
            .find_nearest_neighbors(embedding_2, vec![], 1)
            .await
            .first()
            .unwrap()
            .content
            .clone();

        assert_eq!(result, "goodbye".to_string());
    }
}

#[tokio::test]
async fn store_two_and_retrieve_with_tags() {
    let embedding_1 = vec![1.0, 2.0, 3.0];
    let embedding_2 = vec![-1.0, -2.0, -3.0];

    let mut Vectra = Db::new(DirectoryHandle::new());

    Vectra
        .write("hello", embedding_1.clone(), vec!["greetings".to_string()])
        .await;
    Vectra
        .write("goodbye", embedding_2.clone(), vec!["goodbyes".to_string()])
        .await;

    {
        let result = Vectra
            .find_nearest_neighbors(embedding_1.clone(), vec![], 1)
            .await
            .first()
            .unwrap()
            .content
            .clone();

        assert_eq!(result, "hello".to_string());
    }
    {
        let result = Vectra
            .find_nearest_neighbors(embedding_2.clone(), vec![], 1)
            .await
            .first()
            .unwrap()
            .content
            .clone();

        assert_eq!(result, "goodbye".to_string());
    }

    {
        let result = Vectra
            .find_nearest_neighbors(embedding_1.clone(), vec!["goodbyes".to_string()], 1)
            .await
            .first()
            .unwrap()
            .content
            .clone();

        assert_eq!(result, "goodbye".to_string());
    }
    {
        let result = Vectra
            .find_nearest_neighbors(embedding_2, vec!["greetings".to_string()], 1)
            .await
            .first()
            .unwrap()
            .clone();

        assert_eq!(result.content, "hello");
    }
    {
        let result = Vectra
            .find_nearest_neighbors(embedding_1, vec!["mysterious".to_string()], 1)
            .await;

        assert_eq!(result.first(), None);
    }
}

#[should_panic]
#[tokio::test]
async fn incompatible_size_panic() {
    let embedding_1 = vec![1.0, 2.0, 3.0];
    let embedding_2 = vec![1.0, 2.0, 3.0, 4.0];

    let mut Vectra = Db::new(DirectoryHandle::new());

    Vectra.write("hello", embedding_1, vec![]).await;
    Vectra.write("hello", embedding_2, vec![]).await;
}
