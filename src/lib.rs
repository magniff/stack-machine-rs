#![allow(dead_code)]

mod numbers;
use numbers::*;

struct Empty;

struct NOOP;
struct ADD;
struct DUP;
struct ROT3;

struct JUMP<T> {
    _phantom: std::marker::PhantomData<T>,
}
struct PUSH<T> {
    _phantom: std::marker::PhantomData<T>,
}

struct Stack<S0, S1, S2, S3> {
    _phantom: std::marker::PhantomData<(S0, S1, S2, S3)>,
}

struct Instructions<I0, I1, I2, I3, I4, I5, I6, I7> {
    _phantom: std::marker::PhantomData<(I0, I1, I2, I3, I4, I5, I6, I7)>,
}

struct VMState<Stack, Instructions, IP> {
    _phantom: std::marker::PhantomData<(Stack, Instructions, IP)>,
}

trait Fetch {
    type I;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch
    for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Zero)
{
    type I = I0;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, One) {
    type I = I1;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Two) {
    type I = I2;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch
    for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Three)
{
    type I = I3;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch
    for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Four)
{
    type I = I4;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch
    for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Five)
{
    type I = I5;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Six) {
    type I = I6;
}

impl<I0, I1, I2, I3, I4, I5, I6, I7> Fetch
    for (Instructions<I0, I1, I2, I3, I4, I5, I6, I7>, Seven)
{
    type I = I7;
}

trait Exec {
    type NewVmState;
}

impl<Stack, Instructions, IP> Exec for (NOOP, Stack, Instructions, IP) {
    type NewVmState = VMState<Stack, Instructions, Suc<IP>>;
}

impl<Stack, Instructions, IP, T> Exec for (JUMP<T>, Stack, Instructions, IP) {
    type NewVmState = VMState<Stack, Instructions, T>;
}

impl<S0, S1, S2, S3, Instructions, IP, T> Exec
    for (PUSH<T>, Stack<S0, S1, S2, S3>, Instructions, IP)
{
    type NewVmState = VMState<Stack<T, S0, S1, S2>, Instructions, Suc<IP>>;
}

impl<S0, S1, S2, S3, Instructions, IP> Exec for (ROT3, Stack<S0, S1, S2, S3>, Instructions, IP) {
    type NewVmState = VMState<Stack<S2, S0, S1, S3>, Instructions, Suc<IP>>;
}

impl<S0, S1, S2, S3, Instructions, IP> Exec for (DUP, Stack<S0, S1, S2, S3>, Instructions, IP) {
    type NewVmState = VMState<Stack<S0, S0, S1, S2>, Instructions, Suc<IP>>;
}

impl<S0, S1, S2, S3, Instructions, IP> Exec for (ADD, Stack<S0, S1, S2, S3>, Instructions, IP)
where
    Add<S0, S1>: NormalForm,
{
    type NewVmState =
        VMState<Stack<<Add<S0, S1> as NormalForm>::NF, S2, S3, Empty>, Instructions, Suc<IP>>;
}

impl<Stack, Instructions, IP> VMState<Stack, Instructions, IP>
where
    (Instructions, IP): Fetch,
    (<(Instructions, IP) as Fetch>::I, Stack, Instructions, IP): Exec,
{
    pub fn step(
        self,
    ) -> <(<(Instructions, IP) as Fetch>::I, Stack, Instructions, IP) as Exec>::NewVmState {
        todo!()
    }
}

mod tests {
    use super::*;

    type Program = Instructions<NOOP, PUSH<Zero>, PUSH<One>, NOOP, NOOP, NOOP, NOOP, NOOP>;

    #[test]
    fn test_basic() {
        let vm: VMState<
            Stack<Empty, Empty, Empty, Empty>,
            Instructions<PUSH<One>, PUSH<Two>, ADD, NOOP, NOOP, NOOP, NOOP, NOOP>,
            Zero,
        > = VMState {
            _phantom: std::marker::PhantomData,
        };
        let vm = vm.step().step().step();
    }

    #[test]
    fn test_jump() {
        let vm: VMState<
            Stack<Empty, Empty, Empty, Empty>,
            Instructions<PUSH<One>, DUP, ADD, JUMP<One>, NOOP, NOOP, NOOP, NOOP>,
            Zero,
        > = VMState {
            _phantom: std::marker::PhantomData,
        };
        let vm = vm
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step();
    }

    #[test]
    fn test_fib() {
        // 1 1 2 3 5 8 13 21
        let vm: VMState<
            Stack<Empty, Empty, Empty, Empty>,
            Instructions<PUSH<One>, PUSH<Two>, DUP, ROT3, ADD, JUMP<Two>, NOOP, NOOP>,
            Zero,
        > = VMState {
            _phantom: std::marker::PhantomData,
        };
        let vm = vm
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step()
            .step();
    }
}
