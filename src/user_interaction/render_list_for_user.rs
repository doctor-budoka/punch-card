const DEFAULT_MAX_ITEMS: usize = 5;

pub fn render_list_for_user(str_list: &Vec<String>, max_items: Option<usize>) -> String {
    let max_items_non_opt: usize =  max_items.unwrap_or(DEFAULT_MAX_ITEMS);
    if (str_list.len() == 0) | (max_items_non_opt == 0) {
        return "None".to_string();
    } else if str_list.len() <= max_items_non_opt {
        return str_list.join(", ");
    } else {
        return str_list[..max_items_non_opt].join(", ") + ", ...";
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;
    use test_case::test_matrix;
    use crate::random_generators::generate_random_string::generate_random_string;

    #[test_case(None; "No max given")]
    #[test_case(Some(0); "Max 0")]
    #[test_case(Some(1); "Max 1")]
    #[test_case(Some(3); "Max 3")]
    fn test_returns_none_for_empty_list(max_items: Option<usize>) {
        let out = render_list_for_user(&Vec::new(), max_items);
        assert_eq!(out, "None");
    }

    #[test_case(0; "Empty vec")]
    #[test_case(1; "Single item")]
    #[test_case(3; "3 items")]
    fn test_returns_none_for_max_size_0(vec_size: usize) {
        let test_list = (0..vec_size).into_iter().map(|_x| generate_random_string(10)).collect();
        let out = render_list_for_user(&test_list, Some(0));
        assert_eq!(out, "None");
    }
    
    #[test_matrix(
        [1, 3, 5],
        [None, Some(5), Some(10)]
    )]
    fn test_under_the_max(num_items_to_test: usize, test_max_size: Option<usize>) {
        let test_list = (0..num_items_to_test).into_iter().map(|_x| generate_random_string(10)).collect();
        
        let out: String = render_list_for_user(&test_list, test_max_size);
        let expected: String = test_list.join(", ");
        assert_eq!(out, expected);
    }

    #[test_matrix(
        [6, 8, 10],
        [None, Some(1), Some(3)]
    )]
    fn test_over_the_max(num_items_to_test: usize, test_max_size: Option<usize>) {
        let test_list = (0..num_items_to_test).into_iter().map(|_x| generate_random_string(10)).collect();
        let out: String = render_list_for_user(&test_list, test_max_size);
        let max_size: usize = test_max_size.unwrap_or(5);
        let expected: String = Vec::from_iter(test_list.iter().take(max_size).map(|x|  x.to_owned())).join(", ") + ", ...";
        assert_eq!(out, expected);
    }
}
