
# Formal Verification on Solana

_Powered by Otters._
## Core concept:

Replace:

```rust
anchor-lang = "0.26.0"
```

with

```rust
anchor-lang = { package = "onchor", git = "git@github.com:otter-sec/verify.git" }
```

## Usage

Otter formal verification for Solana only works on rust code that is using the nightly toolchain. So create a new file `rust-toolchain` in the root of your project with the following contents:

```toml
[toolchain]
channel = "nightly"
```

Then, add the `otter-solana-verify` to your `Cargo.toml` dependencies:

```bash
cargo add otter-solana-verify
```


## Invariants

Invariants are properties that should always be true.  For example, the balance of a token account should never be negative. There are two types of invariants in the Solana programs: `account invariants` and `instruction invariants`.

### Instruction Invariants

An instruction invariant specifies sufficient conditions for an instruction to succeed (or fail). These are specified as `succeeds_if` or `errors_if` macro annotations on the instruction handler.

- `succeeds_if` - The instruction should succeed if and only if the given condition is true.

```rust
#[succeeds_if(
    ctx.user.balance > amount
)]
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) {
    // ...
}
```

- `errors_if` - The instruction should fail if and only if the given condition is true.

```rust
#[errors_if(
    ctx.user.balance < amount
)]
pub fn withdraw(ctx: Context<Withdraw>, amount: u64) {
    // ...
}
```

### Account Invariants

The other type of invariant is an Account Invariant. This invariant describes some property of an account that should always hold. We use the `invariant` macro to specify these invariants.

- `invariant` - The account invariant should hold if and only if the given condition is true.

```rust
#[account]
#[invariant(
    self.balance >= 0
)]
struct User {
    pub balance: i64,
}
```
