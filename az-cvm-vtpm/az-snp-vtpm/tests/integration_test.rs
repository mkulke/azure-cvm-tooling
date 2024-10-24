use az_snp_vtpm::vtpm;

#[test]
fn test_various_report_data_len() {
    let mut report_data = "test".as_bytes();
    vtpm::get_report_with_report_data(report_data).unwrap();
    report_data = "test_test".as_bytes();
    vtpm::get_report_with_report_data(report_data).unwrap();
}
