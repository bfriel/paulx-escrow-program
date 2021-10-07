/// Localnet testing params:
/// Completed Program ID: D2Ce7a3RjWd7SF8LHXqqcxqWp7WXWLvBDywUMJZNc4PD
/// Alice Throwaway Sollet Address: GLAdjCa38boaXipQzy2fhy1QxnQR9YZLPuDnhumg8bPP
/// Bob Throwaway Sollet Address: AAYs6yJkdZUokZ1PqEBKTeFDLyjWMR8x5XiAH9EaobYg
/// Token Mint Account X: FMLNEfUxuXhDmpcoAzaC8sK24Nxn2ZNQ9JJ3nRwTgqmB
/// Alice X Token Account: HJsscEATGN8KrUMkzJNahfoCdnSoj9253aPie9Dx7iuH | Start with 100
/// Bob X Token Account: H6f33vEnBJdBB3DpjeJaadtZNVvZCkRTKFqpkiscouxV | Start with 0
/// Token Mint Account Y: HYVDM6tua8Di5SFPsBNKwgXzZekTn898ZzMRRZdDtPoA
/// Alice Y Token Account: EmJT3qqim485pRjCWfiU9oXiuMYxaPY1ZiGTy9bhnJ4Z | Start with 0
/// Bob Y Token Account: 3TSntfftxJM2u7k1jesSaRgBuCmCVM2cdppfd3VbJCL7 | Start with 100
pub mod instruction;
pub mod error;
pub mod processor;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint;