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
    println!("{:?}", session.get_skippable_tests());
    println!("{:?}", session.get_test_management_tests());

    println!("session id: {:?}", session.session_id);

    session.set_string_tag("Session-KeyFromRust", "Hello world");
    session.set_number_tag("Session-NumberFromRust", 42f64);

    // Session span
    let session_span = Span::create(session.session_id,  "my-operation-name", "my-service", "session-resource-name", "span-type");
    println!("span_id (from session): {:?}", session_span.span_id);
    session_span.set_string_tag("Session-KeyFromRust", "Hello world");
    session_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("session_span close: {}", session_span.close());

    // module
    let module_name = String::from("my-test-module");
    let module = session.create_module(module_name, "Framework Name", "Framework Version");
    println!("module id: {:?}", module.module_id);

    module.set_string_tag("Module-KeyFromRust", "Hello world");
    module.set_number_tag("Module-NumberFromRust", 42f64);

    // Module span
    let module_span = Span::create(module.module_id,  "my-operation-name", "my-service", "module-resource-name", "span-type");
    println!("span_id (from module): {:?}", module_span.span_id);
    module_span.set_string_tag("Session-KeyFromRust", "Hello world");
    module_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("module_span close: {}", module_span.close());
    
    // suite
    let suite = module.create_test_suite("My Suite");
    println!("suite id: {:?}", suite.suite_id);

    suite.set_string_tag("Suite-KeyFromRust", "Hello world");
    suite.set_number_tag("Suite-NumberFromRust", 42f64);

    // Suite span
    let suite_span = Span::create(suite.suite_id,  "my-operation-name", "my-service", "suite-resource-name", "span-type");
    println!("span_id (from suite): {:?}", suite_span.span_id);
    suite_span.set_string_tag("Session-KeyFromRust", "Hello world");
    suite_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("suite_span close: {}", suite_span.close());

    // pass test
    let pass_test = suite.create_test("My PassTest");
    pass_test.set_string_tag("Pass-KeyFromRust", "Hello world");
    pass_test.set_number_tag("Pass-NumberFromRust", 42f64);
    pass_test.set_test_source("test.rs", &6, &58);
    pass_test.set_coverage_data(&["file.rs"]);
    sleep(Duration::from_millis(1000));

    // Test span
    let test_span = Span::create(pass_test.test_id,  "my-operation-name", "my-service", "test-resource-name", "span-type");
    println!("span_id (from test): {:?}", test_span.span_id);
    test_span.set_string_tag("Session-KeyFromRust", "Hello world");
    test_span.set_number_tag("Session-NumberFromRust", 42f64);
    sleep(Duration::from_millis(500));
    println!("test_span close: {}", test_span.close());

    println!("pass test close: {}", pass_test.close(TestStatus::Pass));
    
    // fail test
    let fail_test = suite.create_test("My FailTest");
    fail_test.set_string_tag("Fail-KeyFromRust", "Hello world");
    fail_test.set_number_tag("Fail-NumberFromRust", 42f64);
    fail_test.set_error_info("custom_error_type", "error from rust lib", "...");
    sleep(Duration::from_millis(1000));
    println!("fail test close: {}", fail_test.close(TestStatus::Fail));

    // skip test
    let skip_test = suite.create_test("My SkipTest");
    skip_test.set_string_tag("Skip-KeyFromRust", "Hello world");
    skip_test.set_number_tag("Skip-KeyFromRust", 42f64);
    sleep(Duration::from_millis(1000));
    let skip_reason = String::from("skip because yes");
    println!("skip test close: {}", skip_test.close_with_skip_reason(skip_reason));

    // close everything
    println!("suite closed: {}", suite.close());
    println!("module closed: {}", module.close());
    session.close(0);
}
