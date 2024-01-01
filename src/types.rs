/// `ActionFunc` is a type alias for a boxed closure that can be called once.
///
/// # Lifetime
///
/// The `'a` lifetime ensures that the closure borrows data with the same lifetime `'a`.
/// This means that the closure cannot outlive the data it borrows.
///
/// # Returns
///
/// Returns a `Result` with an empty tuple as the `Ok` variant and a `Box<dyn std::error::Error>`
/// as the `Err` variant. The `Box<dyn std::error::Error>` is a boxed dynamic trait object that
/// implements the `std::error::Error` trait.
pub type ActionFunc<'a> =
    Box<dyn for<'b> FnOnce() -> Result<(), Box<dyn std::error::Error + 'a>> + 'a>;
