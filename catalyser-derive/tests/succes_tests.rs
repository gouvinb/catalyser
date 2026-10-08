mod something {
    use catalyser_derive::overloads;
    use std::{
        fmt,
        time::Duration,
    };

    #[overloads]
    pub fn add(a: i32, #[default(10)] b: i32) -> i32 {
        a + b
    }

    #[overloads]
    pub fn describe<T: fmt::Debug>(v: T, #[default("value")] label: &'static str) -> String {
        format!("{label}: {v:?}")
    }

    #[overloads]
    pub fn ports(#[default(80)] http: u16, #[default(http + 363)] https: u16) -> (u16, u16) {
        (http, https)
    }

    #[overloads]
    pub fn as_micros_str(#[default(Duration::from_secs(0))] duration: Duration) -> String {
        duration.as_micros().to_string()
    }

    #[overloads]
    pub fn nothing() -> &'static str {
        "ok"
    }

    #[overloads]
    pub fn opt(#[default(None)] x: Option<u8>, #[default(Some(1))] y: Option<u8>) -> (Option<u8>, Option<u8>) {
        (x, y)
    }

    #[overloads]
    pub async fn async_calc(x: i32, #[default(5)] factor: i32) -> i32 {
        x * factor
    }

    pub struct Calculator {
        pub base: i32,
    }

    #[overloads]
    impl Calculator {
        pub fn new(#[default(100)] base: i32) -> Self {
            Calculator { base }
        }

        pub fn calculate(&self, multiplier: i32, #[default(10)] offset: i32) -> i32 {
            self.base * multiplier + offset
        }

        pub fn add_assign(&mut self, amount: i32, #[default(1)] step: i32) {
            self.base += amount * step;
        }

        pub fn into_total(self, #[default(0)] extra: i32) -> i32 {
            self.base + extra
        }

        pub async fn async_eval(&self, factor: i32, #[default(2)] exponent: u32) -> i32 {
            self.base * factor.pow(exponent)
        }
    }

    pub struct Container<T> {
        pub item: T,
    }

    #[overloads]
    impl<T: Default> Container<T> {
        pub fn create(#[default(T::default())] item: T) -> Self {
            Container { item }
        }

        #[allow(clippy::needless_lifetimes)]
        pub fn format_with<'a, U: fmt::Display>(&self, prefix: U, #[default("item")] suffix: &'a str) -> String {
            format!("{prefix} {suffix}")
        }
    }

    #[overloads]
    pub trait MathOps {
        fn modulo(&self, a: i32, #[default(10)] b: i32) -> i32;

        fn power(base: u32, #[default(2)] exp: u32) -> u32;

        fn default_body_mult(&self, x: i32, #[default(2)] factor: i32) -> i32 {
            x * factor
        }
    }

    pub struct MathWorker;

    impl MathOps for MathWorker {
        fn modulo(&self, a: i32, b: i32) -> i32 {
            a % b
        }

        fn power(base: u32, exp: u32) -> u32 {
            base.pow(exp)
        }
    }

    #[overloads]
    pub trait Transformer<T: Clone> {
        fn transform(&self, item: T, #[default(1)] repeat: usize) -> Vec<T>;
    }

    pub struct Replicator;

    impl<T: Clone> Transformer<T> for Replicator {
        fn transform(&self, item: T, repeat: usize) -> Vec<T> {
            vec![item; repeat]
        }
    }
}

use something::*;
use std::time::Duration;

#[test]
fn function() {
    assert_eq!(add(1, 2), 3);
    assert_eq!(add_overload(1, AddArgs { b: 2 }), 3);
    assert_eq!(add_overload(1, AddArgs::default()), 11);
    assert_eq!(add_default(1), 11);
}

#[test]
fn function_with_generic() {
    assert_eq!(describe(5, "n"), "n: 5");
    assert_eq!(describe_overload(5, DescribeArgs { label: "n" }), "n: 5");
    assert_eq!(describe_overload(5, DescribeArgs::default()), "value: 5");
    assert_eq!(describe_default(5), "value: 5");
}

#[test]
fn function_with_combined_defaults() {
    assert_eq!(ports(80, 443), (80, 443));
    assert_eq!(
        ports_overload(PortsArgs {
            http: 80,
            https: 443
        }),
        (80, 443)
    );
    assert_eq!(
        ports_overload(PortsArgs {
            http: 8080,
            ..PortsArgs::default()
        }),
        (8080, 443)
    );
    assert_eq!(ports_overload(PortsArgs::default()), (80, 443));
    assert_eq!(ports_default(), (80, 443));
}

#[test]
fn function_with_struct_default_param() {
    assert_eq!(as_micros_str(Duration::from_secs(1)), "1000000");
    assert_eq!(
        as_micros_str_overload(AsMicrosStrArgs {
            duration: Duration::from_secs(1)
        }),
        "1000000"
    );
    assert_eq!(as_micros_str_overload(AsMicrosStrArgs::default()), "0");
    assert_eq!(as_micros_str_default(), "0");
}

#[test]
fn function_without_param() {
    assert_eq!(nothing(), "ok");
    assert_eq!(nothing_overload(NothingArgs), "ok");
    assert_eq!(nothing_default(), "ok");
}

#[test]
fn function_with_combined_defaults_and_struct_default_param() {
    assert_eq!(opt(None, None), (None, None));
    assert_eq!(opt_overload(OptArgs { x: None, y: None }), (None, None));
    assert_eq!(opt_overload(OptArgs::default()), (None, Some(1)));
    assert_eq!(
        opt_overload(OptArgs {
            x: Some(42),
            ..OptArgs::default()
        }),
        (Some(42), Some(1))
    );
    assert_eq!(opt_default(), (None, Some(1)));
}

#[test]
fn async_function() {
    pollster::block_on(async {
        assert_eq!(async_calc(3, 4).await, 12);
        assert_eq!(
            async_calc_overload(3, AsyncCalcArgs { factor: 4 }).await,
            12
        );
        assert_eq!(async_calc_overload(3, AsyncCalcArgs::default()).await, 15);
        assert_eq!(async_calc_default(3).await, 15);
    });
}

#[test]
fn impl_methods_and_receivers() {
    let calc = Calculator::new(50);
    assert_eq!(calc.base, 50);

    let calc_overload = Calculator::new_overload(CalculatorNewArgs { base: 200 });
    assert_eq!(calc_overload.base, 200);

    let calc_default = Calculator::new_default();
    assert_eq!(calc_default.base, 100);

    // &self method
    assert_eq!(calc.calculate(2, 5), 105);
    assert_eq!(
        calc.calculate_overload(2, CalculatorCalculateArgs { offset: 20 }),
        120
    );
    assert_eq!(calc.calculate_default(2), 110);

    // &mut self method
    let mut calc_mut = Calculator::new(10);
    calc_mut.add_assign(5, 2);
    assert_eq!(calc_mut.base, 20);

    calc_mut.add_assign_overload(5, CalculatorAddAssignArgs { step: 3 });
    assert_eq!(calc_mut.base, 35);

    calc_mut.add_assign_default(5);
    assert_eq!(calc_mut.base, 40);

    // self (by value) method
    assert_eq!(calc.into_total(5), 55);

    let calc2 = Calculator::new(50);
    assert_eq!(
        calc2.into_total_overload(CalculatorIntoTotalArgs { extra: 15 }),
        65
    );

    let calc3 = Calculator::new(50);
    assert_eq!(calc3.into_total_default(), 50);

    // async method in impl
    pollster::block_on(async {
        let calc = Calculator::new(10);
        assert_eq!(calc.async_eval(3, 3).await, 270);
        assert_eq!(
            calc.async_eval_overload(3, CalculatorAsyncEvalArgs { exponent: 4 })
                .await,
            810
        );
        assert_eq!(calc.async_eval_default(3).await, 90);
    });
}

#[test]
fn impl_generic_struct_and_methods() {
    let container: Container<i32> = Container::create_default();
    assert_eq!(container.item, 0);

    let container_custom = Container::create_overload(ContainerCreateArgs { item: 42 });
    assert_eq!(container_custom.item, 42);

    assert_eq!(
        container_custom.format_with("value:", "custom"),
        "value: custom"
    );
    assert_eq!(
        container_custom.format_with_overload("value:", ContainerFormatWithArgs { suffix: "custom" }),
        "value: custom"
    );
    assert_eq!(
        container_custom.format_with_default("value:"),
        "value: item"
    );
}

#[test]
fn trait_methods() {
    let worker = MathWorker;

    // &self method
    assert_eq!(worker.modulo(14, 4), 2);
    assert_eq!(worker.modulo_overload(14, MathOpsModuloArgs { b: 4 }), 2);
    assert_eq!(worker.modulo_overload(14, MathOpsModuloArgs::default()), 4);
    assert_eq!(worker.modulo_default(14), 4);

    // static associated function
    assert_eq!(MathWorker::power(3, 3), 27);
    assert_eq!(
        MathWorker::power_overload(3, MathOpsPowerArgs { exp: 4 }),
        81
    );
    assert_eq!(
        MathWorker::power_overload(3, MathOpsPowerArgs::default()),
        9
    );
    assert_eq!(MathWorker::power_default(3), 9);

    // default body in trait
    assert_eq!(worker.default_body_mult(10, 5), 50);
    assert_eq!(
        worker.default_body_mult_overload(10, MathOpsDefaultBodyMultArgs { factor: 3 }),
        30
    );
    assert_eq!(worker.default_body_mult_default(10), 20);
}

#[test]
fn generic_trait() {
    let rep = Replicator;
    assert_eq!(rep.transform("a", 3), vec!["a", "a", "a"]);
    assert_eq!(
        rep.transform_overload("a", TransformerTransformArgs { repeat: 2 }),
        vec!["a", "a"]
    );
    assert_eq!(rep.transform_default("a"), vec!["a"]);
}
