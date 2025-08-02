use bson;

use crate::field_witnesses::FieldName;

pub struct UpdateBuilder<T> {
    prefix: Vec<String>,
    clauses: Vec<(String, bson::Bson)>,
    _marker: std::marker::PhantomData<T>,
}

impl<T> UpdateBuilder<T> {
    pub fn new() -> Self {
        UpdateBuilder {
            prefix: Vec::new(),
            clauses: Vec::new(),
            _marker: std::marker::PhantomData,
        }
    }

    /// Returns a fully qualified field path for the given field name marker type.
    fn field_path<F: FieldName>(&self) -> String {
        if self.prefix.is_empty() {
            F::field_name().to_string()
        } else {
            format!("{}.{}", self.prefix.join("."), F::field_name())
        }
    }

    pub fn build(self) -> bson::Document {
        let mut doc = bson::Document::new();

        for (field, clause) in self.clauses {
            doc.insert(field, clause);
        }
        
        doc
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::field_witnesses::FieldName;

    // Mock field name marker types for testing
    struct TestFieldName;
    impl FieldName for TestFieldName {
        fn field_name() -> &'static str {
            "test_field"
        }
    }

    struct AnotherFieldName;
    impl FieldName for AnotherFieldName {
        fn field_name() -> &'static str {
            "another_field"
        }
    }

    struct NestedFieldName;
    impl FieldName for NestedFieldName {
        fn field_name() -> &'static str {
            "nested.field"
        }
    }

    #[test]
    fn test_field_path_empty_prefix() {
        let builder = UpdateBuilder::<()>::new();
        let path = builder.field_path::<TestFieldName>();
        assert_eq!(path, "test_field");
    }

    #[test]
    fn test_field_path_single_prefix() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("parent".to_string());
        
        let path = builder.field_path::<TestFieldName>();
        
        assert_eq!(path, "parent.test_field");
    }

    #[test]
    fn test_field_path_multiple_prefix() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("root".to_string());
        builder.prefix.push("parent".to_string());
        builder.prefix.push("child".to_string());
        
        let path = builder.field_path::<TestFieldName>();
        
        assert_eq!(path, "root.parent.child.test_field");
    }

    #[test]
    fn test_field_path_different_field_types() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("prefix".to_string());
        
        let path1 = builder.field_path::<TestFieldName>();
        
        assert_eq!(path1, "prefix.test_field");
        
        let path2 = builder.field_path::<AnotherFieldName>();
        
        assert_eq!(path2, "prefix.another_field");
    }

    #[test]
    fn test_field_path_nested_field_name() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("outer".to_string());
        
        let path = builder.field_path::<NestedFieldName>();
        
        assert_eq!(path, "outer.nested.field");
    }

    #[test]
    fn test_field_path_empty_string_prefix() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("".to_string());
        
        let path = builder.field_path::<TestFieldName>();
        
        assert_eq!(path, ".test_field");
    }

    #[test]
    fn test_field_path_consistency_across_multiple_calls() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("consistent".to_string());
        
        let path1 = builder.field_path::<TestFieldName>();
        let path2 = builder.field_path::<TestFieldName>();
        
        assert_eq!(path1, path2);
        assert_eq!(path1, "consistent.test_field");
    }

    #[test]
    fn test_field_path_deeply_nested_prefix() {
        let mut builder = UpdateBuilder::<()>::new();
        
        // Simulate a deeply nested structure
        for i in 0..10 {
            builder.prefix.push(format!("level{}", i));
        }
        
        let path = builder.field_path::<TestFieldName>();
        
        assert_eq!(path, "level0.level1.level2.level3.level4.level5.level6.level7.level8.level9.test_field");
    }

    #[test]
    fn test_field_path_special_characters_in_prefix() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("with-dash".to_string());
        builder.prefix.push("with_underscore".to_string());
        builder.prefix.push("with123numbers".to_string());
        
        let path = builder.field_path::<TestFieldName>();
        
        assert_eq!(path, "with-dash.with_underscore.with123numbers.test_field");
    }

    #[test]
    fn test_field_path_with_numeric_string_prefix() {
        let mut builder = UpdateBuilder::<()>::new();
        
        builder.prefix.push("0".to_string());
        builder.prefix.push("123".to_string());
        
        let path = builder.field_path::<TestFieldName>();
        
        assert_eq!(path, "0.123.test_field");
    }
}