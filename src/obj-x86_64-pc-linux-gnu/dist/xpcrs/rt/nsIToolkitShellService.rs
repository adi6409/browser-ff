//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/shell/nsIToolkitShellService.idl
//


/// `interface nsIToolkitShellService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIToolkitShellService {
    vtable: *const nsIToolkitShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIToolkitShellService.
unsafe impl XpCom for nsIToolkitShellService {
    const IID: nsIID = nsID(0x9246cad6, 0x926a, 0x4c17,
        [0x88, 0xb0, 0xec, 0xba, 0x80, 0x78, 0xd1, 0x43]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIToolkitShellService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIToolkitShellService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIToolkitShellServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIToolkitShellService`.
    fn coerce_from(v: &nsIToolkitShellService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIToolkitShellServiceCoerce for nsIToolkitShellService {
    #[inline]
    fn coerce_from(v: &nsIToolkitShellService) -> &Self {
        v
    }
}

impl nsIToolkitShellService {
    /// Cast this `nsIToolkitShellService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIToolkitShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIToolkitShellService {
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
impl<T: nsISupportsCoerce> nsIToolkitShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIToolkitShellService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIToolkitShellService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIToolkitShellServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean isDefaultApplication (); */
    pub IsDefaultApplication: unsafe extern "system" fn (this: *const nsIToolkitShellService, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIToolkitShellService {

    /// ```text
    /// /**
    ///    * Determines whether or not this application is the default for the operating
    ///    * system. It is up to the application to determine how to answer this
    ///    * question but it would typically involve checking if it is registered as the
    ///    * default handler for web protocols and/or file types.
    ///    */
    /// ```
    ///

    /// `boolean isDefaultApplication ();`
    #[inline]
    pub unsafe fn IsDefaultApplication(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDefaultApplication)(self, _retval)
    }


}


