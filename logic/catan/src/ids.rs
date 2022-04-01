/// Type-safe wrappers around ints.
/// This is used to refer to handles to Entities as a typed ID,
/// and not some kind of arbitrary u8 for e.g.
/// 
/// This is mostly to give semantic meaning to types such as Map<EntityID, Entity>,
/// where we can now see, that EntityID and Entity are correlated and EntityID
/// shouldn't be used in places where it is not expected to be seen.
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
