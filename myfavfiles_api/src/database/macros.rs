macro_rules! columns {
    ($($x: ident => $y: literal,)+) => {
        #[derive(Debug, Eq, PartialEq)]
        pub enum Columns {
            $($x,)+
        }

        impl sea_query::Iden for Columns {
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
