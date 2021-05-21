//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/localstorage/nsILocalStorageManager.idl
//


/// `interface nsILocalStorageManager : nsISupports`
///

/// ```text
/// /**
///  * Methods specific to LocalStorage, see nsIDOMStorageManager for methods shared
///  * with SessionStorage.  Methods may migrate there as SessionStorage is
///  * overhauled.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsILocalStorageManager {
    vtable: *const nsILocalStorageManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsILocalStorageManager.
unsafe impl XpCom for nsILocalStorageManager {
    const IID: nsIID = nsID(0xd4f534da, 0x2744, 0x4db3,
        [0x87, 0x74, 0x8b, 0x18, 0x7c, 0x64, 0xad, 0xe9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsILocalStorageManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsILocalStorageManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsILocalStorageManagerCoerce {
    /// Cheaply cast a value of this type from a `nsILocalStorageManager`.
    fn coerce_from(v: &nsILocalStorageManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsILocalStorageManagerCoerce for nsILocalStorageManager {
    #[inline]
    fn coerce_from(v: &nsILocalStorageManager) -> &Self {
        v
    }
}

impl nsILocalStorageManager {
    /// Cast this `nsILocalStorageManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsILocalStorageManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsILocalStorageManager {
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
impl<T: nsISupportsCoerce> nsILocalStorageManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsILocalStorageManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsILocalStorageManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsILocalStorageManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean nextGenLocalStorageEnabled; */
    pub GetNextGenLocalStorageEnabled: unsafe extern "system" fn (this: *const nsILocalStorageManager, aNextGenLocalStorageEnabled: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] Promise preload (in nsIPrincipal aPrincipal); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub Preload: *const ::libc::c_void,

    /* [implicit_jscontext] Promise isPreloaded (in nsIPrincipal aPrincipal); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub IsPreloaded: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsILocalStorageManager {


    /// `readonly attribute boolean nextGenLocalStorageEnabled;`
    #[inline]
    pub unsafe fn GetNextGenLocalStorageEnabled(&self, aNextGenLocalStorageEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetNextGenLocalStorageEnabled)(self, aNextGenLocalStorageEnabled)
    }


    /// ```text
    /// /**
    ///    * Trigger preload of LocalStorage for the given principal.  For use by
    ///    * ContentParent::AboutToLoadHttpFtpDocumentForChild to maximize the
    ///    * amount of time we have to load the data off disk before the page might
    ///    * attempt to touch LocalStorage.
    ///    *
    ///    * This method will not create a QuotaManager-managed directory on disk if
    ///    * one does not already exist for the principal.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] Promise preload (in nsIPrincipal aPrincipal);`
    const _Preload: () = ();


    /// `[implicit_jscontext] Promise isPreloaded (in nsIPrincipal aPrincipal);`
    const _IsPreloaded: () = ();

}


