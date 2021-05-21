//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/browser/components/shell/nsIGNOMEShellService.idl
//


/// `interface nsIGNOMEShellService : nsIShellService`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIGNOMEShellService {
    vtable: *const nsIGNOMEShellServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIGNOMEShellService.
unsafe impl XpCom for nsIGNOMEShellService {
    const IID: nsIID = nsID(0x2ce5c803, 0xedcd, 0x443d,
        [0x98, 0xeb, 0xce, 0xba, 0x86, 0xd0, 0x2d, 0x13]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIGNOMEShellService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIGNOMEShellService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIGNOMEShellServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIGNOMEShellService`.
    fn coerce_from(v: &nsIGNOMEShellService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIGNOMEShellServiceCoerce for nsIGNOMEShellService {
    #[inline]
    fn coerce_from(v: &nsIGNOMEShellService) -> &Self {
        v
    }
}

impl nsIGNOMEShellService {
    /// Cast this `nsIGNOMEShellService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIGNOMEShellServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIGNOMEShellService {
    type Target = nsIShellService;
    #[inline]
    fn deref(&self) -> &nsIShellService {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIShellServiceCoerce> nsIGNOMEShellServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIGNOMEShellService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIGNOMEShellService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIGNOMEShellServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIShellServiceVTable,

    /* readonly attribute boolean canSetDesktopBackground; */
    pub GetCanSetDesktopBackground: unsafe extern "system" fn (this: *const nsIGNOMEShellService, aCanSetDesktopBackground: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIGNOMEShellService {

    /// ```text
    /// /**
    ///    * Used to determine whether or not to offer "Set as desktop background"
    ///    * functionality. Even if shell service is available it is not
    ///    * guaranteed that it is able to set the background for every desktop
    ///    * which is especially true for Linux with its many different desktop
    ///    * environments.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean canSetDesktopBackground;`
    #[inline]
    pub unsafe fn GetCanSetDesktopBackground(&self, aCanSetDesktopBackground: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanSetDesktopBackground)(self, aCanSetDesktopBackground)
    }


}


