//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/commandhandler/nsIControllerContext.idl
//


/// `interface nsIControllerContext : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIControllerContext {
    vtable: *const nsIControllerContextVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIControllerContext.
unsafe impl XpCom for nsIControllerContext {
    const IID: nsIID = nsID(0x47b82b60, 0xa36f, 0x4167,
        [0x80, 0x72, 0x6f, 0x42, 0x11, 0x51, 0xed, 0x50]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIControllerContext {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIControllerContext.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIControllerContextCoerce {
    /// Cheaply cast a value of this type from a `nsIControllerContext`.
    fn coerce_from(v: &nsIControllerContext) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIControllerContextCoerce for nsIControllerContext {
    #[inline]
    fn coerce_from(v: &nsIControllerContext) -> &Self {
        v
    }
}

impl nsIControllerContext {
    /// Cast this `nsIControllerContext` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIControllerContextCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIControllerContext {
    type Target = nsISupports;
    #[inline]
    fn deref(&self) -> &nsISupports {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsISupportsCoerce> nsIControllerContextCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIControllerContext) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIControllerContext
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIControllerContextVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setCommandContext (in nsISupports aCommandContext); */
    pub SetCommandContext: unsafe extern "system" fn (this: *const nsIControllerContext, aCommandContext: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIControllerContext {

    /// ```text
    /// /**
    ///    *  Set a context on this controller, which is passed
    ///    *  to commands to give them some context when they execute.
    ///    *
    ///    * @param aCommandContext  the context passed to commands.
    ///    *                        Note that this is *not* addreffed by the
    ///    *                        controller, and so needs to outlive it,
    ///    *                        or be nulled out.
    ///    */
    /// ```
    ///

    /// `void setCommandContext (in nsISupports aCommandContext);`
    #[inline]
    pub unsafe fn SetCommandContext(&self, aCommandContext: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).SetCommandContext)(self, aCommandContext)
    }


}


