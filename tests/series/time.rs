use theta_chart::{chart::ScaleTime, series::STime};

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

#[test]
fn new_series_time() {
    let d = NaiveDate::from_ymd_opt(2015, 6, 3).unwrap();
    let t = NaiveTime::from_hms_milli_opt(12, 34, 56, 789).unwrap();

    let dt1 = NaiveDateTime::new(d, t);
    let dt2: NaiveDateTime = NaiveDate::from_ymd_opt(2016, 7, 8)
        .unwrap()
        .and_hms_opt(9, 10, 11)
        .unwrap();

    let stime1 = STime::default()
        .set_format("%Y-%m-%d %H:%M:%S")
        .set_data(vec![dt1, dt2]);

    let stime2 = STime::from((
        vec![
            "1982-04-03 00:00:00",
            "1986-02-12 00:00:00",
            "2017-02-04 00:00:00",
            "2020-05-22 00:00:00",
        ],
        "%Y-%m-%d %H:%M:%S",
        "full",
    ));

    let stime3 = STime::from((
        vec!["1982-04-03", "1986-02-12", "2017-02-04", "2020-05-22"],
        "%Y-%m-%d",
        "date",
    ));

    let stime4 = STime::from((vec!["1986", "2017", "2020"], "%Y", "year"));

    dbg!(stime1, stime2, stime3, stime4);
}

#[test]
fn scale_time() {
    let stime = STime::from((vec!["1982", "1986", "2017", "2020"], "%Y", "year"));
    dbg!(&stime);
    let domain = stime.domain();
    dbg!(&domain);

    let distance = stime.count_distance_step();
    dbg!(&distance);

    let intervale = stime.get_intervale(800.);
    dbg!(&intervale);

    let vec_string = stime.gen_sticks_label_step();
    dbg!(vec_string);
}
