
// Fill in the blanks to make it work
struct A;          // Concrete type `A`.
struct S(A);       // Concrete type `S`.
struct SGen<T>(T); // Generic type `SGen`.

fn reg_fn(_s: S) {}

fn gen_spec_t(_s: SGen<A>) {}

fn gen_spec_i32(_s: SGen<i32>) {}

fn generic<T>(_s: SGen<T>) {}

fn main() {
    // Using the non-generic functions
    reg_fn(__);          // Concrete type.
    gen_spec_t(__);   // Implicitly specified type parameter `A`.
    gen_spec_i32(__); // Implicitly specified type parameter `i32`.

    // Explicitly specified type parameter `char` to `generic()`.
    generic::<char>(__);

    // Implicitly specified type parameter `char` to `generic()`.
    generic(__);

    println!("Success!");
}
