macro_rules! int_wrapper {
    ($name: ident, $ty: ty) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, ::serde::Deserialize, Hash)]
        pub struct $name(pub $ty);

        impl From<$name> for usize {
            fn from(v: $name) -> Self {
                v.0 as usize
            }
        }

        impl TryFrom<usize> for $name {
            type Error = ::std::num::TryFromIntError;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                value.try_into().map($name)
            }
        }

    };
}

int_wrapper!(TileID, u8);
int_wrapper!(ResourceTileID, u8);
int_wrapper!(RoadID, u16);
int_wrapper!(SettlePlaceID, u16);
int_wrapper!(DiceMarkerID, u8);
int_wrapper!(PlayerID, u8);
