pub mod m_probe_opts {
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum ProbeOperations {
        FlashProgram,
        RTTIO,
    }

    impl Default for ProbeOperations {
        fn default() -> Self {
            Self::FlashProgram
        }
    }
}
