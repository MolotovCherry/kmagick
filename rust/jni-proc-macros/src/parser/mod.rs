use syn::{Attribute, Block, ImplItemMethod, ItemFn, ReturnType, Signature, Visibility};

mod fn_parser;
mod impl_parser;
mod attr_parser;

// re-export as public face of the module
pub use fn_parser::*;
pub use impl_parser::*;
pub use attr_parser::*;

// Make relevant fields in ItemFn and ImplItemMethod accessible
trait GenericFn {
    fn attrs(&self) -> &Vec<Attribute>;
    fn vis(&self) -> &Visibility;
    fn sig(&self) -> &Signature;
    fn block(&self) -> &Block;
    fn output(&self) -> &ReturnType;
    fn is_impl(&self) -> bool;
}

impl GenericFn for ImplItemMethod {
    fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    fn vis(&self) -> &Visibility {
        &self.vis
    }

    fn sig(&self) -> &Signature {
        &self.sig
    }

    fn block(&self) -> &Block {
        &self.block
    }

    fn output(&self) -> &ReturnType {
        self.output()
    }

    fn is_impl(&self) -> bool {
        true
    }
}

impl GenericFn for ItemFn {
    fn attrs(&self) -> &Vec<Attribute> {
        &self.attrs
    }

    fn vis(&self) -> &Visibility {
        &self.vis
    }

    fn sig(&self) -> &Signature {
        &self.sig
    }

    fn block(&self) -> &Block {
        &self.block
    }

    fn output(&self) -> &ReturnType {
        self.output()
    }

    fn is_impl(&self) -> bool {
        false
    }
}
