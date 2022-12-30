// Helper macro to copy iterator state between parse helper function invocations
#[macro_export]
macro_rules! run_parser {
    ($iter: ident, $function:ident) => {
        {
            let mut iter_clone= $iter.clone();
            let token= $function(&mut iter_clone)?;
            $iter.clone_from(&iter_clone);

            Some(token)
        }
    }
}
