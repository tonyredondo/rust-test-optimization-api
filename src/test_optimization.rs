use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use crate::bindings::*;
use std::ffi::{c_uchar, CString};
use std::ptr::{null_mut};
use std::thread::panicking;
use std::time::SystemTime;
use libc::{free, c_void};

fn get_now() -> unix_time {
    let u_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    unix_time {
        sec: u_time.as_secs(),
        nsec: u_time.subsec_nanos() as u64,
    }
}

fn c_uchar_to_bool(value: c_uchar) -> bool {
    value > 0
}

#[derive(Debug, Clone)]
pub struct Settings {
    #[allow(dead_code)]
    pub code_coverage: bool,
    #[allow(dead_code)]
    pub early_flake_detection: EfDSettings,
    #[allow(dead_code)]
    pub flaky_test_retries_enabled: bool,
    #[allow(dead_code)]
    pub itr_enabled: bool,
    #[allow(dead_code)]
    pub require_git: bool,
    #[allow(dead_code)]
    pub tests_skipping: bool,
}

#[derive(Debug, Clone)]
pub struct EfDSettings {
    #[allow(dead_code)]
    pub enabled: bool,
    #[allow(dead_code)]
    pub slow_test_retries: EfdSlowTestRetriesSettings,
    #[allow(dead_code)]
    pub faulty_session_threshold: i32,
}

#[derive(Debug, Clone)]
pub struct EfdSlowTestRetriesSettings {
    #[allow(dead_code)]
    pub ten_s: i32,
    #[allow(dead_code)]
    pub thirty_s: i32,
    #[allow(dead_code)]
    pub five_m: i32,
    #[allow(dead_code)]
    pub five_s: i32,
}

#[derive(Debug, Clone)]
pub struct FlakyTestRetriesSettings {
    #[allow(dead_code)]
    pub retry_count: i32,
    #[allow(dead_code)]
    pub total_retry_count: i32,
}

#[derive(Debug, Clone)]
pub struct SkippableTest {
    #[allow(dead_code)]
    pub suite_name: String,
    #[allow(dead_code)]
    pub test_name: String,
    #[allow(dead_code)]
    pub parameters: String,
    #[allow(dead_code)]
    pub custom_configurations_json: String,
}

/********************************
    Test session
*********************************/

#[derive(Debug, Clone)]
pub struct TestSession;
impl TestSession {

    #[allow(dead_code)]
    pub fn init() -> Self {
        Self::init_with_values("rust", "rustc", rustc_version_runtime::version().to_string())
    }

    #[allow(dead_code)]
    pub fn init_with_values(language_name: impl AsRef<str>, runtime_name: impl AsRef<str>, runtime_version: impl AsRef<str>) -> Self {

        #[cfg(target_os = "windows")]
        unsafe {
            _rt0_amd64_windows_lib()
        }

        let language_name_cstring = CString::new(language_name.as_ref()).unwrap();
        let runtime_name_cstring = CString::new(runtime_name.as_ref()).unwrap();
        let runtime_version_cstring = CString::new(runtime_version.as_ref()).unwrap();

        // Convert to raw pointers
        let language_ptr = language_name_cstring.into_raw();
        let runtime_name_ptr = runtime_name_cstring.into_raw();
        let runtime_version_ptr = runtime_version_cstring.into_raw();

        let mut now = get_now();
        unsafe {
            civisibility_initialize(
                language_ptr,
                runtime_name_ptr,
                runtime_version_ptr,
                null_mut(),
                null_mut(),
                &mut now
            );
        }

        // Restore and free
        unsafe {
            let _ = CString::from_raw(language_ptr);
            let _ = CString::from_raw(runtime_name_ptr);
            let _ = CString::from_raw(runtime_version_ptr);
        }

        Self {}
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();

        let key_ptr = key_cstring.into_raw();
        let value_ptr = value_cstring.into_raw();
        let result = unsafe {
            c_uchar_to_bool(civisibility_session_set_string_tag(key_ptr, value_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
            let _ = CString::from_raw(value_ptr);
        }
        result
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let key_ptr = key_cstring.into_raw();
        let result = unsafe {
            c_uchar_to_bool(civisibility_session_set_number_tag(key_ptr, value))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
        }
        result
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();

        let error_type_ptr = error_type_cstring.into_raw();
        let error_message_ptr = error_message_cstring.into_raw();
        let error_stacktrace_ptr = error_stacktrace_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_session_set_error(error_type_ptr, error_message_ptr, error_stacktrace_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(error_type_ptr);
            let _ = CString::from_raw(error_message_ptr);
            let _ = CString::from_raw(error_stacktrace_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn close(&self, exit_code: i32) {
        let mut now = get_now();
        unsafe {
            if panicking() {
                civisibility_shutdown(1, &mut now);
            } else {
                civisibility_shutdown(exit_code, &mut now);
            }
        }
    }

    #[allow(dead_code)]
    pub fn create_module(&self, name: impl AsRef<str>, framework_name: impl AsRef<str>, framework_version: impl AsRef<str>) -> TestModule {
        let module_name_cstring = CString::new(name.as_ref()).unwrap();
        let framework_name_cstring = CString::new(framework_name.as_ref()).unwrap();
        let framework_version_cstring = CString::new(framework_version.as_ref()).unwrap();

        let module_name_ptr = module_name_cstring.into_raw();
        let framework_name_ptr = framework_name_cstring.into_raw();
        let framework_version_ptr = framework_version_cstring.into_raw();

        let mut now = get_now();
        let module_id = unsafe {
            civisibility_create_module(
                module_name_ptr,
                framework_name_ptr,
                framework_version_ptr,
                &mut now)
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(module_name_ptr);
            let _ = CString::from_raw(framework_name_ptr);
            let _ = CString::from_raw(framework_version_ptr);
        }

        TestModule { module_id }
    }

    #[allow(dead_code)]
    pub fn get_settings(&self) -> Settings {
        unsafe {
            let settings_response = civisibility_get_settings();
            Settings {
                code_coverage: c_uchar_to_bool(settings_response.code_coverage),
                early_flake_detection: EfDSettings {
                    enabled: c_uchar_to_bool(settings_response.early_flake_detection.enabled),
                    slow_test_retries: EfdSlowTestRetriesSettings {
                        ten_s: settings_response.early_flake_detection.slow_test_retries.ten_s,
                        thirty_s: settings_response.early_flake_detection.slow_test_retries.thirty_s,
                        five_m: settings_response.early_flake_detection.slow_test_retries.five_m,
                        five_s: settings_response.early_flake_detection.slow_test_retries.five_s,
                    },
                    faulty_session_threshold: settings_response.early_flake_detection.faulty_session_threshold,
                },
                flaky_test_retries_enabled: c_uchar_to_bool(settings_response.flaky_test_retries_enabled),
                itr_enabled: c_uchar_to_bool(settings_response.itr_enabled),
                require_git: c_uchar_to_bool(settings_response.require_git),
                tests_skipping: c_uchar_to_bool(settings_response.tests_skipping),
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_flaky_test_retries_settings(&self) -> FlakyTestRetriesSettings {
        unsafe {
            let response = civisibility_get_flaky_test_retries_settings();
            FlakyTestRetriesSettings {
                retry_count: response.retry_count,
                total_retry_count: response.total_retry_count,
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_known_tests(&self) -> HashMap<String, HashMap<String, Vec<String>>> {
        unsafe {
            let mut modules_map: HashMap<String, HashMap<String, Vec<String>>> = HashMap::new();
            let mut known_tests: *mut known_test = null_mut();
            let length = civisibility_get_known_tests(&mut known_tests) as i32;
            for i in 0..length {
                let element = &*known_tests.add(i as usize);

                let module_name_c = CString::from_raw(element.module_name);
                let suite_name_c = CString::from_raw(element.suite_name);
                let test_name_c = CString::from_raw(element.test_name);

                let module_name_string = module_name_c.into_string().unwrap();
                let suite_name_string = suite_name_c.into_string().unwrap();
                let test_name = test_name_c.into_string().unwrap();

                let suites_map = modules_map.entry(module_name_string).or_insert_with(HashMap::new);
                let tests_vec = suites_map.entry(suite_name_string).or_insert_with(Vec::new);
                tests_vec.push(test_name);
            }
            free(known_tests as *mut c_void);
            modules_map
        }
    }

    #[allow(dead_code)]
    pub fn get_skippable_tests(&self) -> HashMap<String, HashMap<String, Vec<SkippableTest>>> {
        unsafe {
            let mut suites_map : HashMap<String, HashMap<String, Vec<SkippableTest>>> = HashMap::new();
            let mut skippable_tests : *mut skippable_test = null_mut();
            let length = civisibility_get_skippable_tests(&mut skippable_tests) as u32;
            for i in 0..length {
                let element = &*skippable_tests.add(i as usize);

                let suite_name_c = CString::from_raw(element.suite_name);
                let test_name_c = CString::from_raw(element.test_name);
                let parameters_c = CString::from_raw(element.parameters);
                let custom_configurations_json_c = CString::from_raw(element.custom_configurations_json);

                let suite_name_string = suite_name_c.into_string().unwrap();
                let test_name_string = test_name_c.into_string().unwrap();
                let parameters_string = parameters_c.into_string().unwrap();
                let custom_configurations_json_string = custom_configurations_json_c.into_string().unwrap();

                let suites_map_entry = suites_map.entry(suite_name_string.clone()).or_insert_with(HashMap::new);
                let tests_vec = suites_map_entry.entry(test_name_string.clone()).or_insert_with(Vec::new);

                tests_vec.push(SkippableTest {
                    suite_name: suite_name_string,
                    test_name: test_name_string,
                    parameters: parameters_string,
                    custom_configurations_json: custom_configurations_json_string,
                });
            }
            free(skippable_tests as *mut c_void);
            suites_map
        }
    }
}

/********************************
    Test module
*********************************/

#[derive(Debug, Clone)]
pub struct TestModule {
    pub module_id: u64,
}
impl TestModule {

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();

        let key_ptr = key_cstring.into_raw();
        let value_ptr = value_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_module_set_string_tag(self.module_id, key_ptr, value_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
            let _ = CString::from_raw(value_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let key_ptr = key_cstring.into_raw();
        let result = unsafe {
            c_uchar_to_bool(civisibility_module_set_number_tag(self.module_id, key_ptr, value))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();

        let error_type_ptr = error_type_cstring.into_raw();
        let error_message_ptr = error_message_cstring.into_raw();
        let error_stacktrace_ptr = error_stacktrace_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_module_set_error(self.module_id, error_type_ptr, error_message_ptr, error_stacktrace_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(error_type_ptr);
            let _ = CString::from_raw(error_message_ptr);
            let _ = CString::from_raw(error_stacktrace_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            c_uchar_to_bool(civisibility_close_module(self.module_id, &mut now))
        }
    }

    #[allow(dead_code)]
    pub fn create_test_suite(&self, name: impl AsRef<str>) -> TestSuite {
        let test_suite_name_cstring = CString::new(name.as_ref()).unwrap();
        let test_suite_name_ptr = test_suite_name_cstring.into_raw();
        let mut now = get_now();
        let suite_id = unsafe {
            civisibility_create_test_suite(
                self.module_id,
                test_suite_name_ptr,
                &mut now)
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(test_suite_name_ptr);
        }
        TestSuite { suite_id, module_id: self.module_id }
    }
}

/********************************
    Test suite
*********************************/

#[derive(Debug, Clone)]
pub struct TestSuite {
    pub suite_id: u64,
    module_id: u64
}
impl TestSuite {
    #[allow(dead_code)]
    pub fn get_module(&self) -> TestModule {
        TestModule { module_id: self.module_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();

        let key_ptr = key_cstring.into_raw();
        let value_ptr = value_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_suite_set_string_tag(self.suite_id, key_ptr, value_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
            let _ = CString::from_raw(value_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let key_ptr = key_cstring.into_raw();
        let result = unsafe {
            c_uchar_to_bool(civisibility_suite_set_number_tag(self.suite_id, key_ptr, value))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
        }
        result
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();

        let error_type_ptr = error_type_cstring.into_raw();
        let error_message_ptr = error_message_cstring.into_raw();
        let error_stacktrace_ptr = error_stacktrace_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_suite_set_error(self.suite_id, error_type_ptr, error_message_ptr, error_stacktrace_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(error_type_ptr);
            let _ = CString::from_raw(error_message_ptr);
            let _ = CString::from_raw(error_stacktrace_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            c_uchar_to_bool(civisibility_close_test_suite(self.suite_id, &mut now))
        }
    }

    #[allow(dead_code)]
    pub fn create_test(&self, name: impl AsRef<str>) -> Test {
        let test_name_cstring = CString::new(name.as_ref()).unwrap();
        let test_name_ptr = test_name_cstring.into_raw();
        let mut now = get_now();
        let test_id = unsafe {
            civisibility_create_test(
                self.suite_id,
                test_name_ptr,
                &mut now)
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(test_name_ptr);
        }
        Test { test_id, suite_id: self.suite_id, module_id: self.module_id }
    }
}

/********************************
    Test
*********************************/

#[derive(Debug, Clone)]
pub enum TestStatus {
    Pass = 0,
    Fail = 1,
    Skip = 2,
}

#[derive(Debug, Clone)]
pub struct Test {
    pub test_id: u64,
    suite_id: u64,
    module_id: u64,
}
impl Test {
    #[allow(dead_code)]
    pub fn get_suite(&self) -> TestSuite {
        TestSuite { suite_id: self.suite_id, module_id: self.module_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();

        let key_ptr = key_cstring.into_raw();
        let value_ptr = value_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_test_set_string_tag(self.test_id, key_ptr, value_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
            let _ = CString::from_raw(value_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let key_ptr = key_cstring.into_raw();
        let result = unsafe {
            c_uchar_to_bool(civisibility_test_set_number_tag(self.test_id, key_ptr, value))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(key_ptr);
        }
        result
    }

    #[allow(dead_code)]
    pub fn set_error_info(&self, error_type: impl AsRef<str>, error_message: impl AsRef<str>, error_stacktrace: impl AsRef<str>) -> bool {
        let error_type_cstring = CString::new(error_type.as_ref()).unwrap();
        let error_message_cstring = CString::new(error_message.as_ref()).unwrap();
        let error_stacktrace_cstring = CString::new(error_stacktrace.as_ref()).unwrap();

        let error_type_ptr = error_type_cstring.into_raw();
        let error_message_ptr = error_message_cstring.into_raw();
        let error_stacktrace_ptr = error_stacktrace_cstring.into_raw();

        let result = unsafe {
            c_uchar_to_bool(civisibility_test_set_error(self.test_id, error_type_ptr, error_message_ptr, error_stacktrace_ptr))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(error_type_ptr);
            let _ = CString::from_raw(error_message_ptr);
            let _ = CString::from_raw(error_stacktrace_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn set_test_source(
        &self,
        file: impl AsRef<str>,
        start_line: *const i32,
        end_line: *const i32,
    ) -> bool {
        let file_cstring = CString::new(file.as_ref()).unwrap();
        let file_ptr = file_cstring.into_raw();
        let c_start_line = start_line as *mut i32;
        let c_end_line = end_line as *mut i32;

        let result = unsafe {
            c_uchar_to_bool(civisibility_test_set_test_source(
                self.test_id,
                file_ptr,
                c_start_line,
                c_end_line,
            ))
        };

        // Restore and free
        unsafe {
            let _ = CString::from_raw(file_ptr);
        }

        result
    }

    #[allow(dead_code)]
    pub fn close(&self, status: TestStatus) -> bool {
        let mut now = get_now();
        unsafe {
            c_uchar_to_bool(civisibility_close_test(self.test_id,
                                                    status as u8,
                                                    null_mut(),
                                                    &mut now))
        }
    }

    #[allow(dead_code)]
    pub fn close_with_skip_reason(&self, skip_reason: impl AsRef<str>) -> bool {
        let skip_reason_ref = skip_reason.as_ref();
        if !skip_reason_ref.is_empty() {
            let skip_reason_cstring = CString::new(skip_reason_ref).unwrap();
            let skip_reason_ptr = skip_reason_cstring.into_raw();

            let mut now = get_now();
            let result = unsafe {
                c_uchar_to_bool(civisibility_close_test(self.test_id,
                                                        TestStatus::Skip as u8,
                                                        skip_reason_ptr,
                                                        &mut now))
            };

            // Restore and free
            unsafe {
                let _ = CString::from_raw(skip_reason_ptr);
            }
            result
        } else {
            let mut now = get_now();
            unsafe {
                c_uchar_to_bool(civisibility_close_test(self.test_id,
                                                        TestStatus::Skip as u8,
                                                        null_mut(),
                                                        &mut now))
            }
        }
    }

    #[allow(dead_code)]
    pub fn set_coverage_data(&self, files: &[impl AsRef<str>]) {
        unsafe {
            let layout = Layout::array::<test_coverage_file>(files.len()).unwrap();
            let coverage_file_ptr = alloc(layout) as *mut test_coverage_file;
            let mut idx = 0;
            // Allocate and store file names
            for file in files.iter() {
                let file_cstring = CString::new(file.as_ref()).unwrap();
                let filename_ptr = file_cstring.into_raw();
                *coverage_file_ptr.add(idx) = test_coverage_file {
                    filename: filename_ptr,
                };
                idx += 1;
            }

            let mut coverage_data = test_coverage {
                test_suite_id: self.suite_id,
                span_id: self.test_id,
                files: coverage_file_ptr,
                files_len: files.len() as u64,
            };

            civisibility_send_code_coverage_payload(&mut coverage_data, 1);

            // Free filenames
            for i in 0..files.len() {
                let file_struct = &*coverage_file_ptr.add(i);
                let _ = CString::from_raw(file_struct.filename);
            }

            dealloc(coverage_file_ptr as *mut u8, layout);
        }
    }
}
