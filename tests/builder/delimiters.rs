use clap::{App, Arg};

#[test]
fn opt_default_no_delim() {
    let m = App::new("no_delim")
        .arg(Arg::new("option").long("option").takes_value(true))
        .try_get_matches_from(vec!["", "--option", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn opt_eq_no_delim() {
    let m = App::new("no_delim")
        .arg(Arg::new("option").long("option").takes_value(true))
        .try_get_matches_from(vec!["", "--option=val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn opt_s_eq_no_delim() {
    let m = App::new("no_delim")
        .arg(Arg::new("option").short('o').takes_value(true))
        .try_get_matches_from(vec!["", "-o=val1,val2,val3"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn opt_s_default_no_delim() {
    let m = App::new("no_delim")
        .arg(Arg::new("option").short('o').takes_value(true))
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{:?}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn opt_s_no_space_no_delim() {
    let m = App::new("no_delim")
        .arg(Arg::new("option").short('o').takes_value(true))
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn opt_s_no_space_mult_no_delim() {
    let m = App::new("no_delim")
        .arg(
            Arg::new("option")
                .short('o')
                .takes_value(true)
                .multiple_values(true),
        )
        .try_get_matches_from(vec!["", "-o", "val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(m.value_of("option").unwrap(), "val1,val2,val3");
}

#[test]
fn opt_eq_mult_def_delim() {
    let m = App::new("no_delim")
        .arg(
            Arg::new("option")
                .long("opt")
                .takes_value(true)
                .multiple_values(true)
                .use_delimiter(true),
        )
        .try_get_matches_from(vec!["", "--opt=val1,val2,val3"]);

    assert!(m.is_ok(), "{}", m.unwrap_err());
    let m = m.unwrap();

    assert!(m.is_present("option"));
    assert_eq!(m.occurrences_of("option"), 1);
    assert_eq!(
        m.values_of("option").unwrap().collect::<Vec<_>>(),
        &["val1", "val2", "val3"]
    );
}
