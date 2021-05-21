//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/push/nsIPushNotifier.idl
//


/// `interface nsIPushNotifier : nsISupports`
///

/// ```text
/// /**
///  * Fires XPCOM observer notifications and service worker events for
///  * messages sent to push subscriptions.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushNotifier {
    vtable: *const nsIPushNotifierVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushNotifier.
unsafe impl XpCom for nsIPushNotifier {
    const IID: nsIID = nsID(0xb00dfdeb, 0x14e5, 0x425b,
        [0xad, 0xc7, 0xb5, 0x31, 0x44, 0x2e, 0x32, 0x16]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushNotifier {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushNotifier.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushNotifierCoerce {
    /// Cheaply cast a value of this type from a `nsIPushNotifier`.
    fn coerce_from(v: &nsIPushNotifier) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushNotifierCoerce for nsIPushNotifier {
    #[inline]
    fn coerce_from(v: &nsIPushNotifier) -> &Self {
        v
    }
}

impl nsIPushNotifier {
    /// Cast this `nsIPushNotifier` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushNotifierCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushNotifier {
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
impl<T: nsISupportsCoerce> nsIPushNotifierCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushNotifier) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushNotifier
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushNotifierVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void notifyPush (in ACString scope, in nsIPrincipal principal, in AString messageId); */
    pub NotifyPush: unsafe extern "system" fn (this: *const nsIPushNotifier, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal, messageId: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void notifyPushWithData (in ACString scope, in nsIPrincipal principal, in AString messageId, in Array<uint8_t> data); */
    pub NotifyPushWithData: unsafe extern "system" fn (this: *const nsIPushNotifier, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal, messageId: *const ::nsstring::nsAString, data: *const thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,

    /* void notifySubscriptionChange (in ACString scope, in nsIPrincipal principal); */
    pub NotifySubscriptionChange: unsafe extern "system" fn (this: *const nsIPushNotifier, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* void notifySubscriptionModified (in ACString scope, in nsIPrincipal principal); */
    pub NotifySubscriptionModified: unsafe extern "system" fn (this: *const nsIPushNotifier, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* void notifyError (in ACString scope, in nsIPrincipal principal, in AString message, in uint32_t flags); */
    pub NotifyError: unsafe extern "system" fn (this: *const nsIPushNotifier, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal, message: *const ::nsstring::nsAString, flags: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushNotifier {

    /// ```text
    /// /**
    ///    * Fires a `push-message` observer notification, and sends a `push` event to
    ///    * the service worker registered for the |scope|. |messageId| is an opaque ID
    ///    * used to report errors if the worker fails to handle the message.
    ///    */
    /// ```
    ///

    /// `void notifyPush (in ACString scope, in nsIPrincipal principal, in AString messageId);`
    #[inline]
    pub unsafe fn NotifyPush(&self, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal, messageId: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).NotifyPush)(self, scope, principal, messageId)
    }


    /// ```text
    /// /**
    ///    * Same as `notifyPush`, except the subject of the observer notification
    ///    * receives an `nsIPushMessage` instance containing the |data|. Service
    ///    * workers can access the |data| via the `PushMessageData` WebIDL interface.
    ///    */
    /// ```
    ///

    /// `void notifyPushWithData (in ACString scope, in nsIPrincipal principal, in AString messageId, in Array<uint8_t> data);`
    #[inline]
    pub unsafe fn NotifyPushWithData(&self, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal, messageId: *const ::nsstring::nsAString, data: *const thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).NotifyPushWithData)(self, scope, principal, messageId, data)
    }


    /// ```text
    /// /**
    ///    * Fires a `push-subscription-change` observer notification, and sends a
    ///    * `pushsubscriptionchange` event to the service worker registered for the
    ///    * |scope|.
    ///    */
    /// ```
    ///

    /// `void notifySubscriptionChange (in ACString scope, in nsIPrincipal principal);`
    #[inline]
    pub unsafe fn NotifySubscriptionChange(&self, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).NotifySubscriptionChange)(self, scope, principal)
    }


    /// ```text
    /// /**
    ///    * Fires a `push-subscription-modified` observer notification. Chrome code
    ///    * can listen for this notification to see when a subscription is added,
    ///    * updated, removed, or expired for any |scope|.
    ///    *
    ///    * This is useful for Dev Tools and debugging add-ons that passively observe
    ///    * when subscriptions are created or dropped. Other callers should listen for
    ///    * `push-subscription-change` and resubscribe instead.
    ///    */
    /// ```
    ///

    /// `void notifySubscriptionModified (in ACString scope, in nsIPrincipal principal);`
    #[inline]
    pub unsafe fn NotifySubscriptionModified(&self, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).NotifySubscriptionModified)(self, scope, principal)
    }



    /// `void notifyError (in ACString scope, in nsIPrincipal principal, in AString message, in uint32_t flags);`
    #[inline]
    pub unsafe fn NotifyError(&self, scope: *const ::nsstring::nsACString, principal: *const nsIPrincipal, message: *const ::nsstring::nsAString, flags: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).NotifyError)(self, scope, principal, message, flags)
    }


}


/// `interface nsIPushData : nsISupports`
///

/// ```text
/// /**
///  * Provides methods for retrieving push message data in different formats.
///  * This interface resembles the `PushMessageData` WebIDL interface.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushData {
    vtable: *const nsIPushDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushData.
unsafe impl XpCom for nsIPushData {
    const IID: nsIID = nsID(0xdfc4f151, 0xcead, 0x40df,
        [0x8e, 0xb7, 0x7a, 0x7a, 0x67, 0xc5, 0x4b, 0x16]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushDataCoerce {
    /// Cheaply cast a value of this type from a `nsIPushData`.
    fn coerce_from(v: &nsIPushData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushDataCoerce for nsIPushData {
    #[inline]
    fn coerce_from(v: &nsIPushData) -> &Self {
        v
    }
}

impl nsIPushData {
    /// Cast this `nsIPushData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushData {
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
impl<T: nsISupportsCoerce> nsIPushDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* AString text (); */
    pub Text: unsafe extern "system" fn (this: *const nsIPushData, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* [implicit_jscontext] jsval json (); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub Json: *const ::libc::c_void,

    /* Array<uint8_t> binary (); */
    pub Binary: unsafe extern "system" fn (this: *const nsIPushData, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushData {

    /// ```text
    /// /** Extracts the data as a UTF-8 text string. */
    /// ```
    ///

    /// `AString text ();`
    #[inline]
    pub unsafe fn Text(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Text)(self, _retval)
    }


    /// ```text
    /// /** Extracts the data as a JSON value. */
    /// ```
    ///

    /// `[implicit_jscontext] jsval json ();`
    const _Json: () = ();

    /// ```text
    /// /** Extracts the raw binary data. */
    /// ```
    ///

    /// `Array<uint8_t> binary ();`
    #[inline]
    pub unsafe fn Binary(&self, _retval: *mut thin_vec::ThinVec<uint8_t>) -> ::nserror::nsresult {
        ((*self.vtable).Binary)(self, _retval)
    }


}


/// `interface nsIPushMessage : nsISupports`
///

/// ```text
/// /**
///  * The subject of a `push-message` observer notification. |data| may be |null|
///  * for messages without data.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPushMessage {
    vtable: *const nsIPushMessageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPushMessage.
unsafe impl XpCom for nsIPushMessage {
    const IID: nsIID = nsID(0xb9d063ca, 0x0e3f, 0x4fee,
        [0xbe, 0x4b, 0xea, 0x91, 0x03, 0x26, 0x34, 0x33]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPushMessage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPushMessage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPushMessageCoerce {
    /// Cheaply cast a value of this type from a `nsIPushMessage`.
    fn coerce_from(v: &nsIPushMessage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPushMessageCoerce for nsIPushMessage {
    #[inline]
    fn coerce_from(v: &nsIPushMessage) -> &Self {
        v
    }
}

impl nsIPushMessage {
    /// Cast this `nsIPushMessage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPushMessageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPushMessage {
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
impl<T: nsISupportsCoerce> nsIPushMessageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPushMessage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPushMessage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPushMessageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIPushMessage, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute nsIPushData data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIPushMessage, aData: *mut *const nsIPushData) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPushMessage {


    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }



    /// `readonly attribute nsIPushData data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut *const nsIPushData) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


}


