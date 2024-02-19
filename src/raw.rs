pub trait FromRaw {
    type CType;

    fn from_raw(raw: Self::CType) -> Self;

    fn from_ref(raw: &Self::CType) -> &Self;
}

pub trait AsRaw: FromRaw {
    fn as_raw(&self) -> &Self::CType;
}

pub trait AsRawMut: AsRaw {
    fn as_raw_mut(&mut self) -> &mut Self::CType;
}

pub trait IntoRaw: FromRaw {
    fn into_raw(self) -> Self::CType;
}
