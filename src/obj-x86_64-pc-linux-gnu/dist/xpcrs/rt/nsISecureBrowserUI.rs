//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsISecureBrowserUI.idl
//


/// `interface nsISecureBrowserUI : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISecureBrowserUI {
    vtable: *const nsISecureBrowserUIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISecureBrowserUI.
unsafe impl XpCom for nsISecureBrowserUI {
    const IID: nsIID = nsID(0x718c662a, 0xf810, 0x4a80,
        [0xa6, 0xc9, 0x0b, 0x18, 0x10, 0xec, 0xad, 0xe2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISecureBrowserUI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISecureBrowserUI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISecureBrowserUICoerce {
    /// Cheaply cast a value of this type from a `nsISecureBrowserUI`.
    fn coerce_from(v: &nsISecureBrowserUI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISecureBrowserUICoerce for nsISecureBrowserUI {
    #[inline]
    fn coerce_from(v: &nsISecureBrowserUI) -> &Self {
        v
    }
}

impl nsISecureBrowserUI {
    /// Cast this `nsISecureBrowserUI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISecureBrowserUICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISecureBrowserUI {
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
impl<T: nsISupportsCoerce> nsISecureBrowserUICoerce for T {
    #[inline]
    fn coerce_from(v: &nsISecureBrowserUI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISecureBrowserUI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISecureBrowserUIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long state; */
    pub GetState: unsafe extern "system" fn (this: *const nsISecureBrowserUI, aState: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute bool isSecureContext; */
    pub GetIsSecureContext: unsafe extern "system" fn (this: *const nsISecureBrowserUI, aIsSecureContext: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute nsITransportSecurityInfo secInfo; */
    pub GetSecInfo: unsafe extern "system" fn (this: *const nsISecureBrowserUI, aSecInfo: *mut*const nsITransportSecurityInfo) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISecureBrowserUI {


    /// `readonly attribute unsigned long state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }



    /// `readonly attribute bool isSecureContext;`
    #[inline]
    pub unsafe fn GetIsSecureContext(&self, aIsSecureContext: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsSecureContext)(self, aIsSecureContext)
    }



    /// `readonly attribute nsITransportSecurityInfo secInfo;`
    #[inline]
    pub unsafe fn GetSecInfo(&self, aSecInfo: *mut*const nsITransportSecurityInfo) -> ::nserror::nsresult {
        ((*self.vtable).GetSecInfo)(self, aSecInfo)
    }


}


