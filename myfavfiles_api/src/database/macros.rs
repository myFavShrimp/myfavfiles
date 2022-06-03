macro_rules! columns {
    ($($x: ident => $y: literal,)+) => {
        use sea_query::Iden;

        #[derive(Debug)]
        pub enum Columns {
            $($x,)+
        }

        impl Iden for Columns {
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
