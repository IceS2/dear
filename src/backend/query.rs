// pub(crate) struct Query<Entity, Filter>
// where
//     Entity: SQLEntity,
//     Filter: SQLFilter
// {
//     entity: Entity,
//     filter: Option<Filter>
// }

// pub(crate) struct NoteUpdatedFilter {
//     from: Option<String>,
//     to: Option<String>
// }

// pub(crate) struct NoteFilter {
//     tags: Option<Vec<String>>,
//     updated: Option<NoteUpdatedFilter>
// }

// pub(crate) enum Filter {
//     NoteFilter,
// }

// pub(crate) struct Filter {
//     attribute: Attribute,
//     value: AttributeValue,
//     operator: Operator
// }

// impl Filter {
//     fn as_str(&self) -> &str {
//         let attribute = self.attribute.as_str();
//         let value = self.value.as_str();
//         let operator = self.operator.as_str();

//         format!("{attribute} {operator} {value}")
//     }
// }

// pub(crate) enum AttributeValue {
//     String(String),
//     Integer(String),
//     Datetime(String),
//     JSON(String)
// }

// impl  Query {
//     fn to_sql(&self) -> &self {
//         let attributes = self.entity.list_attributes.join(", ");
//         let entity = self.entity.name();
//         let filter = self.build_filter();
//         let query = format!("
//             SELECT {attributes}
//             FROM {entity}
//             WHERE {filter}");
//     }
//     fn build_filter(&self) -> &str {
//         let mut filters: Vec<&str> = Vec::new();
//         for filter in self.filters {
//             filters = filters.push(filter.as_str());
//         }
//         filters.join(" AND ");
//     }
// }

// // SELECT <attributes> FROM <entity> WHERE <Filter> AND <Filter>
