use crate::signal::Signal;

#[derive(Clone)]
struct _CounterMeta {
    str: String,
    numb: u32,
}

struct _Counter {
    meta: Signal<_CounterMeta>,
    count: Signal<i32>,
    name: Signal<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let counter = _Counter {
            meta: Signal::new(_CounterMeta {
                str: "initial".to_string(),
                numb: 0,
            }),
            count: Signal::new(0),
            name: Signal::new("Counter1".to_string()),
        };

        // Test initial values
        assert_eq!(counter.count.get(), 0);
        assert_eq!(counter.name.get(), "Counter1".to_string());
        assert_eq!(counter.meta.get().str, "initial".to_string());
        assert_eq!(counter.meta.get().numb, 0);

        // Update and test count
        counter.count.set(5);
        assert_eq!(counter.count.get(), 5);

        // Update and test name
        counter.name.set("UpdatedCounter".to_string());
        assert_eq!(counter.name.get(), "UpdatedCounter".to_string());

        // Update and test meta
        counter.meta.set(_CounterMeta {
            str: "updated".to_string(),
            numb: 42,
        });
        assert_eq!(counter.meta.get().str, "updated".to_string());
        assert_eq!(counter.meta.get().numb, 42);

    }
}

