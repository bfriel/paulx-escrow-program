/// Localnet testing params:
/// Sollet Address: GLAdjCa38boaXipQzy2fhy1QxnQR9YZLPuDnhumg8bPP
/// Token Mint Account X: BzVURnS8u564h2K8mwEGJ88C4yqULMtfzFrFqM5Yi76A
/// Alice X Token Account: 3qxT15LvmQeoAv3oUdEbhEoPNue4fqTVwEpi8gS7ygGU
/// Token Mint Account Y: 7Gfxf1ZwaTSpqrtapNDoWNPBWjcK3C56WN3ykJqAAVg6
/// Alice Y Token Account: H3BXiXbCC3eBaS1MeyUweytWXUWpofKrPMnGvWJ2595m
pub mod instruction;
pub mod error;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;