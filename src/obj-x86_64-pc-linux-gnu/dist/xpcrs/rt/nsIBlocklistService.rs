//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/xpcom/system/nsIBlocklistService.idl
//


/// `interface nsIBlocklistService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIBlocklistService {
    vtable: *const nsIBlocklistServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIBlocklistService.
unsafe impl XpCom for nsIBlocklistService {
    const IID: nsIID = nsID(0xa6dcc76e, 0x9f62, 0x4cc1,
        [0xa4, 0x70, 0xb4, 0x83, 0xa1, 0xa6, 0xf0, 0x96]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIBlocklistService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIBlocklistService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIBlocklistServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIBlocklistService`.
    fn coerce_from(v: &nsIBlocklistService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIBlocklistServiceCoerce for nsIBlocklistService {
    #[inline]
    fn coerce_from(v: &nsIBlocklistService) -> &Self {
        v
    }
}

impl nsIBlocklistService {
    /// Cast this `nsIBlocklistService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIBlocklistServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIBlocklistService {
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
impl<T: nsISupportsCoerce> nsIBlocklistServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIBlocklistService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIBlocklistService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIBlocklistServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Promise getPluginBlocklistState (in nsIPluginTag plugin, [optional] in AString appVersion, [optional] in AString toolkitVersion); */
    /// Unable to generate binding because `specialtype promise unsupported`
    pub GetPluginBlocklistState: *const ::libc::c_void,

    /* readonly attribute boolean isLoaded; */
    pub GetIsLoaded: unsafe extern "system" fn (this: *const nsIBlocklistService, aIsLoaded: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIBlocklistService {

    pub const STATE_NOT_BLOCKED: i64 = 0;


    pub const STATE_SOFTBLOCKED: i64 = 1;


    pub const STATE_BLOCKED: i64 = 2;


    pub const STATE_OUTDATED: i64 = 3;


    pub const STATE_VULNERABLE_UPDATE_AVAILABLE: i64 = 4;


    pub const STATE_VULNERABLE_NO_UPDATE: i64 = 5;


    pub const STATE_MAX: i64 = 6;

    /// ```text
    /// /**
    ///    * Determine the blocklist state of a plugin
    ///    * @param   plugin
    ///    *          The plugin to get the state for
    ///    * @param   appVersion
    ///    *          The version of the application we are checking in the blocklist.
    ///    *          If this parameter is null, the version of the running application
    ///    *          is used.
    ///    * @param   toolkitVersion
    ///    *          The version of the toolkit we are checking in the blocklist.
    ///    *          If this parameter is null, the version of the running toolkit
    ///    *          is used.
    ///    * @returns Promise that resolves to the STATE constant.
    ///    */
    /// ```
    ///

    /// `Promise getPluginBlocklistState (in nsIPluginTag plugin, [optional] in AString appVersion, [optional] in AString toolkitVersion);`
    const _GetPluginBlocklistState: () = ();


    /// `readonly attribute boolean isLoaded;`
    #[inline]
    pub unsafe fn GetIsLoaded(&self, aIsLoaded: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsLoaded)(self, aIsLoaded)
    }


}


