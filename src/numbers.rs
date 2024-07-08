#![allow(dead_code)]

pub struct Zero;
pub struct Suc<T> {
    _phantom: std::marker::PhantomData<T>,
}

pub trait Number {}
impl Number for Zero {}
impl<T: Number> Number for Suc<T> {}

pub trait NormalForm {
    type NF: Number;
}

impl NormalForm for Zero {
    type NF = Zero;
}

impl<T: NormalForm> NormalForm for Suc<T> {
    type NF = Suc<T::NF>;
}

pub trait SameNormalForm {}
impl<A: NormalForm, B: NormalForm> SameNormalForm for (A, B) where (A::NF, B::NF): SameNumber {}

pub trait SameNumber {}
impl<T> SameNumber for (T, T) where T: Number {}

pub type One = Suc<Zero>;
pub type Two = Suc<One>;
pub type Three = Suc<Two>;
pub type Four = Suc<Three>;
pub type Five = Suc<Four>;
pub type Six = Suc<Five>;
pub type Seven = Suc<Six>;
pub type Eight = Suc<Seven>;
pub type Nine = Suc<Eight>;
pub type Ten = Suc<Nine>;

struct Same<A, B>
where
    A: NormalForm,
    B: NormalForm,
    (A, B): SameNormalForm,
{
    _phantom: std::marker::PhantomData<(A, B)>,
}

pub struct Add<A, B> {
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<B: NormalForm> NormalForm for Add<Zero, B> {
    type NF = B::NF;
}

impl<A: NormalForm, B: NormalForm> NormalForm for Add<Suc<A>, B>
where
    Add<A, B>: NormalForm,
{
    type NF = <Suc<Add<A, B>> as NormalForm>::NF;
}

pub struct Mul<A, B> {
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<B: NormalForm> NormalForm for Mul<Zero, B> {
    type NF = Zero;
}

impl<A: NormalForm, B: NormalForm> NormalForm for Mul<Suc<A>, B>
where
    Add<B, Mul<A, B>>: NormalForm,
{
    type NF = <Add<B, Mul<A, B>> as NormalForm>::NF;
}

struct Fac<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl NormalForm for Fac<Zero> {
    type NF = Suc<Zero>;
}

impl<T: NormalForm> NormalForm for Fac<Suc<T>>
where
    Fac<T>: NormalForm,
    Mul<Suc<T>, <Fac<T> as NormalForm>::NF>: NormalForm,
{
    type NF = <Mul<Suc<T>, <Fac<T> as NormalForm>::NF> as NormalForm>::NF;
}
