use crate::tag::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse_quote, Attribute, ItemFn};

#[test]
fn test_parse_tag_labels() {
    let input: TokenStream = "tag1, tag2".parse().unwrap();
    let result = parse_tag_labels(input);

    assert!(result.is_ok());

    let labels: Vec<String> = result
        .unwrap()
        .iter()
        .map(std::string::ToString::to_string)
        .collect();
    assert_eq!(2, labels.len());
    assert_eq!("tag1", labels[0]);
    assert_eq!("tag2", labels[1]);
}

#[test]
fn test_validate_no_tag_attr() {
    let func: ItemFn = parse_quote! {
        #[no_tag_attr]
        fn test_ok() {}
    };
    let result = validate_no_tag_attr(&func);
    assert!(result.is_ok());

    let func: ItemFn = parse_quote! {
        #[tag]
        fn test_fail() {}
    };
    let result = validate_no_tag_attr(&func);
    assert!(result.is_err());
}

#[test]
fn test_validate_tag_labels() {
    let allowed = vec!["tag1".into(), "tag2".into()];

    //valid
    let tags: TagLabels = parse_quote!(tag1, tag2);
    let result = validate_tag_labels(&tags, &allowed);
    assert!(result.is_ok());

    //invalid
    let tags: TagLabels = parse_quote!(tag1, tag3);
    let result = validate_tag_labels(&tags, &allowed);
    assert!(result.is_err());
    assert_eq!(
        "Invalid tag 'tag3'. Allowed tags are: [\"tag1\", \"tag2\"].",
        result.unwrap_err().to_string()
    );

    //duplicated
    let tags: TagLabels = parse_quote!(tag1, tag1);
    let result = validate_tag_labels(&tags, &allowed);
    assert!(result.is_err());
    assert_eq!("Duplicated tag 'tag1'.", result.unwrap_err().to_string());
}

#[test]
fn test_recognize_tag_attribute() {
    let attrs: Vec<Attribute> = vec![
        parse_quote!(#[tag]),
        parse_quote!(#[tag(tag1)]),
        parse_quote!(#[pinny::tag]),
        parse_quote!(#[pinny::tag(tag1)]),
        parse_quote!(#[test]),
        parse_quote!(#[ignore]),
    ];

    assert!(is_tag_attr(&attrs[0]));
    assert!(is_tag_attr(&attrs[1]));
    assert!(is_tag_attr(&attrs[2]));
    assert!(is_tag_attr(&attrs[3]));
    assert!(!is_tag_attr(&attrs[4]));
    assert!(!is_tag_attr(&attrs[5]));
}

#[test]
fn test_disambiguate_any_test_attr() {
    fn to_string(attr: &Attribute) -> String {
        //Token stream will produce: "# [self :: test]"
        //so removing extra spacing for simple comparison
        attr.to_token_stream()
            .to_string()
            .split_whitespace()
            .collect()
    }

    let mut attrs: Vec<Attribute> = vec![
        parse_quote!(#[tag(tag1)]),
        parse_quote!(#[ignore]),
        parse_quote!(#[parent::test]),
        parse_quote!(#[test]),
        parse_quote!(#[test="value"]),
        parse_quote!(#[test(param)]),
    ];

    disambiguate_any_test_attr(&mut attrs);

    assert_eq!(6, attrs.len(), "attributes len should not change");
    assert_eq!(
        "#[tag(tag1)]",
        to_string(&attrs[0]),
        "tag should not change"
    );
    assert_eq!(
        "#[ignore]",
        to_string(&attrs[1]),
        "ignore should not change"
    );
    assert_eq!(
        "#[parent::test]",
        to_string(&attrs[2]),
        "parent::test should not change"
    );
    assert_eq!("#[self::test]", to_string(&attrs[3]), "test should change");
    assert_eq!(
        "#[self::test=\"value\"]",
        to_string(&attrs[4]),
        "test=value should change"
    );
    assert_eq!(
        "#[self::test(param)]",
        to_string(&attrs[5]),
        "test(param) should change"
    );
}
