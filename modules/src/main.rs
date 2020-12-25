use deeply::nested::my_function as other_function;

fn main() {
    function();
    my_mod::function();

    // Public items, including those inside nested modules can be accessed  from outside the parent
    // module
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) items can be called from anywhere in the same crate
    my_mod::public_function_in_crate();

    // only visiable in my_mod modules and compile error
    // my_mod::nested::public_function_in_my_mod();

    // private items of a module cannot be directly accessed, evne if nested in a public module
    // my_mod::private_function();
    // my_mod::nested::private_function();
    // my_mod::private_nested::function();
    // my_mod::private_nested::restricted_function();

    let open_box = my::OpenBox {
        contents: "public information",
    };
    println!("The open box contains: {}", open_box.contents);

    // Public struct with private field cannot be constructed using field names
    /*
     * let closed_box = my::ClosedBox {
     *     contents: "classified information",
     * };
     */

    let _close_box = my::ClosedBox::new("classified information");

    // private field of a public struct cannot be accessed
    // println!("The closed box contains: {}", _close_box.contents);

    // Easier access to `deeply::nested::my_function`
    other_function();

    println!("Entering the block");
    {
        use crate::deeply::nested::my_function;

        // `use` bindings have a local scope
        // thi shadowing of `my_function()` is only in this block
        my_function();
        println!("Leaving the block");
    }

    my_function();

    scope::indirect_call();
}

// `super` and `self` keywords can be used in the path to remove ambiguity when accessing items and
// to prevent unnecessary hardcoding of paths

mod scope {
    fn function() {
        println!("called function()");
    }

    mod cool {
        pub fn function() {
            println!("called cool::function()");
        }
    }

    mod my {
        fn function() {
            println!("called my::function");
        }

        mod cool {
            pub fn function() {
                println!("called my::cool::function()");
            }
        }
    }

    pub fn indirect_call() {
        println!("called my::indirect_call()");

        // the following two function are the same function
        self::function();
        function();

        self::cool::function();
        super::function();
        {
            use crate::scope::cool::function as root_function;
            root_function();
        }
    }
}
// `use` declaration
fn my_function() {
    println!("called my_function()");
}

mod deeply {
    pub mod nested {
        pub fn my_function() {
            println!("called deeply nested my_function()");
        }
    }
}

// Structs visiability defaults to private

mod my {
    // A public struct with a public filed of generic type `T`
    pub struct OpenBox<T> {
        pub contents: T,
    }

    // A public struct with a private filed of generic type `T`
    #[allow(dead_code)]
    pub struct ClosedBox<T> {
        contents: T,
    }

    impl<T> ClosedBox<T> {
        // A public constructor method
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}

// A module is a collection of items: function, structs, traits, impl block and even other modules
// By default, the item in a module is PRIVATE, can be overridden with the `pub` modifer
//

mod my_mod {
    fn private_function() {
        println!("called mymod::private_function()");
    }

    pub fn function() {
        println!("called mymode::function()");
    }

    // Items can access other items in the same module
    pub fn indirect_access() {
        println!("called mymode::indirect_access()");
        private_function();
    }

    // modules can also be nested
    pub mod nested {
        pub fn function() {
            println!("called mymod::nested::function()");
        }

        #[allow(dead_code)]
        fn private_function() {
            println!("called mymod::nested::private_function()");
        }

        // Functions declared using `pub(in path)` syntax are only visable within the given path.
        // `path` must be a PARENT or ANCESTOR module
        pub(in crate::my_mod) fn public_function_in_my_mod() {
            println!("called my_mod::nested::public_function_in_my_mode()");
            public_function_in_nested();
        }

        pub(self) fn public_function_in_nested() {
            println!("called my_mod::nested::public_function_in_nested");
        }

        // Function declares using `pub(super)` syntax are only visiable within the parent module
        pub(super) fn public_function_in_super_mod() {
            println!("called my_mod::nested::public_function_in_super_mod");
        }
    }

    pub fn call_public_function_in_my_mod() {
        println!("called my_mod::call_public_function_in_my_mod");
        nested::public_function_in_my_mod();
        println!("> ");
        nested::public_function_in_super_mod();
    }

    pub(crate) fn public_function_in_crate() {
        println!("called my_mod::public_function_in_crate");
    }

    // private nested modules
    mod private_nested {
        #[allow(dead_code)]
        pub fn function() {
            println!("called my_mod::private_nested::function()");
        }

        #[allow(dead_code)]
        pub(crate) fn restricted_function() {
            println!("called my_mod::private_nested::restricted_function()");
        }
    }
}

fn function() {
    println!("called function()");
}
