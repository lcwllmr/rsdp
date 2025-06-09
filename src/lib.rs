// Re-export BLAS level 1 routines for testing and library use
pub mod blas {
    pub mod traits;

    pub mod level1 {
        pub mod asum;
        pub use asum::asum;
        pub mod nrm2;
        pub use nrm2::nrm2;
        pub mod i_amax;
        pub use i_amax::i_amax;
    }
}
