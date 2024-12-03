use std::thread::sleep;
use std::time::Duration;
use crate::test_optimization::*;

#[test]
fn it_works() {
    // session
    let session = TestSession::init();
    println!("Hello, world!");

    println!("{:?}", session.get_settings());
    println!("{:?}", session.get_flaky_test_retries_settings());
    println!("{:?}", session.get_known_tests());

    session.set_string_tag("Session-KeyFromRust", "Hello world");
    session.set_number_tag("Session-NumberFromRust", 42f64);

    // module
    let module_name = String::from("my-test-module");
    let module = session.create_module(module_name, "Framework Name", "Framework Version");
    println!("module id: {:?}", module.module_id);

    module.set_string_tag("Module-KeyFromRust", "Hello world");
    module.set_number_tag("Module-NumberFromRust", 42f64);

    // suite
    let suite = module.create_test_suite("My Suite");
    println!("suite id: {:?}", suite.suite_id);

    suite.set_string_tag("Suite-KeyFromRust", "Hello world");
    suite.set_number_tag("Suite-NumberFromRust", 42f64);

    // pass test
    let pass_test = suite.create_test("My PassTest");
    pass_test.set_string_tag("Pass-KeyFromRust", "Hello world");
    pass_test.set_number_tag("Pass-NumberFromRust", 42f64);
    pass_test.set_test_source("test.rs", &6, &58);
    sleep(Duration::from_millis(1000));
    println!("pass test close: {}", pass_test.close(TestStatus::Pass, ""));

    // fail test
    let fail_test = suite.create_test("My FailTest");
    fail_test.set_string_tag("Fail-KeyFromRust", "Hello world");
    fail_test.set_number_tag("Fail-NumberFromRust", 42f64);
    fail_test.set_error_info("custom_error_type", "error from rust lib", "...");
    sleep(Duration::from_millis(1000));
    println!("fail test close: {}", fail_test.close(TestStatus::Fail, ""));

    // skip test
    let skip_test = suite.create_test("My SkipTest");
    skip_test.set_string_tag("Skip-KeyFromRust", "Hello world");
    skip_test.set_number_tag("Skip-KeyFromRust", 42f64);
    sleep(Duration::from_millis(1000));
    let skip_reason = String::from("skip because yes");
    println!("skip test close: {}", skip_test.close(TestStatus::Skip, skip_reason));

    // close everything
    println!("suite closed: {}", suite.close());
    println!("module closed: {}", module.close());
    session.close(0);
}
