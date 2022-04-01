pub mod user;


mod macros {
    #[macro_export]
    macro_rules! impl_iden {
        ($enum: ty, $($x: ident => $y: literal,)+) => {
            use sea_query::Iden;
            
            impl Iden for $enum {
                fn unquoted(&self, s: &mut dyn std::fmt::Write) {
                    write!(
                        s,
                        "{}",
                        match self {
                            $(Self::$x => $y,)+
                        }
                    )
                    .unwrap();
                }
            }
        };
    }

    pub use impl_iden;
}
