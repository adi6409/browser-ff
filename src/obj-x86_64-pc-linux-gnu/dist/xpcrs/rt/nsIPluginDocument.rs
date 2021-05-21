//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginDocument.idl
//


/// `interface nsIPluginDocument : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPluginDocument {
    vtable: *const nsIPluginDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPluginDocument.
unsafe impl XpCom for nsIPluginDocument {
    const IID: nsIID = nsID(0xa93a0f0f, 0x24f0, 0x4206,
        [0xa2, 0x1b, 0x56, 0xa4, 0x3d, 0xcb, 0xdd, 0x88]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPluginDocument {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPluginDocument.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPluginDocumentCoerce {
    /// Cheaply cast a value of this type from a `nsIPluginDocument`.
    fn coerce_from(v: &nsIPluginDocument) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPluginDocumentCoerce for nsIPluginDocument {
    #[inline]
    fn coerce_from(v: &nsIPluginDocument) -> &Self {
        v
    }
}

impl nsIPluginDocument {
    /// Cast this `nsIPluginDocument` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPluginDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPluginDocument {
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
impl<T: nsISupportsCoerce> nsIPluginDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginDocument) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPluginDocument
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPluginDocumentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void print (); */
    pub Print: unsafe extern "system" fn (this: *const nsIPluginDocument) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPluginDocument {

    /// ```text
    /// /**
    ///   * Causes the plugin to print in full-page mode
    ///   */
    /// ```
    ///

    /// `void print ();`
    #[inline]
    pub unsafe fn Print(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Print)(self, )
    }


}


