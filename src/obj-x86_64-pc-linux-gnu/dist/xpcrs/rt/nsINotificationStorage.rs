//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/notification/nsINotificationStorage.idl
//


/// `interface nsINotificationStorageCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINotificationStorageCallback {
    vtable: *const nsINotificationStorageCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINotificationStorageCallback.
unsafe impl XpCom for nsINotificationStorageCallback {
    const IID: nsIID = nsID(0xc1622232, 0x259c, 0x43b0,
        [0xb5, 0x2e, 0x89, 0xc3, 0x9d, 0xcd, 0x97, 0x96]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINotificationStorageCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINotificationStorageCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINotificationStorageCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsINotificationStorageCallback`.
    fn coerce_from(v: &nsINotificationStorageCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINotificationStorageCallbackCoerce for nsINotificationStorageCallback {
    #[inline]
    fn coerce_from(v: &nsINotificationStorageCallback) -> &Self {
        v
    }
}

impl nsINotificationStorageCallback {
    /// Cast this `nsINotificationStorageCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINotificationStorageCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINotificationStorageCallback {
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
impl<T: nsISupportsCoerce> nsINotificationStorageCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINotificationStorageCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINotificationStorageCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINotificationStorageCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void handle (in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope); */
    pub Handle: unsafe extern "system" fn (this: *const nsINotificationStorageCallback, id: *const ::nsstring::nsAString, title: *const ::nsstring::nsAString, dir: *const ::nsstring::nsAString, lang: *const ::nsstring::nsAString, body: *const ::nsstring::nsAString, tag: *const ::nsstring::nsAString, icon: *const ::nsstring::nsAString, data: *const ::nsstring::nsAString, behavior: *const ::nsstring::nsAString, serviceWorkerRegistrationScope: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void done (); */
    pub Done: unsafe extern "system" fn (this: *const nsINotificationStorageCallback) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINotificationStorageCallback {

    /// ```text
    /// /**
    ///    * Callback function used to pass single notification back
    ///    * into C++ land for Notification.get return data.
    ///    *
    ///    * @param id: a uuid for this notification
    ///    * @param title: the notification title
    ///    * @param dir: the notification direction,
    ///    *             possible values are "ltr", "rtl", "auto"
    ///    * @param lang: the notification language
    ///    * @param body: the notification body
    ///    * @param tag: the notification tag
    ///    */
    /// ```
    ///

    /// `void handle (in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope);`
    #[inline]
    pub unsafe fn Handle(&self, id: *const ::nsstring::nsAString, title: *const ::nsstring::nsAString, dir: *const ::nsstring::nsAString, lang: *const ::nsstring::nsAString, body: *const ::nsstring::nsAString, tag: *const ::nsstring::nsAString, icon: *const ::nsstring::nsAString, data: *const ::nsstring::nsAString, behavior: *const ::nsstring::nsAString, serviceWorkerRegistrationScope: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Handle)(self, id, title, dir, lang, body, tag, icon, data, behavior, serviceWorkerRegistrationScope)
    }


    /// ```text
    /// /**
    ///    * Callback function used to notify C++ the we have returned
    ///    * all notification objects for this Notification.get call.
    ///    */
    /// ```
    ///

    /// `void done ();`
    #[inline]
    pub unsafe fn Done(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Done)(self, )
    }


}


/// `interface nsINotificationStorage : nsISupports`
///

/// ```text
/// /**
///  * Interface for notification persistence layer.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINotificationStorage {
    vtable: *const nsINotificationStorageVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINotificationStorage.
unsafe impl XpCom for nsINotificationStorage {
    const IID: nsIID = nsID(0x17f85e52, 0xfe57, 0x440e,
        [0x9b, 0xa1, 0x5c, 0x31, 0x2c, 0xa0, 0x2b, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINotificationStorage {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINotificationStorage.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINotificationStorageCoerce {
    /// Cheaply cast a value of this type from a `nsINotificationStorage`.
    fn coerce_from(v: &nsINotificationStorage) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINotificationStorageCoerce for nsINotificationStorage {
    #[inline]
    fn coerce_from(v: &nsINotificationStorage) -> &Self {
        v
    }
}

impl nsINotificationStorage {
    /// Cast this `nsINotificationStorage` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINotificationStorageCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINotificationStorage {
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
impl<T: nsISupportsCoerce> nsINotificationStorageCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINotificationStorage) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINotificationStorage
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINotificationStorageVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void put (in AString origin, in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString alertName, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope); */
    pub Put: unsafe extern "system" fn (this: *const nsINotificationStorage, origin: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, title: *const ::nsstring::nsAString, dir: *const ::nsstring::nsAString, lang: *const ::nsstring::nsAString, body: *const ::nsstring::nsAString, tag: *const ::nsstring::nsAString, icon: *const ::nsstring::nsAString, alertName: *const ::nsstring::nsAString, data: *const ::nsstring::nsAString, behavior: *const ::nsstring::nsAString, serviceWorkerRegistrationScope: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void get (in AString origin, in AString tag, in nsINotificationStorageCallback aCallback); */
    pub Get: unsafe extern "system" fn (this: *const nsINotificationStorage, origin: *const ::nsstring::nsAString, tag: *const ::nsstring::nsAString, aCallback: *const nsINotificationStorageCallback) -> ::nserror::nsresult,

    /* void getByID (in AString origin, in AString id, in nsINotificationStorageCallback aCallback); */
    pub GetByID: unsafe extern "system" fn (this: *const nsINotificationStorage, origin: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, aCallback: *const nsINotificationStorageCallback) -> ::nserror::nsresult,

    /* void delete (in AString origin, in AString id); */
    pub Delete: unsafe extern "system" fn (this: *const nsINotificationStorage, origin: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean canPut (in AString origin); */
    pub CanPut: unsafe extern "system" fn (this: *const nsINotificationStorage, origin: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINotificationStorage {

    /// ```text
    /// /**
    ///    * Add/replace a notification to the persistence layer.
    ///    *
    ///    * @param origin: the origin/app of this notification
    ///    * @param id: a uuid for this notification
    ///    * @param title: the notification title
    ///    * @param dir: the notification direction,
    ///    *             possible values are "ltr", "rtl", "auto"
    ///    * @param lang: the notification language
    ///    * @param body: the notification body
    ///    * @param tag: notification tag, will replace any existing
    ///    *             notifications with same origin/tag pair
    ///    * @param alertName: the alert identifier as used by system app.
    ///    *                   Stored in the database to avoid re-computing
    ///    *                   it. Built from origin and tag or id depending
    ///    *                   whether there is a tag defined.
    ///    * @param registrationID: Opaque string that identifies the service worker
    ///    *                        registration this Notification is associated with.
    ///    *                        May be empty. Only set for Notifications created by
    ///    *                        showNotification().
    ///    */
    /// ```
    ///

    /// `void put (in AString origin, in AString id, in AString title, in AString dir, in AString lang, in AString body, in AString tag, in AString icon, in AString alertName, in AString data, in AString behavior, in AString serviceWorkerRegistrationScope);`
    #[inline]
    pub unsafe fn Put(&self, origin: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, title: *const ::nsstring::nsAString, dir: *const ::nsstring::nsAString, lang: *const ::nsstring::nsAString, body: *const ::nsstring::nsAString, tag: *const ::nsstring::nsAString, icon: *const ::nsstring::nsAString, alertName: *const ::nsstring::nsAString, data: *const ::nsstring::nsAString, behavior: *const ::nsstring::nsAString, serviceWorkerRegistrationScope: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Put)(self, origin, id, title, dir, lang, body, tag, icon, alertName, data, behavior, serviceWorkerRegistrationScope)
    }


    /// ```text
    /// /**
    ///    * Retrieve a list of notifications.
    ///    *
    ///    * @param origin: the origin/app for which to fetch notifications from
    ///    * @param tag: used to fetch only a specific tag
    ///    * @param callback: nsINotificationStorageCallback, used for
    ///    *                  returning notifications objects
    ///    */
    /// ```
    ///

    /// `void get (in AString origin, in AString tag, in nsINotificationStorageCallback aCallback);`
    #[inline]
    pub unsafe fn Get(&self, origin: *const ::nsstring::nsAString, tag: *const ::nsstring::nsAString, aCallback: *const nsINotificationStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).Get)(self, origin, tag, aCallback)
    }


    /// ```text
    /// /**
    ///    * Retrieve a notification by ID.
    ///    *
    ///    * @param origin: the origin/app for which to fetch notifications.
    ///    * @param id: the id of the notification.
    ///    * @param callback: nsINotificationStorageCallback whose Handle method will
    ///    * be called *at most once* if the notification with that ID is found. Not
    ///    * called if that ID is not found. Done() will be called right after
    ///    * Handle().
    ///    */
    /// ```
    ///

    /// `void getByID (in AString origin, in AString id, in nsINotificationStorageCallback aCallback);`
    #[inline]
    pub unsafe fn GetByID(&self, origin: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString, aCallback: *const nsINotificationStorageCallback) -> ::nserror::nsresult {
        ((*self.vtable).GetByID)(self, origin, id, aCallback)
    }


    /// ```text
    /// /**
    ///    * Remove a notification from storage.
    ///    *
    ///    * @param origin: the origin/app to delete the notification from
    ///    * @param id: the uuid for the notification to delete
    ///    */
    /// ```
    ///

    /// `void delete (in AString origin, in AString id);`
    #[inline]
    pub unsafe fn Delete(&self, origin: *const ::nsstring::nsAString, id: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).Delete)(self, origin, id)
    }


    /// ```text
    /// /**
    ///    * Notifications are not supposed to be persistent, according to spec, at
    ///    * least for now. But we want to be able to have this behavior on B2G. Thus,
    ///    * this method will check if the origin sending the notifications is a valid
    ///    * registered app with a manifest or not. Hence, a webpage that has none
    ///    * will have its notification sent and available (via Notification.get())
    ///    * during the life time of the page.
    ///    *
    ///    * @param origin: Origin from which the notification is sent.
    ///    *
    ///    * @return boolean
    ///    */
    /// ```
    ///

    /// `boolean canPut (in AString origin);`
    #[inline]
    pub unsafe fn CanPut(&self, origin: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanPut)(self, origin, _retval)
    }


}


