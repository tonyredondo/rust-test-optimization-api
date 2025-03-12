#![allow(non_snake_case)]

use crate::libcivisibility_bindings::*;
use std::alloc::{alloc, dealloc, Layout};
use std::collections::HashMap;
use std::ffi::{c_char, CStr, CString};
use std::ptr::null_mut;
use std::thread::panicking;
use std::time::SystemTime;

fn get_now() -> topt_UnixTime {
    let u_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
    topt_UnixTime {
        sec: u_time.as_secs(),
        nsec: u_time.subsec_nanos() as u64,
    }
}

fn Bool_to_bool(value: Bool) -> bool {
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
    #[allow(dead_code)]
    pub known_tests_enabled: bool,
    #[allow(dead_code)]
    pub test_management: TestManagementSettings,
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
pub struct TestManagementSettings {
    #[allow(dead_code)]
    pub enabled: bool,
    #[allow(dead_code)]
    pub attempt_to_fix_retries: i32,
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

#[derive(Debug, Clone)]
pub struct TestManagementTest {
    #[allow(dead_code)]
    pub module_name: String,
    #[allow(dead_code)]
    pub suite_name: String,
    #[allow(dead_code)]
    pub test_name: String,
    #[allow(dead_code)]
    pub quarantined: bool,
    #[allow(dead_code)]
    pub disabled: bool,
    #[allow(dead_code)]
    pub attempt_to_fix: bool,
}

/********************************
    Test session
*********************************/

#[derive(Debug, Clone)]
pub struct TestSession {
    #[allow(dead_code)]
    pub session_id: u64,
}
impl TestSession {
    #[allow(dead_code)]
    pub fn init() -> Self {
        Self::init_with_values("rust", "rustc", rustc_version_runtime::version().to_string(), None::<&str>)
    }

    #[allow(dead_code)]
    pub fn init_with_values(
        language_name: impl AsRef<str>,
        runtime_name: impl AsRef<str>,
        runtime_version: impl AsRef<str>,
        working_directory: Option<impl AsRef<str>>,
    ) -> Self {

        #[cfg(target_os = "windows")]
        unsafe {
            // On Windows, call the platform-specific initialization
            // this is required on static libraries compiled by the go toolchain
            // just to start the go runtime
            _rt0_amd64_windows_lib()
        }

        // Create CStrings for the required parameters
        let language_name_cstring = CString::new(language_name.as_ref()).unwrap();
        let runtime_name_cstring = CString::new(runtime_name.as_ref()).unwrap();
        let runtime_version_cstring = CString::new(runtime_version.as_ref()).unwrap();
        // Create an optional CString for working_directory if provided
        let working_directory_cstring = working_directory.map(|wd| CString::new(wd.as_ref()).unwrap());

        // Build the initialization options struct, using as_ptr() so the memory is managed automatically
        let init_options = topt_InitOptions {
            language: language_name_cstring.as_ptr() as *mut c_char,
            runtime_name: runtime_name_cstring.as_ptr() as *mut c_char,
            runtime_version: runtime_version_cstring.as_ptr() as *mut c_char,
            working_directory: working_directory_cstring
                .as_ref()
                .map_or(null_mut(), |s| s.as_ptr() as *mut c_char),
            environment_variables: null_mut(),
            global_tags: null_mut(),
            use_mock_tracer: 0,
            unused01: null_mut(),
            unused02: null_mut(),
            unused03: null_mut(),
            unused04: null_mut(),
            unused05: null_mut(),
        };

        // Initialize the library with the provided options
        let initialized = unsafe { Bool_to_bool(topt_initialize(init_options)) };
        if initialized {
            let mut now = get_now();
            let session_result = unsafe { topt_session_create(null_mut(), null_mut(), &mut now) };
            Self {
                session_id: session_result.session_id,
            }
        } else {
            Self { session_id: 0 }
        }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_session_set_string_tag(
                self.session_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_session_set_number_tag(self.session_id, key_cstring.as_ptr() as *mut c_char, value))
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
            Bool_to_bool(topt_session_set_error(
                self.session_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self, exit_code: i32) {
        let mut now = get_now();
        unsafe {
            if panicking() {
                topt_session_close(self.session_id, 1,  &mut now);
            } else {
                topt_session_close(self.session_id, exit_code,  &mut now);
            }
            topt_shutdown();
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

        let mut now = get_now();
        let module_result = unsafe {
            topt_module_create(
                self.session_id,
                module_name_cstring.as_ptr() as *mut c_char,
                framework_name_cstring.as_ptr() as *mut c_char,
                framework_version_cstring.as_ptr() as *mut c_char,
                &mut now,
            )
        };

        TestModule {
            session_id: self.session_id,
            module_id: module_result.module_id,
        }
    }

    #[allow(dead_code)]
    pub fn get_settings(&self) -> Settings {
        unsafe {
            let settings_response = topt_get_settings();
            Settings {
                code_coverage: Bool_to_bool(settings_response.code_coverage),
                early_flake_detection: EfDSettings {
                    enabled: Bool_to_bool(settings_response.early_flake_detection.enabled),
                    slow_test_retries: EfdSlowTestRetriesSettings {
                        ten_s: settings_response.early_flake_detection.slow_test_retries.ten_s,
                        thirty_s: settings_response.early_flake_detection.slow_test_retries.thirty_s,
                        five_m: settings_response.early_flake_detection.slow_test_retries.five_m,
                        five_s: settings_response.early_flake_detection.slow_test_retries.five_s,
                    },
                    faulty_session_threshold: settings_response.early_flake_detection.faulty_session_threshold,
                },
                flaky_test_retries_enabled: Bool_to_bool(settings_response.flaky_test_retries_enabled),
                itr_enabled: Bool_to_bool(settings_response.itr_enabled),
                require_git: Bool_to_bool(settings_response.require_git),
                tests_skipping: Bool_to_bool(settings_response.tests_skipping),
                known_tests_enabled: Bool_to_bool(settings_response.known_tests_enabled),
                test_management: TestManagementSettings {
                    enabled: Bool_to_bool(settings_response.test_management.enabled),
                    attempt_to_fix_retries: settings_response.test_management.attempt_to_fix_retries,
                }
            }
        }
    }

    #[allow(dead_code)]
    pub fn get_flaky_test_retries_settings(&self) -> FlakyTestRetriesSettings {
        unsafe {
            let response = topt_get_flaky_test_retries_settings();
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
            let known_tests = topt_get_known_tests();
            for i in 0..known_tests.len {
                let element = &*known_tests.data.add(i);

                let module_name_c = CStr::from_ptr(element.module_name);
                let suite_name_c = CStr::from_ptr(element.suite_name);
                let test_name_c = CStr::from_ptr(element.test_name);

                let module_name_string = module_name_c.to_string_lossy().into_owned();
                let suite_name_string = suite_name_c.to_string_lossy().into_owned();
                let test_name = test_name_c.to_string_lossy().into_owned();

                let suites_map = modules_map.entry(module_name_string).or_insert_with(HashMap::new);
                let tests_vec = suites_map.entry(suite_name_string).or_insert_with(Vec::new);
                tests_vec.push(test_name);
            }
            topt_free_known_tests(known_tests);
            modules_map
        }
    }

    #[allow(dead_code)]
    pub fn get_skippable_tests(&self) -> HashMap<String, HashMap<String, Vec<SkippableTest>>> {
        unsafe {
            let mut suites_map: HashMap<String, HashMap<String, Vec<SkippableTest>>> = HashMap::new();
            let skippable_tests = topt_get_skippable_tests();
            for i in 0..skippable_tests.len {
                let element = &*skippable_tests.data.add(i);

                let suite_name_c = CStr::from_ptr(element.suite_name);
                let test_name_c = CStr::from_ptr(element.test_name);
                let parameters_c = CStr::from_ptr(element.parameters);
                let custom_configurations_json_c = CStr::from_ptr(element.custom_configurations_json);

                let suite_name_string = suite_name_c.to_string_lossy().into_owned();
                let test_name_string = test_name_c.to_string_lossy().into_owned();
                let parameters_string = parameters_c.to_string_lossy().into_owned();
                let custom_configurations_json_string = custom_configurations_json_c.to_string_lossy().into_owned();

                let suites_map_entry = suites_map.entry(suite_name_string.clone()).or_insert_with(HashMap::new);
                let tests_vec = suites_map_entry.entry(test_name_string.clone()).or_insert_with(Vec::new);

                tests_vec.push(SkippableTest {
                    suite_name: suite_name_string,
                    test_name: test_name_string,
                    parameters: parameters_string,
                    custom_configurations_json: custom_configurations_json_string,
                });
            }
            topt_free_skippable_tests(skippable_tests);
            suites_map
        }
    }

    #[allow(dead_code)]
    pub fn get_test_management_tests(&self) -> HashMap<String, HashMap<String, HashMap<String, TestManagementTest>>> {
        unsafe {
            let mut modules_map: HashMap<String, HashMap<String, HashMap<String, TestManagementTest>>> =  HashMap::new();
            let test_management_tests = topt_get_test_management_tests();
            for i in 0..test_management_tests.len {
                let element = &*test_management_tests.data.add(i);

                let module_name_c = CStr::from_ptr(element.module_name);
                let suite_name_c = CStr::from_ptr(element.suite_name);
                let test_name_c = CStr::from_ptr(element.test_name);

                let module_name_string = module_name_c.to_string_lossy().into_owned();
                let suite_name_string = suite_name_c.to_string_lossy().into_owned();
                let test_name_string = test_name_c.to_string_lossy().into_owned();

                let modules_map_entry = modules_map.entry(module_name_string.clone()).or_insert_with(HashMap::new);
                let suites_map_entry = modules_map_entry.entry(suite_name_string.clone()).or_insert_with(HashMap::new);
                _ = suites_map_entry.entry(test_name_string.clone()).or_insert(TestManagementTest {
                    module_name: module_name_string,
                    suite_name: suite_name_string,
                    test_name: test_name_string,
                    quarantined: Bool_to_bool(element.quarantined),
                    disabled: Bool_to_bool(element.disabled),
                    attempt_to_fix: Bool_to_bool(element.attempt_to_fix),
                });
            }
            topt_free_test_management_tests(test_management_tests);
            modules_map
        }
    }
}

/********************************
    Test module
*********************************/

#[derive(Debug, Clone)]
pub struct TestModule {
    session_id: u64,
    pub module_id: u64,
}
impl TestModule {
    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_module_set_string_tag(
                self.module_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_module_set_number_tag(
                self.module_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
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
            Bool_to_bool(topt_module_set_error(
                self.module_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            Bool_to_bool(topt_module_close(self.module_id, &mut now))
        }
    }

    #[allow(dead_code)]
    pub fn create_test_suite(&self, name: impl AsRef<str>) -> TestSuite {
        let test_suite_name_cstring = CString::new(name.as_ref()).unwrap();
        let mut now = get_now();
        let suite_result = unsafe {
            topt_suite_create(
                self.module_id,
                test_suite_name_cstring.as_ptr() as *mut c_char,
                &mut now,
            )
        };
        TestSuite {
            suite_id: suite_result.suite_id,
            module_id: self.module_id,
            session_id: self.session_id,
        }
    }
}

/********************************
    Test suite
*********************************/

#[derive(Debug, Clone)]
pub struct TestSuite {
    pub suite_id: u64,
    module_id: u64,
    session_id: u64,
}
impl TestSuite {
    #[allow(dead_code)]
    pub fn get_module(&self) -> TestModule {
        TestModule { module_id: self.module_id, session_id: self.session_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_suite_set_string_tag(
                self.suite_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_suite_set_number_tag(
                self.suite_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
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
            Bool_to_bool(topt_suite_set_error(
                self.suite_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_test_source(
        &self,
        file: impl AsRef<str>,
        start_line: *const i32,
        end_line: *const i32,
    ) -> bool {
        let file_cstring = CString::new(file.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_suite_set_source(
                self.suite_id,
                file_cstring.as_ptr() as *mut c_char,
                start_line as *mut i32,
                end_line as *mut i32,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            Bool_to_bool(topt_suite_close(self.suite_id, &mut now))
        }
    }

    #[allow(dead_code)]
    pub fn create_test(&self, name: impl AsRef<str>) -> Test {
        let test_name_cstring = CString::new(name.as_ref()).unwrap();
        let mut now = get_now();
        let test_result = unsafe {
            topt_test_create(
                self.suite_id,
                test_name_cstring.as_ptr() as *mut c_char,
                &mut now,
            )
        };
        Test {
            test_id: test_result.test_id,
            suite_id: self.suite_id,
            module_id: self.module_id,
            session_id: self.session_id,
        }
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
    session_id: u64,
}
impl Test {
    #[allow(dead_code)]
    pub fn get_suite(&self) -> TestSuite {
        TestSuite { suite_id: self.suite_id, module_id: self.module_id,  session_id: self.session_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_string_tag(
                self.test_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_number_tag(
                self.test_id,
                key_cstring.as_ptr() as *mut c_char,
                value,
            ))
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
            Bool_to_bool(topt_test_set_error(
                self.test_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_test_source(
        &self,
        file: impl AsRef<str>,
        start_line: *const i32,
        end_line: *const i32,
    ) -> bool {
        let file_cstring = CString::new(file.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_test_set_source(
                self.test_id,
                file_cstring.as_ptr() as *mut c_char,
                start_line as *mut i32,
                end_line as *mut i32,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self, status: TestStatus) -> bool {
        let mut now = get_now();
        let close_options = topt_TestCloseOptions {
            status: status as u8,
            finish_time: &mut now,
            skip_reason: null_mut(),
            unused01: null_mut(),
            unused02: null_mut(),
            unused03: null_mut(),
            unused04: null_mut(),
            unused05: null_mut(),
        };
        unsafe {
            Bool_to_bool(topt_test_close(self.test_id,close_options))
        }
    }

    #[allow(dead_code)]
    pub fn close_with_skip_reason(&self, skip_reason: impl AsRef<str>) -> bool {
        let skip_reason_ref = skip_reason.as_ref();
        if !skip_reason_ref.is_empty() {
            let skip_reason_cstring = CString::new(skip_reason_ref).unwrap();
            let mut now = get_now();
            let close_options = topt_TestCloseOptions {
                status: TestStatus::Skip as u8,
                finish_time: &mut now,
                skip_reason: skip_reason_cstring.as_ptr() as *mut c_char,
                unused01: null_mut(),
                unused02: null_mut(),
                unused03: null_mut(),
                unused04: null_mut(),
                unused05: null_mut(),
            };
            unsafe { Bool_to_bool(topt_test_close(self.test_id, close_options)) }
        } else {
            self.close(TestStatus::Skip)
        }
    }

    #[allow(dead_code)]
    pub fn set_coverage_data(&self, files: &[impl AsRef<str>]) {
        unsafe {
            // Allocate memory for an array of topt_TestCoverageFile
            let layout = Layout::array::<topt_TestCoverageFile>(files.len()).unwrap();
            let coverage_file_ptr = alloc(layout) as *mut topt_TestCoverageFile;
            // Create a vector to hold the CString values so they remain valid
            let mut cstrings = Vec::with_capacity(files.len());
            for (idx, file) in files.iter().enumerate() {
                // Create a CString from the file string
                let cstr = CString::new(file.as_ref()).unwrap();
                // Store the CString to keep it alive
                cstrings.push(cstr);
                // Get the pointer to the stored CString
                let filename_ptr = cstrings.last().unwrap().as_ptr() as *mut c_char;
                *coverage_file_ptr.add(idx) = topt_TestCoverageFile {
                    filename: filename_ptr,
                    bitmap: null_mut(),
                    bitmap_len: 0,
                };
            }

            let mut coverage_data = topt_TestCoverage {
                session_id: self.session_id,
                suite_id: self.suite_id,
                test_id: self.test_id,
                files: coverage_file_ptr,
                files_len: files.len(),
            };

            // Send the code coverage payload
            topt_send_code_coverage_payload(&mut coverage_data, 1);

            // Deallocate the memory for the array of topt_TestCoverageFile
            dealloc(coverage_file_ptr as *mut u8, layout);
            // The CString objects in `cstrings` are automatically freed when they go out of scope.
        }
    }
}

/********************************
    Spans
*********************************/

#[derive(Debug, Clone)]
pub struct Span {
    pub span_id: u64,
    pub parent_id: u64,
}
impl Span {
    pub fn create(
        parent_id: u64,
        operation_name: impl AsRef<str>,
        service_name: impl AsRef<str>,
        resource_name: impl AsRef<str>,
        span_type: impl AsRef<str>,
    ) -> Self {

        let operation_name_cstring = CString::new(operation_name.as_ref()).unwrap();
        let service_name_cstring = CString::new(service_name.as_ref()).unwrap();
        let resource_name_cstring = CString::new(resource_name.as_ref()).unwrap();
        let span_type_cstring = CString::new(span_type.as_ref()).unwrap();
        let mut now = get_now();

        let span_start_options = topt_SpanStartOptions {
            operation_name: operation_name_cstring.as_ptr() as *mut c_char,
            service_name: service_name_cstring.as_ptr() as *mut c_char,
            resource_name: resource_name_cstring.as_ptr() as *mut c_char,
            span_type: span_type_cstring.as_ptr() as *mut c_char,
            start_time: &mut now,
            string_tags: null_mut(),
            number_tags: null_mut(),
        };

        let span_result = unsafe {
            topt_span_create(parent_id, span_start_options)
        };

        Self{ span_id: span_result.span_id, parent_id }
    }

    #[allow(dead_code)]
    pub fn set_string_tag(&self, key: impl AsRef<str>, value: impl AsRef<str>) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        let value_cstring = CString::new(value.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_span_set_string_tag(
                self.span_id,
                key_cstring.as_ptr() as *mut c_char,
                value_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn set_number_tag(&self, key: impl AsRef<str>, value: f64) -> bool {
        let key_cstring = CString::new(key.as_ref()).unwrap();
        unsafe {
            Bool_to_bool(topt_span_set_number_tag(self.span_id, key_cstring.as_ptr() as *mut c_char, value))
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
            Bool_to_bool(topt_span_set_error(
                self.span_id,
                error_type_cstring.as_ptr() as *mut c_char,
                error_message_cstring.as_ptr() as *mut c_char,
                error_stacktrace_cstring.as_ptr() as *mut c_char,
            ))
        }
    }

    #[allow(dead_code)]
    pub fn close(&self) -> bool {
        let mut now = get_now();
        unsafe {
            Bool_to_bool(topt_span_close(self.span_id, &mut now))
        }
    }
}