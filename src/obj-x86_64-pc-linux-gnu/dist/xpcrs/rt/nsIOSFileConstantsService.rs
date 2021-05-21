//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/system/nsIOSFileConstantsService.idl
//


/// `interface nsIOSFileConstantsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIOSFileConstantsService {
    vtable: *const nsIOSFileConstantsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIOSFileConstantsService.
unsafe impl XpCom for nsIOSFileConstantsService {
    const IID: nsIID = nsID(0xd6dd239f, 0x34d6, 0x4b34,
        [0xba, 0xa1, 0xf6, 0x9a, 0xb4, 0xa2, 0x0b, 0xc4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIOSFileConstantsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIOSFileConstantsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIOSFileConstantsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIOSFileConstantsService`.
    fn coerce_from(v: &nsIOSFileConstantsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIOSFileConstantsServiceCoerce for nsIOSFileConstantsService {
    #[inline]
    fn coerce_from(v: &nsIOSFileConstantsService) -> &Self {
        v
    }
}

impl nsIOSFileConstantsService {
    /// Cast this `nsIOSFileConstantsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIOSFileConstantsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIOSFileConstantsService {
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
impl<T: nsISupportsCoerce> nsIOSFileConstantsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIOSFileConstantsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIOSFileConstantsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIOSFileConstantsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void init (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub Init: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIOSFileConstantsService {

    /// ```text
    /// /**
    ///    * Inject module OS.Constants in the environment.
    ///    *
    ///    * This method must be called only from the main thread.
    ///    * Method is idempotent.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void init ();`
    const _Init: () = ();

}


