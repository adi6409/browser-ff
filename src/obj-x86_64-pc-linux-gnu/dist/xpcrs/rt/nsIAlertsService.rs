//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/alerts/nsIAlertsService.idl
//


/// `interface nsIAlertNotificationImageListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAlertNotificationImageListener {
    vtable: *const nsIAlertNotificationImageListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAlertNotificationImageListener.
unsafe impl XpCom for nsIAlertNotificationImageListener {
    const IID: nsIID = nsID(0xa71a637d, 0xde1d, 0x47c6,
        [0xa8, 0xd2, 0xc6, 0x0b, 0x25, 0x96, 0xf4, 0x71]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAlertNotificationImageListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAlertNotificationImageListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAlertNotificationImageListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIAlertNotificationImageListener`.
    fn coerce_from(v: &nsIAlertNotificationImageListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAlertNotificationImageListenerCoerce for nsIAlertNotificationImageListener {
    #[inline]
    fn coerce_from(v: &nsIAlertNotificationImageListener) -> &Self {
        v
    }
}

impl nsIAlertNotificationImageListener {
    /// Cast this `nsIAlertNotificationImageListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAlertNotificationImageListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAlertNotificationImageListener {
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
impl<T: nsISupportsCoerce> nsIAlertNotificationImageListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertNotificationImageListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAlertNotificationImageListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAlertNotificationImageListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onImageReady (in nsISupports aUserData, in imgIRequest aRequest); */
    pub OnImageReady: unsafe extern "system" fn (this: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports, aRequest: *const imgIRequest) -> ::nserror::nsresult,

    /* void onImageMissing (in nsISupports aUserData); */
    pub OnImageMissing: unsafe extern "system" fn (this: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAlertNotificationImageListener {

    /// ```text
    /// /**
    ///    * Called when the image finishes loading.
    ///    *
    ///    * @param aUserData An opaque parameter passed to |loadImage|.
    ///    * @param aRequest  The image request.
    ///    */
    /// ```
    ///

    /// `void onImageReady (in nsISupports aUserData, in imgIRequest aRequest);`
    #[inline]
    pub unsafe fn OnImageReady(&self, aUserData: *const nsISupports, aRequest: *const imgIRequest) -> ::nserror::nsresult {
        ((*self.vtable).OnImageReady)(self, aUserData, aRequest)
    }


    /// ```text
    /// /**
    ///    * Called if the alert doesn't have an image, or if the image request times
    ///    * out or fails.
    ///    *
    ///    * @param aUserData An opaque parameter passed to |loadImage|.
    ///    */
    /// ```
    ///

    /// `void onImageMissing (in nsISupports aUserData);`
    #[inline]
    pub unsafe fn OnImageMissing(&self, aUserData: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).OnImageMissing)(self, aUserData)
    }


}


/// `interface nsIAlertNotification : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAlertNotification {
    vtable: *const nsIAlertNotificationVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAlertNotification.
unsafe impl XpCom for nsIAlertNotification {
    const IID: nsIID = nsID(0xcf2e4cb6, 0x4b8f, 0x4eca,
        [0xae, 0xa9, 0xd5, 0x1a, 0x8f, 0x9f, 0x7a, 0x50]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAlertNotification {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAlertNotification.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAlertNotificationCoerce {
    /// Cheaply cast a value of this type from a `nsIAlertNotification`.
    fn coerce_from(v: &nsIAlertNotification) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAlertNotificationCoerce for nsIAlertNotification {
    #[inline]
    fn coerce_from(v: &nsIAlertNotification) -> &Self {
        v
    }
}

impl nsIAlertNotification {
    /// Cast this `nsIAlertNotification` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAlertNotificationCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAlertNotification {
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
impl<T: nsISupportsCoerce> nsIAlertNotificationCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertNotification) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAlertNotification
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAlertNotificationVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
    pub Init: unsafe extern "system" fn (this: *const nsIAlertNotification, aName: *const ::nsstring::nsAString, aImageURL: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aText: *const ::nsstring::nsAString, aTextClickable: bool, aCookie: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aPrincipal: *const nsIPrincipal, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIAlertNotification, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString imageURL; */
    pub GetImageURL: unsafe extern "system" fn (this: *const nsIAlertNotification, aImageURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsIAlertNotification, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString text; */
    pub GetText: unsafe extern "system" fn (this: *const nsIAlertNotification, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean textClickable; */
    pub GetTextClickable: unsafe extern "system" fn (this: *const nsIAlertNotification, aTextClickable: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString cookie; */
    pub GetCookie: unsafe extern "system" fn (this: *const nsIAlertNotification, aCookie: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString dir; */
    pub GetDir: unsafe extern "system" fn (this: *const nsIAlertNotification, aDir: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString lang; */
    pub GetLang: unsafe extern "system" fn (this: *const nsIAlertNotification, aLang: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString data; */
    pub GetData: unsafe extern "system" fn (this: *const nsIAlertNotification, aData: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute nsIPrincipal principal; */
    pub GetPrincipal: unsafe extern "system" fn (this: *const nsIAlertNotification, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* readonly attribute nsIURI URI; */
    pub GetURI: unsafe extern "system" fn (this: *const nsIAlertNotification, aURI: *mut*const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute boolean inPrivateBrowsing; */
    pub GetInPrivateBrowsing: unsafe extern "system" fn (this: *const nsIAlertNotification, aInPrivateBrowsing: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean requireInteraction; */
    pub GetRequireInteraction: unsafe extern "system" fn (this: *const nsIAlertNotification, aRequireInteraction: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean actionable; */
    pub GetActionable: unsafe extern "system" fn (this: *const nsIAlertNotification, aActionable: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute AString source; */
    pub GetSource: unsafe extern "system" fn (this: *const nsIAlertNotification, aSource: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData); */
    pub LoadImage: unsafe extern "system" fn (this: *const nsIAlertNotification, aTimeout: u32, aListener: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports, _retval: *mut*const nsICancelable) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAlertNotification {

    /// ```text
    /// /** Initializes an alert notification. */
    /// ```
    ///

    /// `void init ([optional] in AString aName, [optional] in AString aImageURL, [optional] in AString aTitle, [optional] in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction);`
    #[inline]
    pub unsafe fn Init(&self, aName: *const ::nsstring::nsAString, aImageURL: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aText: *const ::nsstring::nsAString, aTextClickable: bool, aCookie: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aPrincipal: *const nsIPrincipal, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aName, aImageURL, aTitle, aText, aTextClickable, aCookie, aDir, aLang, aData, aPrincipal, aInPrivateBrowsing, aRequireInteraction)
    }


    /// ```text
    /// /**
    ///    * The name of the notification. On Android, the name is hashed and used as
    ///    * a notification ID. Notifications will replace previous notifications with
    ///    * the same name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * A URL identifying the image to put in the alert. The OS X backend limits
    ///    * the amount of time it will wait for the image to load to six seconds. After
    ///    * that time, the alert will show without an image.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString imageURL;`
    #[inline]
    pub unsafe fn GetImageURL(&self, aImageURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetImageURL)(self, aImageURL)
    }


    /// ```text
    /// /** The title for the alert. */
    /// ```
    ///

    /// `readonly attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }


    /// ```text
    /// /** The contents of the alert. */
    /// ```
    ///

    /// `readonly attribute AString text;`
    #[inline]
    pub unsafe fn GetText(&self, aText: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetText)(self, aText)
    }


    /// ```text
    /// /**
    ///    * Controls the click behavior. If true, the alert listener will be notified
    ///    * when the user clicks on the alert.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean textClickable;`
    #[inline]
    pub unsafe fn GetTextClickable(&self, aTextClickable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetTextClickable)(self, aTextClickable)
    }


    /// ```text
    /// /**
    ///    * An opaque cookie that will be passed to the alert listener for each
    ///    * callback.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString cookie;`
    #[inline]
    pub unsafe fn GetCookie(&self, aCookie: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetCookie)(self, aCookie)
    }


    /// ```text
    /// /**
    ///    * Bidi override for the title and contents. Valid values are "auto", "ltr",
    ///    * or "rtl". Ignored if the backend doesn't support localization.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString dir;`
    #[inline]
    pub unsafe fn GetDir(&self, aDir: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDir)(self, aDir)
    }


    /// ```text
    /// /**
    ///    * Language of the title and text. Ignored if the backend doesn't support
    ///    * localization.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString lang;`
    #[inline]
    pub unsafe fn GetLang(&self, aLang: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetLang)(self, aLang)
    }


    /// ```text
    /// /**
    ///    * A Base64-encoded structured clone buffer containing data associated with
    ///    * this alert. Only used for web notifications. Chrome callers should use a
    ///    * cookie instead.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString data;`
    #[inline]
    pub unsafe fn GetData(&self, aData: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aData)
    }


    /// ```text
    /// /**
    ///    * The principal of the page that created the alert. Used for IPC security
    ///    * checks, and to determine whether the alert is actionable.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIPrincipal principal;`
    #[inline]
    pub unsafe fn GetPrincipal(&self, aPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetPrincipal)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///    * The URI of the page that created the alert. |null| if the alert is not
    ///    * actionable.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIURI URI;`
    #[inline]
    pub unsafe fn GetURI(&self, aURI: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetURI)(self, aURI)
    }


    /// ```text
    /// /**
    ///    * Controls the image loading behavior. If true, the image request will be
    ///    * loaded anonymously (without cookies or authorization tokens).
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean inPrivateBrowsing;`
    #[inline]
    pub unsafe fn GetInPrivateBrowsing(&self, aInPrivateBrowsing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInPrivateBrowsing)(self, aInPrivateBrowsing)
    }


    /// ```text
    /// /**
    ///    * Indicates that the notification should remain readily available until
    ///    * the user activates or dismisses the notification.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean requireInteraction;`
    #[inline]
    pub unsafe fn GetRequireInteraction(&self, aRequireInteraction: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRequireInteraction)(self, aRequireInteraction)
    }


    /// ```text
    /// /**
    ///    * Indicates whether this alert should show the source string and action
    ///    * buttons. False for system alerts (which can omit the principal), or
    ///    * expanded, system, and null principals.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean actionable;`
    #[inline]
    pub unsafe fn GetActionable(&self, aActionable: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetActionable)(self, aActionable)
    }


    /// ```text
    /// /**
    ///    * The host and port of the originating page, or an empty string if the alert
    ///    * is not actionable.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString source;`
    #[inline]
    pub unsafe fn GetSource(&self, aSource: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSource)(self, aSource)
    }


    /// ```text
    /// /**
    ///    * Loads the image associated with this alert.
    ///    *
    ///    * @param aTimeout  The number of milliseconds to wait before cancelling the
    ///    *                  image request. If zero, there is no timeout.
    ///    * @param aListener An |nsIAlertNotificationImageListener| implementation,
    ///    *                  notified when the image loads. The listener is kept alive
    ///    *                  until the request completes.
    ///    * @param aUserData An opaque parameter passed to the listener's methods.
    ///    *                  Not used by the libnotify backend, but the OS X backend
    ///    *                  passes the pending notification.
    ///    */
    /// ```
    ///

    /// `nsICancelable loadImage (in unsigned long aTimeout, in nsIAlertNotificationImageListener aListener, [optional] in nsISupports aUserData);`
    #[inline]
    pub unsafe fn LoadImage(&self, aTimeout: u32, aListener: *const nsIAlertNotificationImageListener, aUserData: *const nsISupports, _retval: *mut*const nsICancelable) -> ::nserror::nsresult {
        ((*self.vtable).LoadImage)(self, aTimeout, aListener, aUserData, _retval)
    }


}


/// `interface nsIAlertsService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAlertsService {
    vtable: *const nsIAlertsServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAlertsService.
unsafe impl XpCom for nsIAlertsService {
    const IID: nsIID = nsID(0xf7a36392, 0xd98b, 0x4141,
        [0xa7, 0xd7, 0x4e, 0x46, 0x64, 0x26, 0x84, 0xe3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAlertsService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAlertsService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAlertsServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIAlertsService`.
    fn coerce_from(v: &nsIAlertsService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAlertsServiceCoerce for nsIAlertsService {
    #[inline]
    fn coerce_from(v: &nsIAlertsService) -> &Self {
        v
    }
}

impl nsIAlertsService {
    /// Cast this `nsIAlertsService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAlertsServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAlertsService {
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
impl<T: nsISupportsCoerce> nsIAlertsServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAlertsService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAlertsServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
    pub ShowPersistentNotification: unsafe extern "system" fn (this: *const nsIAlertsService, aPersistentData: *const ::nsstring::nsAString, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver) -> ::nserror::nsresult,

    /* void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener); */
    pub ShowAlert: unsafe extern "system" fn (this: *const nsIAlertsService, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver) -> ::nserror::nsresult,

    /* void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction); */
    pub ShowAlertNotification: unsafe extern "system" fn (this: *const nsIAlertsService, aImageURL: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aText: *const ::nsstring::nsAString, aTextClickable: bool, aCookie: *const ::nsstring::nsAString, aAlertListener: *const nsIObserver, aName: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aPrincipal: *const nsIPrincipal, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> ::nserror::nsresult,

    /* void closeAlert ([optional] in AString aName); */
    pub CloseAlert: unsafe extern "system" fn (this: *const nsIAlertsService, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAlertsService {


    /// `void showPersistentNotification (in AString aPersistentData, in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener);`
    #[inline]
    pub unsafe fn ShowPersistentNotification(&self, aPersistentData: *const ::nsstring::nsAString, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).ShowPersistentNotification)(self, aPersistentData, aAlert, aAlertListener)
    }



    /// `void showAlert (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener);`
    #[inline]
    pub unsafe fn ShowAlert(&self, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver) -> ::nserror::nsresult {
        ((*self.vtable).ShowAlert)(self, aAlert, aAlertListener)
    }


    /// ```text
    /// /**
    ///    * Initializes and shows an |nsIAlertNotification| with the given parameters.
    ///    *
    ///    * @param aAlertListener Used for callbacks. May be null if the caller
    ///    *                       doesn't care about callbacks.
    ///    * @see nsIAlertNotification for descriptions of all other parameters.
    ///    * @throws NS_ERROR_NOT_AVAILABLE If the notification cannot be displayed.
    ///    *
    ///    * The following arguments will be passed to the alertListener's observe()
    ///    * method:
    ///    *   subject - null
    ///    *   topic   - "alertfinished" when the alert goes away
    ///    *             "alertdisablecallback" when alerts should be disabled for the principal
    ///    *             "alertsettingscallback" when alert settings should be opened
    ///    *             "alertclickcallback" when the text is clicked
    ///    *             "alertshow" when the alert is shown
    ///    *   data    - the value of the cookie parameter passed to showAlertNotification.
    ///    *
    ///    * @note Depending on current circumstances (if the user's in a fullscreen
        ///    *       application, for instance), the alert might not be displayed at all.
    ///    *       In that case, if an alert listener is passed in it will receive the
    ///    *       "alertfinished" notification immediately.
    ///    */
    /// ```
    ///

    /// `void showAlertNotification (in AString aImageURL, in AString aTitle, in AString aText, [optional] in boolean aTextClickable, [optional] in AString aCookie, [optional] in nsIObserver aAlertListener, [optional] in AString aName, [optional] in AString aDir, [optional] in AString aLang, [optional] in AString aData, [optional] in nsIPrincipal aPrincipal, [optional] in boolean aInPrivateBrowsing, [optional] in boolean aRequireInteraction);`
    #[inline]
    pub unsafe fn ShowAlertNotification(&self, aImageURL: *const ::nsstring::nsAString, aTitle: *const ::nsstring::nsAString, aText: *const ::nsstring::nsAString, aTextClickable: bool, aCookie: *const ::nsstring::nsAString, aAlertListener: *const nsIObserver, aName: *const ::nsstring::nsAString, aDir: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aData: *const ::nsstring::nsAString, aPrincipal: *const nsIPrincipal, aInPrivateBrowsing: bool, aRequireInteraction: bool) -> ::nserror::nsresult {
        ((*self.vtable).ShowAlertNotification)(self, aImageURL, aTitle, aText, aTextClickable, aCookie, aAlertListener, aName, aDir, aLang, aData, aPrincipal, aInPrivateBrowsing, aRequireInteraction)
    }


    /// ```text
    /// /**
    ///    * Close alerts created by the service.
    ///    *
    ///    * @param aName          The name of the notification to close. If no name
    ///    *                       is provided then only a notification created with
    ///    *                       no name (if any) will be closed.
    ///    */
    /// ```
    ///

    /// `void closeAlert ([optional] in AString aName);`
    #[inline]
    pub unsafe fn CloseAlert(&self, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).CloseAlert)(self, aName)
    }


}


/// `interface nsIAlertsDoNotDisturb : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAlertsDoNotDisturb {
    vtable: *const nsIAlertsDoNotDisturbVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAlertsDoNotDisturb.
unsafe impl XpCom for nsIAlertsDoNotDisturb {
    const IID: nsIID = nsID(0xc5d63e3a, 0x259d, 0x45a8,
        [0xb9, 0x64, 0x83, 0x77, 0x96, 0x7c, 0xb4, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAlertsDoNotDisturb {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAlertsDoNotDisturb.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAlertsDoNotDisturbCoerce {
    /// Cheaply cast a value of this type from a `nsIAlertsDoNotDisturb`.
    fn coerce_from(v: &nsIAlertsDoNotDisturb) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAlertsDoNotDisturbCoerce for nsIAlertsDoNotDisturb {
    #[inline]
    fn coerce_from(v: &nsIAlertsDoNotDisturb) -> &Self {
        v
    }
}

impl nsIAlertsDoNotDisturb {
    /// Cast this `nsIAlertsDoNotDisturb` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAlertsDoNotDisturbCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAlertsDoNotDisturb {
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
impl<T: nsISupportsCoerce> nsIAlertsDoNotDisturbCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsDoNotDisturb) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAlertsDoNotDisturb
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAlertsDoNotDisturbVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute bool manualDoNotDisturb; */
    pub GetManualDoNotDisturb: unsafe extern "system" fn (this: *const nsIAlertsDoNotDisturb, aManualDoNotDisturb: *mut bool) -> ::nserror::nsresult,

    /* attribute bool manualDoNotDisturb; */
    pub SetManualDoNotDisturb: unsafe extern "system" fn (this: *const nsIAlertsDoNotDisturb, aManualDoNotDisturb: bool) -> ::nserror::nsresult,

    /* attribute bool suppressForScreenSharing; */
    pub GetSuppressForScreenSharing: unsafe extern "system" fn (this: *const nsIAlertsDoNotDisturb, aSuppressForScreenSharing: *mut bool) -> ::nserror::nsresult,

    /* attribute bool suppressForScreenSharing; */
    pub SetSuppressForScreenSharing: unsafe extern "system" fn (this: *const nsIAlertsDoNotDisturb, aSuppressForScreenSharing: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAlertsDoNotDisturb {

    /// ```text
    /// /**
    ///    * Toggles a manual Do Not Disturb mode for the service to reduce the amount
    ///    * of disruption that alerts cause the user.
    ///    * This may mean only displaying them in a notification tray/center or not
    ///    * displaying them at all. If a system backend already supports a similar
    ///    * feature controlled by the user, enabling this may not have any impact on
    ///    * code to show an alert. e.g. on OS X, the system will take care not
    ///    * disrupting a user if we simply create a notification like usual.
    ///    */
    /// ```
    ///

    /// `attribute bool manualDoNotDisturb;`
    #[inline]
    pub unsafe fn GetManualDoNotDisturb(&self, aManualDoNotDisturb: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetManualDoNotDisturb)(self, aManualDoNotDisturb)
    }


    /// ```text
    /// /**
    ///    * Toggles a manual Do Not Disturb mode for the service to reduce the amount
    ///    * of disruption that alerts cause the user.
    ///    * This may mean only displaying them in a notification tray/center or not
    ///    * displaying them at all. If a system backend already supports a similar
    ///    * feature controlled by the user, enabling this may not have any impact on
    ///    * code to show an alert. e.g. on OS X, the system will take care not
    ///    * disrupting a user if we simply create a notification like usual.
    ///    */
    /// ```
    ///

    /// `attribute bool manualDoNotDisturb;`
    #[inline]
    pub unsafe fn SetManualDoNotDisturb(&self, aManualDoNotDisturb: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetManualDoNotDisturb)(self, aManualDoNotDisturb)
    }


    /// ```text
    /// /**
    ///    * Toggles a mode for the service to suppress all notifications from
    ///    * being dispatched when sharing the screen via the getMediaDisplay
    ///    * API.
    ///    */
    /// ```
    ///

    /// `attribute bool suppressForScreenSharing;`
    #[inline]
    pub unsafe fn GetSuppressForScreenSharing(&self, aSuppressForScreenSharing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSuppressForScreenSharing)(self, aSuppressForScreenSharing)
    }


    /// ```text
    /// /**
    ///    * Toggles a mode for the service to suppress all notifications from
    ///    * being dispatched when sharing the screen via the getMediaDisplay
    ///    * API.
    ///    */
    /// ```
    ///

    /// `attribute bool suppressForScreenSharing;`
    #[inline]
    pub unsafe fn SetSuppressForScreenSharing(&self, aSuppressForScreenSharing: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSuppressForScreenSharing)(self, aSuppressForScreenSharing)
    }


}


/// `interface nsIAlertsIconData : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAlertsIconData {
    vtable: *const nsIAlertsIconDataVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAlertsIconData.
unsafe impl XpCom for nsIAlertsIconData {
    const IID: nsIID = nsID(0xfc6d7f0a, 0x0cf6, 0x4268,
        [0x8c, 0x71, 0xab, 0x64, 0x08, 0x42, 0xb9, 0xb1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAlertsIconData {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAlertsIconData.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAlertsIconDataCoerce {
    /// Cheaply cast a value of this type from a `nsIAlertsIconData`.
    fn coerce_from(v: &nsIAlertsIconData) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAlertsIconDataCoerce for nsIAlertsIconData {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconData) -> &Self {
        v
    }
}

impl nsIAlertsIconData {
    /// Cast this `nsIAlertsIconData` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAlertsIconDataCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAlertsIconData {
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
impl<T: nsISupportsCoerce> nsIAlertsIconDataCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconData) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAlertsIconData
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAlertsIconDataVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void showAlertWithIconData (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in uint32_t aIconSize, [array, size_is (aIconSize), const] in uint8_t aIconData); */
    pub ShowAlertWithIconData: unsafe extern "system" fn (this: *const nsIAlertsIconData, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver, aIconSize: uint32_t, aIconData: *const uint8_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAlertsIconData {

    /// ```text
    /// /**
    ///    * Shows an alert with an icon. Web notifications use the favicon of the
    ///    * page that created the alert. If the favicon is not in the Places database,
    ///    * |aIconSize| will be zero.
    ///   */
    /// ```
    ///

    /// `void showAlertWithIconData (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in uint32_t aIconSize, [array, size_is (aIconSize), const] in uint8_t aIconData);`
    #[inline]
    pub unsafe fn ShowAlertWithIconData(&self, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver, aIconSize: uint32_t, aIconData: *const uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).ShowAlertWithIconData)(self, aAlert, aAlertListener, aIconSize, aIconData)
    }


}


/// `interface nsIAlertsIconURI : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAlertsIconURI {
    vtable: *const nsIAlertsIconURIVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAlertsIconURI.
unsafe impl XpCom for nsIAlertsIconURI {
    const IID: nsIID = nsID(0xf3c82915, 0xbf60, 0x41ea,
        [0x91, 0xce, 0x6c, 0x46, 0xb2, 0x2e, 0x38, 0x1a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAlertsIconURI {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAlertsIconURI.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAlertsIconURICoerce {
    /// Cheaply cast a value of this type from a `nsIAlertsIconURI`.
    fn coerce_from(v: &nsIAlertsIconURI) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAlertsIconURICoerce for nsIAlertsIconURI {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconURI) -> &Self {
        v
    }
}

impl nsIAlertsIconURI {
    /// Cast this `nsIAlertsIconURI` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAlertsIconURICoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAlertsIconURI {
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
impl<T: nsISupportsCoerce> nsIAlertsIconURICoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAlertsIconURI) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAlertsIconURI
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAlertsIconURIVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI); */
    pub ShowAlertWithIconURI: unsafe extern "system" fn (this: *const nsIAlertsIconURI, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver, aIconURI: *const nsIURI) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAlertsIconURI {

    /// ```text
    /// /**
    ///    * Shows an alert with an icon URI. Web notifications use |moz-anno:|
    ///    * URIs to reference favicons from Places. If the page doesn't have a
    ///    * favicon, |aIconURI| will be |null|.
    ///    */
    /// ```
    ///

    /// `void showAlertWithIconURI (in nsIAlertNotification aAlert, [optional] in nsIObserver aAlertListener, [optional] in nsIURI aIconURI);`
    #[inline]
    pub unsafe fn ShowAlertWithIconURI(&self, aAlert: *const nsIAlertNotification, aAlertListener: *const nsIObserver, aIconURI: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).ShowAlertWithIconURI)(self, aAlert, aAlertListener, aIconURI)
    }


}


