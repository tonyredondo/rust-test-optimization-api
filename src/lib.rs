mod bindings;

mod test_optimization {
    use std::ffi::CString;
    use std::ptr::null_mut;
    use std::thread::panicking;
    use std::time::SystemTime;
    use crate::bindings::*;

    fn get_now() -> unix_time {
        let u_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap();
        let time = unix_time {
            sec: u_time.as_secs(),
            nsec: u_time.subsec_nanos() as u64,
        };
        time
    }

    /********************************
        Test session
    *********************************/

    pub struct TestSession;
    impl TestSession {
        #[allow(dead_code)]
        pub fn init() -> Self {
            let runtime_name = CString::new("Rust").unwrap();
            let runtime_version = CString::new("0.0.1").unwrap();
            unsafe {
                civisibility_initialize(
                    runtime_name.into_raw(),
                    runtime_version.into_raw(),
                    null_mut(),
                    null_mut(),
                    &mut get_now(),
                );
            }
            Self {}
        }

        #[allow(dead_code)]
        pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            let value_cstring = CString::new(value.as_ref()).unwrap();
            unsafe {
                let res = civisibility_session_set_string_tag(
                    key_cstring.into_raw(),
                    value_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            unsafe {
                let res = civisibility_session_set_number_tag(key_cstring.into_raw(), value);
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_error_info(
            &self,
            error_type: impl AsRef<str>,
            error_message: impl AsRef<str>,
            error_stacktrace: impl AsRef<str>,
        ) -> bool {
            let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
            let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
            let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
            unsafe {
                let res = civisibility_session_set_error(
                    error_type_cstring.into_raw(),
                    error_message_cstring.into_raw(),
                    error_stacktrace_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn close(&self, exit_code: i32) {
            unsafe {
                if panicking() {
                    civisibility_shutdown(1, &mut get_now());
                } else {
                    civisibility_shutdown(exit_code, &mut get_now());
                }
            }
        }

        #[allow(dead_code)]
        pub fn create_module(
            &self,
            name: impl AsRef<str>,
            framework_name: impl AsRef<str>,
            framework_version: impl AsRef<str>,
        ) -> TestModule {
            let module_name_cstring = CString::new(name.as_ref()).unwrap();
            let framework_name_cstring = CString::new(framework_name.as_ref()).unwrap();
            let framework_version_cstring = CString::new(framework_version.as_ref()).unwrap();
            unsafe {
                let module_id = civisibility_create_module(
                    module_name_cstring.into_raw(),
                    framework_name_cstring.into_raw(),
                    framework_version_cstring.into_raw(),
                    &mut get_now(),
                );

                TestModule { module_id }
            }
        }
    }

    /********************************
        Test module
    *********************************/

    pub struct TestModule {
        pub module_id: u64,
    }
    impl TestModule {
        #[allow(dead_code)]
        pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            let value_cstring = CString::new(value.as_ref()).unwrap();
            unsafe {
                let res = civisibility_module_set_string_tag(
                    self.module_id,
                    key_cstring.into_raw(),
                    value_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            unsafe {
                let res =
                    civisibility_module_set_number_tag(self.module_id, key_cstring.into_raw(), value);
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_error_info(
            &self,
            error_type: impl AsRef<str>,
            error_message: impl AsRef<str>,
            error_stacktrace: impl AsRef<str>,
        ) -> bool {
            let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
            let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
            let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
            unsafe {
                let res = civisibility_module_set_error(
                    self.module_id,
                    error_type_cstring.into_raw(),
                    error_message_cstring.into_raw(),
                    error_stacktrace_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn close(&self) -> bool {
            unsafe {
                let res = civisibility_close_module(self.module_id, &mut get_now());
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn create_test_suite(&self, name: impl AsRef<str>) -> TestSuite {
            let test_suite_name_cstring = CString::new(name.as_ref()).unwrap();
            unsafe {
                let suite_id = civisibility_create_test_suite(
                    self.module_id,
                    test_suite_name_cstring.into_raw(),
                    &mut get_now(),
                );
                TestSuite {
                    suite_id,
                    module_id: self.module_id,
                }
            }
        }
    }

    /********************************
        Test suite
    *********************************/

    pub struct TestSuite {
        pub suite_id: u64,
        module_id: u64,
    }
    impl TestSuite {
        #[allow(dead_code)]
        pub fn get_module(&self) -> TestModule {
            TestModule {
                module_id: self.module_id,
            }
        }

        #[allow(dead_code)]
        pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            let value_cstring = CString::new(value.as_ref()).unwrap();
            unsafe {
                let res = civisibility_suite_set_string_tag(
                    self.suite_id,
                    key_cstring.into_raw(),
                    value_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            unsafe {
                let res =
                    civisibility_suite_set_number_tag(self.suite_id, key_cstring.into_raw(), value);
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_error_info(
            &self,
            error_type: impl AsRef<str>,
            error_message: impl AsRef<str>,
            error_stacktrace: impl AsRef<str>,
        ) -> bool {
            let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
            let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
            let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
            unsafe {
                let res = civisibility_suite_set_error(
                    self.suite_id,
                    error_type_cstring.into_raw(),
                    error_message_cstring.into_raw(),
                    error_stacktrace_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn close(&self) -> bool {
            unsafe {
                let res = civisibility_close_test_suite(self.suite_id, &mut get_now());
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn create_test(&self, name: impl AsRef<str>) -> Test {
            let test_name_cstring = CString::new(name.as_ref()).unwrap();
            unsafe {
                let test_id = civisibility_create_test(
                    self.suite_id,
                    test_name_cstring.into_raw(),
                    &mut get_now(),
                );
                Test {
                    test_id,
                    suite_id: self.suite_id,
                    module_id: self.module_id,
                }
            }
        }
    }

    /********************************
        Test
    *********************************/

    pub enum TestStatus {
        #[allow(dead_code)]
        Pass = 0,
        #[allow(dead_code)]
        Fail = 1,
        #[allow(dead_code)]
        Skip = 2,
    }

    pub struct Test {
        pub test_id: u64,
        suite_id: u64,
        module_id: u64,
    }
    impl Test {
        #[allow(dead_code)]
        pub fn get_suite(&self) -> TestSuite {
            TestSuite {
                suite_id: self.suite_id,
                module_id: self.module_id,
            }
        }

        #[allow(dead_code)]
        pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            let value_cstring = CString::new(value.as_ref()).unwrap();
            unsafe {
                let res = civisibility_test_set_string_tag(
                    self.test_id,
                    key_cstring.into_raw(),
                    value_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
            let key_cstring = CString::new(key.as_ref()).unwrap();
            unsafe {
                let res = civisibility_test_set_number_tag(self.test_id, key_cstring.into_raw(), value);
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn set_error_info(
            &self,
            error_type: impl AsRef<str>,
            error_message: impl AsRef<str>,
            error_stacktrace: impl AsRef<str>,
        ) -> bool {
            let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
            let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
            let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();
            unsafe {
                let res = civisibility_test_set_error(
                    self.test_id,
                    error_type_cstring.into_raw(),
                    error_message_cstring.into_raw(),
                    error_stacktrace_cstring.into_raw(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }

        #[allow(dead_code)]
        pub fn close(&self, status: TestStatus, skip_reason: impl AsRef<str>) -> bool {
            let mut skip_reason_cstring: *mut ::std::os::raw::c_char = null_mut();
            let skip_reason_ref = skip_reason.as_ref();
            if skip_reason_ref != "" {
                skip_reason_cstring = CString::new(skip_reason_ref).unwrap().into_raw();
            }
            unsafe {
                let res = civisibility_close_test(
                    self.test_id,
                    status as u8,
                    skip_reason_cstring,
                    &mut get_now(),
                );
                if res > 0 {
                    true
                } else {
                    false
                }
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use std::thread::sleep;
        use std::time::Duration;
        use super::*;

        #[test]
        fn it_works() {
            // session
            let session = TestSession::init();
            println!("Hello, world!");

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
    }
}