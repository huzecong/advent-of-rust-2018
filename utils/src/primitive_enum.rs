pub trait PrimitiveEnum: Sized {
    fn next_enum(&self) -> Option<Self>;
    fn prev_enum(&self) -> Option<Self>;
    fn first_enum() -> Self;
    fn last_enum() -> Self;
}

// Implementation reference: crate `enum_derive`
#[macro_export]
macro_rules! PrimitiveEnum {
// main part
    (
        () pub enum $name:ident { $($val:ident),* }
    ) => {
        PrimitiveEnum! { [pub] ($name) ($($val)*) }
    };

    (
        () enum $name:ident { $($val:ident),* }
    ) => {
        PrimitiveEnum! { [] ($name) ($($val)*) }
    };

    (
        [$($pub_:tt)?] ($name:ident) ($($val:ident)*)
    ) => {
        $($pub_)? impl PrimitiveEnum for $name {
            fn next_enum(&self) -> Option<Self> {
                PrimitiveEnum! { @next_enum ($name, self) ($($val)*) }
            }

            fn prev_enum(&self) -> Option<Self> {
                PrimitiveEnum! { @prev_enum ($name, self) ($($val)*) }
            }

            fn first_enum() -> Self {
                PrimitiveEnum! { @first_enum ($name) ($($val)*) }
            }

            fn last_enum() -> Self {
                PrimitiveEnum! { @last_enum ($name) ($($val)*) }
            }
        }
    };

// next_enum
	(
        @next_enum ($name:ident, $self_:ident) ($($val:ident)*)
    ) => {
        PrimitiveEnum! { @next_enum @arms ($name, $self_) ($($val)*) -> () }
    };

    (
        @next_enum @arms ($name:ident, $self_:ident) ($a:ident) -> ($($body:tt)*)
    ) => {
        match *$self_ {
            $($body)*
            $name::$a => ::std::option::Option::None,
        }
    };

    (
        @next_enum @arms ($name:ident, $self_:ident) ($a:ident $b:ident $($rest:ident)*) -> ($($body:tt)*)
    ) => {
        PrimitiveEnum! { @next_enum
            @arms ($name, $self_) ($b $($rest)*) -> (
                $($body)*
                $name::$a => ::std::option::Option::Some($name::$b),
            )
        }
    };

// prev_enum
    (
        @prev_enum ($name:ident, $self_:ident) ($($val:ident)*)
    ) => {
        PrimitiveEnum! { @prev_enum @arms ($name, $self_) (::std::option::Option::None) ($($val)*) -> () }
    };

    (
        @prev_enum @arms ($name:ident, $self_:ident) ($prev:expr) ($a:ident) -> ($($body:tt)*)
    ) => {
        match *$self_ {
            $($body)*
            $name::$a => $prev,
        }
    };

    (
        @prev_enum @arms ($name:ident, $self_:ident) ($prev:expr) ($a:ident $($rest:ident)+) -> ($($body:tt)*)
    ) => {
        PrimitiveEnum! { @prev_enum
            @arms ($name, $self_) (::std::option::Option::Some($name::$a)) ($($rest)*) -> (
                $($body)*
                $name::$a => $prev,
            )
        }
    };

// last_enum
	(
        @last_enum ($name:ident) ($a:ident $($rest:ident)+)
    ) => {
        PrimitiveEnum! { @last_enum ($name) ($($rest)*) }
    };

    (
        @last_enum ($name:ident) ($a:ident)
    ) => {
        $name::$a
    };

// first_enum
    (
        @first_enum ($name:ident) ($a:ident $($rest:ident)*)
    ) => {
        $name::$a
    };
}

pub fn next_enum<E: PrimitiveEnum>(e: &E) -> E {
    match e.next_enum() {
        Some(e) => e,
        None => E::first_enum(),
    }
}

pub fn prev_enum<E: PrimitiveEnum>(e: &E) -> E {
    match e.prev_enum() {
        Some(e) => e,
        None => E::last_enum(),
    }
}
