pub mod test_optimization;
mod bindings;
#[cfg(test)]
mod tests;
#[cfg(target_os = "windows")]
mod cgo;