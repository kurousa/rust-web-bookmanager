#[derive(Debug)]
pub struct PaginatedList<T> {
    pub total: i64,
    pub limit: i64,
    pub offset: i64,
    pub items: Vec<T>,
}

impl<T> PaginatedList<T> {
    pub fn into_inner(self) -> Vec<T> {
        self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_into_inner() {
        let list = PaginatedList {
            total: 10,
            limit: 5,
            offset: 0,
            items: vec![1, 2, 3],
        };

        let items = list.into_inner();
        assert_eq!(items, vec![1, 2, 3]);
    }
}
