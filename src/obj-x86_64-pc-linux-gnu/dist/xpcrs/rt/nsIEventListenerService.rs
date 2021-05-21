//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/events/nsIEventListenerService.idl
//


/// `interface nsIEventListenerChange : nsISupports`
///

/// ```text
/// /**
///  * Contains an event target along with a count of event listener changes
///  * affecting accessibility.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEventListenerChange {
    vtable: *const nsIEventListenerChangeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEventListenerChange.
unsafe impl XpCom for nsIEventListenerChange {
    const IID: nsIID = nsID(0x07222b02, 0xda12, 0x4cf4,
        [0xb2, 0xf7, 0x76, 0x1d, 0xa0, 0x07, 0xa8, 0xd8]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEventListenerChange {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEventListenerChange.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEventListenerChangeCoerce {
    /// Cheaply cast a value of this type from a `nsIEventListenerChange`.
    fn coerce_from(v: &nsIEventListenerChange) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEventListenerChangeCoerce for nsIEventListenerChange {
    #[inline]
    fn coerce_from(v: &nsIEventListenerChange) -> &Self {
        v
    }
}

impl nsIEventListenerChange {
    /// Cast this `nsIEventListenerChange` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEventListenerChangeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEventListenerChange {
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
impl<T: nsISupportsCoerce> nsIEventListenerChangeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventListenerChange) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEventListenerChange
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEventListenerChangeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute EventTarget target; */
    pub GetTarget: unsafe extern "system" fn (this: *const nsIEventListenerChange, aTarget: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [noscript] readonly attribute uint32_t countOfEventListenerChangesAffectingAccessibility; */
    pub GetCountOfEventListenerChangesAffectingAccessibility: unsafe extern "system" fn (this: *const nsIEventListenerChange, aCountOfEventListenerChangesAffectingAccessibility: *mut uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEventListenerChange {


    /// `readonly attribute EventTarget target;`
    #[inline]
    pub unsafe fn GetTarget(&self, aTarget: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetTarget)(self, aTarget)
    }



    /// `[noscript] readonly attribute uint32_t countOfEventListenerChangesAffectingAccessibility;`
    #[inline]
    pub unsafe fn GetCountOfEventListenerChangesAffectingAccessibility(&self, aCountOfEventListenerChangesAffectingAccessibility: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetCountOfEventListenerChangesAffectingAccessibility)(self, aCountOfEventListenerChangesAffectingAccessibility)
    }


}


/// `interface nsIListenerChangeListener : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIListenerChangeListener {
    vtable: *const nsIListenerChangeListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIListenerChangeListener.
unsafe impl XpCom for nsIListenerChangeListener {
    const IID: nsIID = nsID(0xaa7c95f6, 0xd3b5, 0x44b3,
        [0x95, 0x97, 0x1d, 0x9f, 0x19, 0xb9, 0xc5, 0xf2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIListenerChangeListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIListenerChangeListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIListenerChangeListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIListenerChangeListener`.
    fn coerce_from(v: &nsIListenerChangeListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIListenerChangeListenerCoerce for nsIListenerChangeListener {
    #[inline]
    fn coerce_from(v: &nsIListenerChangeListener) -> &Self {
        v
    }
}

impl nsIListenerChangeListener {
    /// Cast this `nsIListenerChangeListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIListenerChangeListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIListenerChangeListener {
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
impl<T: nsISupportsCoerce> nsIListenerChangeListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIListenerChangeListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIListenerChangeListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIListenerChangeListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void listenersChanged (in nsIArray aEventListenerChanges); */
    pub ListenersChanged: unsafe extern "system" fn (this: *const nsIListenerChangeListener, aEventListenerChanges: *const nsIArray) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIListenerChangeListener {


    /// `void listenersChanged (in nsIArray aEventListenerChanges);`
    #[inline]
    pub unsafe fn ListenersChanged(&self, aEventListenerChanges: *const nsIArray) -> ::nserror::nsresult {
        ((*self.vtable).ListenersChanged)(self, aEventListenerChanges)
    }


}


/// `interface nsIEventListenerInfo : nsISupports`
///

/// ```text
/// /**
///  * An instance of this interface describes how an event listener
///  * was added to an event target.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEventListenerInfo {
    vtable: *const nsIEventListenerInfoVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEventListenerInfo.
unsafe impl XpCom for nsIEventListenerInfo {
    const IID: nsIID = nsID(0x11ba5fd7, 0x8db2, 0x4b1a,
        [0x9f, 0x67, 0x34, 0x2c, 0xfa, 0x11, 0xaf, 0xad]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEventListenerInfo {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEventListenerInfo.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEventListenerInfoCoerce {
    /// Cheaply cast a value of this type from a `nsIEventListenerInfo`.
    fn coerce_from(v: &nsIEventListenerInfo) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEventListenerInfoCoerce for nsIEventListenerInfo {
    #[inline]
    fn coerce_from(v: &nsIEventListenerInfo) -> &Self {
        v
    }
}

impl nsIEventListenerInfo {
    /// Cast this `nsIEventListenerInfo` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEventListenerInfoCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEventListenerInfo {
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
impl<T: nsISupportsCoerce> nsIEventListenerInfoCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventListenerInfo) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEventListenerInfo
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEventListenerInfoVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIEventListenerInfo, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean capturing; */
    pub GetCapturing: unsafe extern "system" fn (this: *const nsIEventListenerInfo, aCapturing: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean allowsUntrusted; */
    pub GetAllowsUntrusted: unsafe extern "system" fn (this: *const nsIEventListenerInfo, aAllowsUntrusted: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean inSystemEventGroup; */
    pub GetInSystemEventGroup: unsafe extern "system" fn (this: *const nsIEventListenerInfo, aInSystemEventGroup: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] readonly attribute jsval listenerObject; */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub GetListenerObject: *const ::libc::c_void,

    /* AString toSource (); */
    pub ToSource: unsafe extern "system" fn (this: *const nsIEventListenerInfo, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEventListenerInfo {

    /// ```text
    /// /**
    ///    * The type of the event for which the listener was added.
    ///    * Null if the listener is for all the events.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }



    /// `readonly attribute boolean capturing;`
    #[inline]
    pub unsafe fn GetCapturing(&self, aCapturing: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCapturing)(self, aCapturing)
    }



    /// `readonly attribute boolean allowsUntrusted;`
    #[inline]
    pub unsafe fn GetAllowsUntrusted(&self, aAllowsUntrusted: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAllowsUntrusted)(self, aAllowsUntrusted)
    }



    /// `readonly attribute boolean inSystemEventGroup;`
    #[inline]
    pub unsafe fn GetInSystemEventGroup(&self, aInSystemEventGroup: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInSystemEventGroup)(self, aInSystemEventGroup)
    }


    /// ```text
    /// /**
    ///    * The underlying JS object of the event listener, if this listener
    ///    * has one.  Null otherwise.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] readonly attribute jsval listenerObject;`
    const _GetListenerObject: () = ();

    /// ```text
    /// /**
    ///    * Tries to serialize event listener to a string.
    ///    * Returns null if serialization isn't possible
    ///    * (for example with C++ listeners).
    ///    */
    /// ```
    ///

    /// `AString toSource ();`
    #[inline]
    pub unsafe fn ToSource(&self, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).ToSource)(self, _retval)
    }


}


/// `interface nsIEventListenerService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEventListenerService {
    vtable: *const nsIEventListenerServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEventListenerService.
unsafe impl XpCom for nsIEventListenerService {
    const IID: nsIID = nsID(0x77aab5f7, 0x213d, 0x4db4,
        [0x9f, 0x22, 0xe4, 0x6d, 0xfb, 0x77, 0x4f, 0x15]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEventListenerService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEventListenerService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEventListenerServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIEventListenerService`.
    fn coerce_from(v: &nsIEventListenerService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEventListenerServiceCoerce for nsIEventListenerService {
    #[inline]
    fn coerce_from(v: &nsIEventListenerService) -> &Self {
        v
    }
}

impl nsIEventListenerService {
    /// Cast this `nsIEventListenerService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEventListenerServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEventListenerService {
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
impl<T: nsISupportsCoerce> nsIEventListenerServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEventListenerService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEventListenerService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEventListenerServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* Array<nsIEventListenerInfo> getListenerInfoFor (in EventTarget aEventTarget); */
    pub GetListenerInfoFor: unsafe extern "system" fn (this: *const nsIEventListenerService, aEventTarget: *const libc::c_void, _retval: *mut thin_vec::ThinVec<RefPtr<nsIEventListenerInfo>>) -> ::nserror::nsresult,

    /* Array<EventTarget> getEventTargetChainFor (in EventTarget aEventTarget, in boolean composed); */
    pub GetEventTargetChainFor: unsafe extern "system" fn (this: *const nsIEventListenerService, aEventTarget: *const libc::c_void, composed: bool, _retval: *mut thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult,

    /* boolean hasListenersFor (in EventTarget aEventTarget, in AString aType); */
    pub HasListenersFor: unsafe extern "system" fn (this: *const nsIEventListenerService, aEventTarget: *const libc::c_void, aType: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* [implicit_jscontext] void addSystemEventListener (in EventTarget target, in AString type, in jsval listener, in boolean useCapture); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddSystemEventListener: *const ::libc::c_void,

    /* [implicit_jscontext] void removeSystemEventListener (in EventTarget target, in AString type, in jsval listener, in boolean useCapture); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RemoveSystemEventListener: *const ::libc::c_void,

    /* [implicit_jscontext] void addListenerForAllEvents (in EventTarget target, in jsval listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted, [optional] in boolean aSystemEventGroup); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub AddListenerForAllEvents: *const ::libc::c_void,

    /* [implicit_jscontext] void removeListenerForAllEvents (in EventTarget target, in jsval listener, [optional] in boolean aUseCapture, [optional] in boolean aSystemEventGroup); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub RemoveListenerForAllEvents: *const ::libc::c_void,

    /* void addListenerChangeListener (in nsIListenerChangeListener aListener); */
    pub AddListenerChangeListener: unsafe extern "system" fn (this: *const nsIEventListenerService, aListener: *const nsIListenerChangeListener) -> ::nserror::nsresult,

    /* void removeListenerChangeListener (in nsIListenerChangeListener aListener); */
    pub RemoveListenerChangeListener: unsafe extern "system" fn (this: *const nsIEventListenerService, aListener: *const nsIListenerChangeListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEventListenerService {

    /// ```text
    /// /**
    ///    * Returns an array of nsIEventListenerInfo objects.
    ///    * If aEventTarget doesn't have any listeners, this returns null.
    ///    */
    /// ```
    ///

    /// `Array<nsIEventListenerInfo> getListenerInfoFor (in EventTarget aEventTarget);`
    #[inline]
    pub unsafe fn GetListenerInfoFor(&self, aEventTarget: *const libc::c_void, _retval: *mut thin_vec::ThinVec<RefPtr<nsIEventListenerInfo>>) -> ::nserror::nsresult {
        ((*self.vtable).GetListenerInfoFor)(self, aEventTarget, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns an array of event targets.
    ///    * aEventTarget will be at index 0.
    ///    * The objects are the ones that would be used as DOMEvent.currentTarget while
    ///    * dispatching an event to aEventTarget
    ///    * @note Some events, especially 'load', may actually have a shorter
    ///    *       event target chain than what this methods returns.
    ///    */
    /// ```
    ///

    /// `Array<EventTarget> getEventTargetChainFor (in EventTarget aEventTarget, in boolean composed);`
    #[inline]
    pub unsafe fn GetEventTargetChainFor(&self, aEventTarget: *const libc::c_void, composed: bool, _retval: *mut thin_vec::ThinVec<*const libc::c_void>) -> ::nserror::nsresult {
        ((*self.vtable).GetEventTargetChainFor)(self, aEventTarget, composed, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns true if a event target has any listener for the given type.
    ///    */
    /// ```
    ///

    /// `boolean hasListenersFor (in EventTarget aEventTarget, in AString aType);`
    #[inline]
    pub unsafe fn HasListenersFor(&self, aEventTarget: *const libc::c_void, aType: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).HasListenersFor)(self, aEventTarget, aType, _retval)
    }


    /// ```text
    /// /**
    ///    * Add a system-group eventlistener to a event target.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void addSystemEventListener (in EventTarget target, in AString type, in jsval listener, in boolean useCapture);`
    const _AddSystemEventListener: () = ();

    /// ```text
    /// /**
    ///    * Remove a system-group eventlistener from a event target.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void removeSystemEventListener (in EventTarget target, in AString type, in jsval listener, in boolean useCapture);`
    const _RemoveSystemEventListener: () = ();


    /// `[implicit_jscontext] void addListenerForAllEvents (in EventTarget target, in jsval listener, [optional] in boolean aUseCapture, [optional] in boolean aWantsUntrusted, [optional] in boolean aSystemEventGroup);`
    const _AddListenerForAllEvents: () = ();


    /// `[implicit_jscontext] void removeListenerForAllEvents (in EventTarget target, in jsval listener, [optional] in boolean aUseCapture, [optional] in boolean aSystemEventGroup);`
    const _RemoveListenerForAllEvents: () = ();


    /// `void addListenerChangeListener (in nsIListenerChangeListener aListener);`
    #[inline]
    pub unsafe fn AddListenerChangeListener(&self, aListener: *const nsIListenerChangeListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListenerChangeListener)(self, aListener)
    }



    /// `void removeListenerChangeListener (in nsIListenerChangeListener aListener);`
    #[inline]
    pub unsafe fn RemoveListenerChangeListener(&self, aListener: *const nsIListenerChangeListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListenerChangeListener)(self, aListener)
    }


}


