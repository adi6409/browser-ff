//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/cookie/nsICookieService.idl
//


/// `interface nsICookieTransactionCallback : nsISupports`
///

/// ```text
/// /**
///  * @see nsICookieService::runInTransaction
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICookieTransactionCallback {
    vtable: *const nsICookieTransactionCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICookieTransactionCallback.
unsafe impl XpCom for nsICookieTransactionCallback {
    const IID: nsIID = nsID(0x0fc41ffb, 0xf1b7, 0x42d9,
        [0x9a, 0x42, 0x8d, 0xc4, 0x20, 0xc1, 0x58, 0xc1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICookieTransactionCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICookieTransactionCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICookieTransactionCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsICookieTransactionCallback`.
    fn coerce_from(v: &nsICookieTransactionCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICookieTransactionCallbackCoerce for nsICookieTransactionCallback {
    #[inline]
    fn coerce_from(v: &nsICookieTransactionCallback) -> &Self {
        v
    }
}

impl nsICookieTransactionCallback {
    /// Cast this `nsICookieTransactionCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICookieTransactionCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICookieTransactionCallback {
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
impl<T: nsISupportsCoerce> nsICookieTransactionCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieTransactionCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICookieTransactionCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICookieTransactionCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void callback (); */
    pub Callback: unsafe extern "system" fn (this: *const nsICookieTransactionCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICookieTransactionCallback {


    /// `void callback ();`
    #[inline]
    pub unsafe fn Callback(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Callback)(self, )
    }


}


/// `interface nsICookieService : nsISupports`
///

/// ```text
/// /**
///  * nsICookieService
///  *
///  * Provides methods for setting and getting cookies in the context of a
///  * page load.  See nsICookieManager for methods to manipulate the cookie
///  * database directly.  This separation of interface is mainly historical.
///  *
///  * This service broadcasts the notifications detailed below when the cookie
///  * list is changed, or a cookie is rejected.
///  *
///  * NOTE: observers of these notifications *must* not attempt to change profile
///  *       or switch into or out of private browsing mode from within the
///  *       observer. Doing so will cause undefined behavior. Mutating the cookie
///  *       list (e.g. by calling methods on nsICookieService and friends) is
///  *       allowed, but beware that there may be pending notifications you haven't
///  *       seen yet -- for instance, a "batch-deleted" notification will likely be
///  *       immediately followed by "added". You may check the state of the cookie
///  *       list to determine if this is the case.
///  *
///  * topic  : "cookie-changed"
///  *          broadcast whenever the cookie list changes in some way. see
///  *          explanation of data strings below.
///  * subject: see below.
///  * data   : "deleted"
///  *          a cookie was deleted. the subject is an nsICookie representing
///  *          the deleted cookie.
///  *          "added"
///  *          a cookie was added. the subject is an nsICookie representing
///  *          the added cookie.
///  *          "changed"
///  *          a cookie was changed. the subject is an nsICookie representing
///  *          the new cookie. (note that host, path, and name are invariant
    ///  *          for a given cookie; other parameters may change.)
///  *          "batch-deleted"
///  *          a set of cookies was purged (typically, because they have either
    ///  *          expired or because the cookie list has grown too large). The subject
///  *          is an nsIArray of nsICookie's representing the deleted cookies.
///  *          Note that the array could contain a single cookie.
///  *          "cleared"
///  *          the entire cookie list was cleared. the subject is null.
///  *
///  * topic  : "cookie-rejected"
///  *          broadcast whenever a cookie was rejected from being set as a
///  *          result of user prefs.
///  * subject: an nsIURI interface pointer representing the URI that attempted
///  *          to set the cookie.
///  * data   : none.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsICookieService {
    vtable: *const nsICookieServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsICookieService.
unsafe impl XpCom for nsICookieService {
    const IID: nsIID = nsID(0x1e94e283, 0x2811, 0x4f43,
        [0xb9, 0x47, 0xd2, 0x2b, 0x15, 0x49, 0xd8, 0x24]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsICookieService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsICookieService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsICookieServiceCoerce {
    /// Cheaply cast a value of this type from a `nsICookieService`.
    fn coerce_from(v: &nsICookieService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsICookieServiceCoerce for nsICookieService {
    #[inline]
    fn coerce_from(v: &nsICookieService) -> &Self {
        v
    }
}

impl nsICookieService {
    /// Cast this `nsICookieService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsICookieServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsICookieService {
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
impl<T: nsISupportsCoerce> nsICookieServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsICookieService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsICookieService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsICookieServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* ACString getCookieStringFromDocument (in Document aDocument); */
    pub GetCookieStringFromDocument: unsafe extern "system" fn (this: *const nsICookieService, aDocument: *const libc::c_void, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* ACString getCookieStringFromHttp (in nsIURI aURI, in nsIChannel aChannel); */
    pub GetCookieStringFromHttp: unsafe extern "system" fn (this: *const nsICookieService, aURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setCookieStringFromDocument (in Document aDocument, in ACString aCookie); */
    pub SetCookieStringFromDocument: unsafe extern "system" fn (this: *const nsICookieService, aDocument: *const libc::c_void, aCookie: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setCookieStringFromHttp (in nsIURI aURI, in ACString aCookie, in nsIChannel aChannel); */
    pub SetCookieStringFromHttp: unsafe extern "system" fn (this: *const nsICookieService, aURI: *const nsIURI, aCookie: *const ::nsstring::nsACString, aChannel: *const nsIChannel) -> ::nserror::nsresult,

    /* void runInTransaction (in nsICookieTransactionCallback aCallback); */
    pub RunInTransaction: unsafe extern "system" fn (this: *const nsICookieService, aCallback: *const nsICookieTransactionCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsICookieService {

    pub const BEHAVIOR_ACCEPT: i64 = 0;


    pub const BEHAVIOR_REJECT_FOREIGN: i64 = 1;


    pub const BEHAVIOR_REJECT: i64 = 2;


    pub const BEHAVIOR_LIMIT_FOREIGN: i64 = 3;


    pub const BEHAVIOR_REJECT_TRACKER: i64 = 4;


    pub const BEHAVIOR_REJECT_TRACKER_AND_PARTITION_FOREIGN: i64 = 5;


    pub const BEHAVIOR_LAST: i64 = 5;


    pub const ACCEPT_NORMALLY: i64 = 0;


    pub const ACCEPT_SESSION: i64 = 2;


    /// `ACString getCookieStringFromDocument (in Document aDocument);`
    #[inline]
    pub unsafe fn GetCookieStringFromDocument(&self, aDocument: *const libc::c_void, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCookieStringFromDocument)(self, aDocument, _retval)
    }



    /// `ACString getCookieStringFromHttp (in nsIURI aURI, in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn GetCookieStringFromHttp(&self, aURI: *const nsIURI, aChannel: *const nsIChannel, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetCookieStringFromHttp)(self, aURI, aChannel, _retval)
    }



    /// `void setCookieStringFromDocument (in Document aDocument, in ACString aCookie);`
    #[inline]
    pub unsafe fn SetCookieStringFromDocument(&self, aDocument: *const libc::c_void, aCookie: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetCookieStringFromDocument)(self, aDocument, aCookie)
    }



    /// `void setCookieStringFromHttp (in nsIURI aURI, in ACString aCookie, in nsIChannel aChannel);`
    #[inline]
    pub unsafe fn SetCookieStringFromHttp(&self, aURI: *const nsIURI, aCookie: *const ::nsstring::nsACString, aChannel: *const nsIChannel) -> ::nserror::nsresult {
        ((*self.vtable).SetCookieStringFromHttp)(self, aURI, aCookie, aChannel)
    }



    /// `void runInTransaction (in nsICookieTransactionCallback aCallback);`
    #[inline]
    pub unsafe fn RunInTransaction(&self, aCallback: *const nsICookieTransactionCallback) -> ::nserror::nsresult {
        ((*self.vtable).RunInTransaction)(self, aCallback)
    }


}


