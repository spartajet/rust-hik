include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_hik_vision() {
    //     unsafe {
    //         let result = MV_CAML_GetDeviceBauderate();
    //         assert_eq!(result, 1);
    //     }
    // }
}