pub mod blas {
    pub mod level1 {
        pub mod sasum;
        pub use sasum::sasum;
        pub mod snrm2;
        pub use snrm2::snrm2;
        pub mod isamax;
        pub use isamax::isamax;
    }
}
