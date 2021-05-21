//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/synth/nsISpeechService.idl
//


/// `interface nsISpeechTaskCallback : nsISupports`
///

/// ```text
/// /**
///  * A callback is implemented by the service.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeechTaskCallback {
    vtable: *const nsISpeechTaskCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeechTaskCallback.
unsafe impl XpCom for nsISpeechTaskCallback {
    const IID: nsIID = nsID(0xc576de0c, 0x8a3d, 0x4570,
        [0xbe, 0x7e, 0x98, 0x76, 0xd3, 0xe5, 0xbe, 0xd2]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeechTaskCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeechTaskCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeechTaskCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsISpeechTaskCallback`.
    fn coerce_from(v: &nsISpeechTaskCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeechTaskCallbackCoerce for nsISpeechTaskCallback {
    #[inline]
    fn coerce_from(v: &nsISpeechTaskCallback) -> &Self {
        v
    }
}

impl nsISpeechTaskCallback {
    /// Cast this `nsISpeechTaskCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeechTaskCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeechTaskCallback {
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
impl<T: nsISupportsCoerce> nsISpeechTaskCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechTaskCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeechTaskCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeechTaskCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onPause (); */
    pub OnPause: unsafe extern "system" fn (this: *const nsISpeechTaskCallback) -> ::nserror::nsresult,

    /* void onResume (); */
    pub OnResume: unsafe extern "system" fn (this: *const nsISpeechTaskCallback) -> ::nserror::nsresult,

    /* void onCancel (); */
    pub OnCancel: unsafe extern "system" fn (this: *const nsISpeechTaskCallback) -> ::nserror::nsresult,

    /* void onVolumeChanged (in float aVolume); */
    pub OnVolumeChanged: unsafe extern "system" fn (this: *const nsISpeechTaskCallback, aVolume: libc::c_float) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeechTaskCallback {

    /// ```text
    /// /**
    ///    * The user or application has paused the speech.
    ///    */
    /// ```
    ///

    /// `void onPause ();`
    #[inline]
    pub unsafe fn OnPause(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnPause)(self, )
    }


    /// ```text
    /// /**
    ///    * The user or application has resumed the speech.
    ///    */
    /// ```
    ///

    /// `void onResume ();`
    #[inline]
    pub unsafe fn OnResume(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnResume)(self, )
    }


    /// ```text
    /// /**
    ///    * The user or application has canceled the speech.
    ///    */
    /// ```
    ///

    /// `void onCancel ();`
    #[inline]
    pub unsafe fn OnCancel(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnCancel)(self, )
    }


    /// ```text
    /// /**
    ///    * The user or application has changed the volume of this speech.
    ///    */
    /// ```
    ///

    /// `void onVolumeChanged (in float aVolume);`
    #[inline]
    pub unsafe fn OnVolumeChanged(&self, aVolume: libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).OnVolumeChanged)(self, aVolume)
    }


}


/// `interface nsISpeechTask : nsISupports`
///

/// ```text
/// /**
///  * A task is associated with a single utterance. It is provided by the browser
///  * to the service in the speak() method.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeechTask {
    vtable: *const nsISpeechTaskVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeechTask.
unsafe impl XpCom for nsISpeechTask {
    const IID: nsIID = nsID(0xad59949c, 0x2437, 0x4b35,
        [0x8e, 0xeb, 0xd7, 0x60, 0xca, 0xab, 0x75, 0xc5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeechTask {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeechTask.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeechTaskCoerce {
    /// Cheaply cast a value of this type from a `nsISpeechTask`.
    fn coerce_from(v: &nsISpeechTask) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeechTaskCoerce for nsISpeechTask {
    #[inline]
    fn coerce_from(v: &nsISpeechTask) -> &Self {
        v
    }
}

impl nsISpeechTask {
    /// Cast this `nsISpeechTask` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeechTaskCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeechTask {
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
impl<T: nsISupportsCoerce> nsISpeechTaskCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechTask) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeechTask
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeechTaskVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setup (in nsISpeechTaskCallback aCallback); */
    pub Setup: unsafe extern "system" fn (this: *const nsISpeechTask, aCallback: *const nsISpeechTaskCallback) -> ::nserror::nsresult,

    /* void dispatchStart (); */
    pub DispatchStart: unsafe extern "system" fn (this: *const nsISpeechTask) -> ::nserror::nsresult,

    /* void dispatchEnd (in float aElapsedTime, in unsigned long aCharIndex); */
    pub DispatchEnd: unsafe extern "system" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult,

    /* void dispatchPause (in float aElapsedTime, in unsigned long aCharIndex); */
    pub DispatchPause: unsafe extern "system" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult,

    /* void dispatchResume (in float aElapsedTime, in unsigned long aCharIndex); */
    pub DispatchResume: unsafe extern "system" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult,

    /* void dispatchError (in float aElapsedTime, in unsigned long aCharIndex); */
    pub DispatchError: unsafe extern "system" fn (this: *const nsISpeechTask, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult,

    /* [optional_argc] void dispatchBoundary (in AString aName, in float aElapsedTime, in unsigned long aCharIndex, [optional] in unsigned long aCharLength); */
    /// Unable to generate binding because `optional_argc is unsupported`
    pub DispatchBoundary: *const ::libc::c_void,

    /* void dispatchMark (in AString aName, in float aElapsedTime, in unsigned long aCharIndex); */
    pub DispatchMark: unsafe extern "system" fn (this: *const nsISpeechTask, aName: *const ::nsstring::nsAString, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeechTask {

    /// ```text
    /// /**
    ///    * Prepare browser for speech.
    ///    *
    ///    * @param aCallback callback object for mid-speech operations.
    ///    */
    /// ```
    ///

    /// `void setup (in nsISpeechTaskCallback aCallback);`
    #[inline]
    pub unsafe fn Setup(&self, aCallback: *const nsISpeechTaskCallback) -> ::nserror::nsresult {
        ((*self.vtable).Setup)(self, aCallback)
    }


    /// ```text
    /// /**
    ///    * Dispatch start event.
    ///    */
    /// ```
    ///

    /// `void dispatchStart ();`
    #[inline]
    pub unsafe fn DispatchStart(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DispatchStart)(self, )
    }


    /// ```text
    /// /**
    ///    * Dispatch end event.
    ///    *
    ///    * @param aElapsedTime time in seconds since speech has started.
    ///    * @param aCharIndex   offset of spoken characters.
    ///    */
    /// ```
    ///

    /// `void dispatchEnd (in float aElapsedTime, in unsigned long aCharIndex);`
    #[inline]
    pub unsafe fn DispatchEnd(&self, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).DispatchEnd)(self, aElapsedTime, aCharIndex)
    }


    /// ```text
    /// /**
    ///    * Dispatch pause event.
    ///    *
    ///    * @param aElapsedTime time in seconds since speech has started.
    ///    * @param aCharIndex   offset of spoken characters.
    ///    */
    /// ```
    ///

    /// `void dispatchPause (in float aElapsedTime, in unsigned long aCharIndex);`
    #[inline]
    pub unsafe fn DispatchPause(&self, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).DispatchPause)(self, aElapsedTime, aCharIndex)
    }


    /// ```text
    /// /**
    ///    * Dispatch resume event.
    ///    *
    ///    * @param aElapsedTime time in seconds since speech has started.
    ///    * @param aCharIndex   offset of spoken characters.
    ///    */
    /// ```
    ///

    /// `void dispatchResume (in float aElapsedTime, in unsigned long aCharIndex);`
    #[inline]
    pub unsafe fn DispatchResume(&self, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).DispatchResume)(self, aElapsedTime, aCharIndex)
    }


    /// ```text
    /// /**
    ///    * Dispatch error event.
    ///    *
    ///    * @param aElapsedTime time in seconds since speech has started.
    ///    * @param aCharIndex   offset of spoken characters.
    ///    */
    /// ```
    ///

    /// `void dispatchError (in float aElapsedTime, in unsigned long aCharIndex);`
    #[inline]
    pub unsafe fn DispatchError(&self, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).DispatchError)(self, aElapsedTime, aCharIndex)
    }


    /// ```text
    /// /**
    ///    * Dispatch boundary event.
    ///    *
    ///    * @param aName        name of boundary, 'word' or 'sentence'
    ///    * @param aElapsedTime time in seconds since speech has started.
    ///    * @param aCharIndex   offset of spoken characters.
    ///    * @param aCharLength  length of text in boundary event to be spoken.
    ///    */
    /// ```
    ///

    /// `[optional_argc] void dispatchBoundary (in AString aName, in float aElapsedTime, in unsigned long aCharIndex, [optional] in unsigned long aCharLength);`
    const _DispatchBoundary: () = ();

    /// ```text
    /// /**
    ///    * Dispatch mark event.
    ///    *
    ///    * @param aName        mark identifier.
    ///    * @param aElapsedTime time in seconds since speech has started.
    ///    * @param aCharIndex   offset of spoken characters.
    ///    */
    /// ```
    ///

    /// `void dispatchMark (in AString aName, in float aElapsedTime, in unsigned long aCharIndex);`
    #[inline]
    pub unsafe fn DispatchMark(&self, aName: *const ::nsstring::nsAString, aElapsedTime: libc::c_float, aCharIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).DispatchMark)(self, aName, aElapsedTime, aCharIndex)
    }


}


/// `interface nsISpeechService : nsISupports`
///

/// ```text
/// /**
///  * The main interface of a speech synthesis service.
///  *
///  * A service is responsible for outputting audio.
///  * The service dispatches events, starting with dispatchStart() and ending with
///  * dispatchEnd or dispatchError().
///  * A service must also respond with the currect actions and events in response
///  * to implemented callback methods.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISpeechService {
    vtable: *const nsISpeechServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISpeechService.
unsafe impl XpCom for nsISpeechService {
    const IID: nsIID = nsID(0x9b7d59db, 0x88ff, 0x43d0,
        [0xb6, 0xee, 0x9f, 0x63, 0xd0, 0x42, 0xd0, 0x8f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISpeechService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISpeechService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISpeechServiceCoerce {
    /// Cheaply cast a value of this type from a `nsISpeechService`.
    fn coerce_from(v: &nsISpeechService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISpeechServiceCoerce for nsISpeechService {
    #[inline]
    fn coerce_from(v: &nsISpeechService) -> &Self {
        v
    }
}

impl nsISpeechService {
    /// Cast this `nsISpeechService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISpeechServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISpeechService {
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
impl<T: nsISupportsCoerce> nsISpeechServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISpeechService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISpeechService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISpeechServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void speak (in AString aText, in AString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask); */
    pub Speak: unsafe extern "system" fn (this: *const nsISpeechService, aText: *const ::nsstring::nsAString, aUri: *const ::nsstring::nsAString, aVolume: libc::c_float, aRate: libc::c_float, aPitch: libc::c_float, aTask: *const nsISpeechTask) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISpeechService {

    /// ```text
    /// /**
    ///    * Speak the given text using the voice identified byu the given uri. See
    ///    * W3C Speech API spec for information about pitch and rate.
    ///    * https://dvcs.w3.org/hg/speech-api/raw-file/tip/speechapi.html#utterance-attributes
    ///    *
    ///    * @param aText   text to utter.
    ///    * @param aUri    unique voice identifier.
    ///    * @param aVolume volume to speak voice in. Only relevant for indirect audio.
    ///    * @param aRate   rate to speak voice in.
    ///    * @param aPitch  pitch to speak voice in.
    ///    * @param aTask  task instance for utterance, used for sending events or audio
    ///    *                 data back to browser.
    ///    */
    /// ```
    ///

    /// `void speak (in AString aText, in AString aUri, in float aVolume, in float aRate, in float aPitch, in nsISpeechTask aTask);`
    #[inline]
    pub unsafe fn Speak(&self, aText: *const ::nsstring::nsAString, aUri: *const ::nsstring::nsAString, aVolume: libc::c_float, aRate: libc::c_float, aPitch: libc::c_float, aTask: *const nsISpeechTask) -> ::nserror::nsresult {
        ((*self.vtable).Speak)(self, aText, aUri, aVolume, aRate, aPitch, aTask)
    }


}


