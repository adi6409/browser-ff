//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/audiochannel/nsIAudioChannelAgent.idl
//


/// `typedef uint32_t  nsSuspendedTypes;`
///


pub type nsSuspendedTypes = uint32_t;


/// `interface nsISuspendedTypes : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISuspendedTypes {
    vtable: *const nsISuspendedTypesVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISuspendedTypes.
unsafe impl XpCom for nsISuspendedTypes {
    const IID: nsIID = nsID(0x2822a840, 0xf009, 0x11e5,
        [0xa8, 0x37, 0x08, 0x00, 0x20, 0x0c, 0x9a, 0x66]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISuspendedTypes {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISuspendedTypes.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISuspendedTypesCoerce {
    /// Cheaply cast a value of this type from a `nsISuspendedTypes`.
    fn coerce_from(v: &nsISuspendedTypes) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISuspendedTypesCoerce for nsISuspendedTypes {
    #[inline]
    fn coerce_from(v: &nsISuspendedTypes) -> &Self {
        v
    }
}

impl nsISuspendedTypes {
    /// Cast this `nsISuspendedTypes` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISuspendedTypesCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISuspendedTypes {
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
impl<T: nsISupportsCoerce> nsISuspendedTypesCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISuspendedTypes) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISuspendedTypes
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISuspendedTypesVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISuspendedTypes {
    /// ```text
    /// /**
    ///    * The suspended enum is used for delaying autoplay video in non-visited tab
    ///    *
    ///    * Note: the "remote side" must control the AudioChannelAgent using
    ///    * nsIAudioChannelAgentCallback.windowSuspendChanged() callback instead using
    ///    * play/pause methods or any button in the webpage.
    ///    *
    ///    * - SUSPENDED_BLOCK
    ///    * It's used to prevent auto-playing media in inactive page in order to
    ///    * reduce the power consumption, and the media can't be resumed until the
    ///    * page becomes active again. It would change the internal state of
    ///    * MediaElement when it's being blocked/resumed, so it won't trigger the
    ///    * related JS event. eg. "play" and "pause" event.
    ///    */
    /// ```
    ///

    pub const NONE_SUSPENDED: i64 = 0;


    pub const SUSPENDED_BLOCK: i64 = 1;


}


/// `interface nsIAudioChannelAgentCallback : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAudioChannelAgentCallback {
    vtable: *const nsIAudioChannelAgentCallbackVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAudioChannelAgentCallback.
unsafe impl XpCom for nsIAudioChannelAgentCallback {
    const IID: nsIID = nsID(0x15c05894, 0x408e, 0x4798,
        [0xb5, 0x27, 0xa8, 0xc3, 0x2d, 0x9c, 0x5f, 0x8c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAudioChannelAgentCallback {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAudioChannelAgentCallback.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAudioChannelAgentCallbackCoerce {
    /// Cheaply cast a value of this type from a `nsIAudioChannelAgentCallback`.
    fn coerce_from(v: &nsIAudioChannelAgentCallback) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAudioChannelAgentCallbackCoerce for nsIAudioChannelAgentCallback {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgentCallback) -> &Self {
        v
    }
}

impl nsIAudioChannelAgentCallback {
    /// Cast this `nsIAudioChannelAgentCallback` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAudioChannelAgentCallbackCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAudioChannelAgentCallback {
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
impl<T: nsISupportsCoerce> nsIAudioChannelAgentCallbackCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgentCallback) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAudioChannelAgentCallback
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAudioChannelAgentCallbackVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void windowVolumeChanged (in float aVolume, in bool aMuted); */
    pub WindowVolumeChanged: unsafe extern "system" fn (this: *const nsIAudioChannelAgentCallback, aVolume: libc::c_float, aMuted: bool) -> ::nserror::nsresult,

    /* void windowSuspendChanged (in uint32_t aSuspend); */
    pub WindowSuspendChanged: unsafe extern "system" fn (this: *const nsIAudioChannelAgentCallback, aSuspend: uint32_t) -> ::nserror::nsresult,

    /* void windowAudioCaptureChanged (in bool aCapture); */
    pub WindowAudioCaptureChanged: unsafe extern "system" fn (this: *const nsIAudioChannelAgentCallback, aCapture: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAudioChannelAgentCallback {

    /// ```text
    /// /**
    ///    * Notified when the window volume/mute is changed
    ///    */
    /// ```
    ///

    /// `void windowVolumeChanged (in float aVolume, in bool aMuted);`
    #[inline]
    pub unsafe fn WindowVolumeChanged(&self, aVolume: libc::c_float, aMuted: bool) -> ::nserror::nsresult {
        ((*self.vtable).WindowVolumeChanged)(self, aVolume, aMuted)
    }


    /// ```text
    /// /**
    ///    * Notified when the window needs to be suspended or resumed.
    ///    */
    /// ```
    ///

    /// `void windowSuspendChanged (in uint32_t aSuspend);`
    #[inline]
    pub unsafe fn WindowSuspendChanged(&self, aSuspend: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).WindowSuspendChanged)(self, aSuspend)
    }


    /// ```text
    /// /**
    ///    * Notified when the capture state is changed.
    ///    */
    /// ```
    ///

    /// `void windowAudioCaptureChanged (in bool aCapture);`
    #[inline]
    pub unsafe fn WindowAudioCaptureChanged(&self, aCapture: bool) -> ::nserror::nsresult {
        ((*self.vtable).WindowAudioCaptureChanged)(self, aCapture)
    }


}


/// `interface nsIAudioChannelAgent : nsISupports`
///

/// ```text
/// /**
///  * This interface provides an agent for gecko components to participate
///  * in the audio channel service. Gecko components are responsible for
///  *   1. Notifying the agent when they start/stop using this channel.
///  *   2. Notifying the agent when they are audible.
///  *
///  * The agent will invoke a callback to notify Gecko components of
///  *   1. Changes to the playable status of this channel.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAudioChannelAgent {
    vtable: *const nsIAudioChannelAgentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAudioChannelAgent.
unsafe impl XpCom for nsIAudioChannelAgent {
    const IID: nsIID = nsID(0x4d212770, 0x5d7b, 0x446f,
        [0x93, 0x94, 0x63, 0x2e, 0x35, 0x1d, 0x96, 0xee]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAudioChannelAgent {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAudioChannelAgent.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAudioChannelAgentCoerce {
    /// Cheaply cast a value of this type from a `nsIAudioChannelAgent`.
    fn coerce_from(v: &nsIAudioChannelAgent) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAudioChannelAgentCoerce for nsIAudioChannelAgent {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgent) -> &Self {
        v
    }
}

impl nsIAudioChannelAgent {
    /// Cast this `nsIAudioChannelAgent` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAudioChannelAgentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAudioChannelAgent {
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
impl<T: nsISupportsCoerce> nsIAudioChannelAgentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAudioChannelAgent) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAudioChannelAgent
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAudioChannelAgentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void init (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback); */
    pub Init: unsafe extern "system" fn (this: *const nsIAudioChannelAgent, window: *const mozIDOMWindow, callback: *const nsIAudioChannelAgentCallback) -> ::nserror::nsresult,

    /* void initWithWeakCallback (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback); */
    pub InitWithWeakCallback: unsafe extern "system" fn (this: *const nsIAudioChannelAgent, window: *const mozIDOMWindow, callback: *const nsIAudioChannelAgentCallback) -> ::nserror::nsresult,

    /* void notifyStartedPlaying (in uint8_t audible); */
    pub NotifyStartedPlaying: unsafe extern "system" fn (this: *const nsIAudioChannelAgent, audible: uint8_t) -> ::nserror::nsresult,

    /* void notifyStoppedPlaying (); */
    pub NotifyStoppedPlaying: unsafe extern "system" fn (this: *const nsIAudioChannelAgent) -> ::nserror::nsresult,

    /* void notifyStartedAudible (in uint8_t audible, in uint32_t reason); */
    pub NotifyStartedAudible: unsafe extern "system" fn (this: *const nsIAudioChannelAgent, audible: uint8_t, reason: uint32_t) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAudioChannelAgent {

    pub const AUDIO_AGENT_STATE_NORMAL: i64 = 0;


    pub const AUDIO_AGENT_STATE_MUTED: i64 = 1;


    pub const AUDIO_AGENT_STATE_FADED: i64 = 2;

    /// ```text
    /// /**
    ///    * Initialize the agent with a channel type.
    ///    * Note: This function should only be called once.
    ///    *
    ///    * @param window
    ///    *    The window
    ///    * @param callback
    ///    *    1. Once the playable status changes, agent uses this callback function
    ///    *       to notify Gecko component.
    ///    *    2. The callback is allowed to be null. Ex: telephony doesn't need to
    ///    *       listen change of the playable status.
    ///    *    3. The AudioChannelAgent keeps a strong reference to the callback
    ///    *       object.
    ///    */
    /// ```
    ///

    /// `void init (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback);`
    #[inline]
    pub unsafe fn Init(&self, window: *const mozIDOMWindow, callback: *const nsIAudioChannelAgentCallback) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, window, callback)
    }


    /// ```text
    /// /**
    ///    * This method is just like init(), except the audio channel agent keeps a
    ///    * weak reference to the callback object.
    ///    *
    ///    * In order for this to work, |callback| must implement
    ///    * nsISupportsWeakReference.
    ///    */
    /// ```
    ///

    /// `void initWithWeakCallback (in mozIDOMWindow window, in nsIAudioChannelAgentCallback callback);`
    #[inline]
    pub unsafe fn InitWithWeakCallback(&self, window: *const mozIDOMWindow, callback: *const nsIAudioChannelAgentCallback) -> ::nserror::nsresult {
        ((*self.vtable).InitWithWeakCallback)(self, window, callback)
    }


    /// ```text
    /// /**
    ///    * Notify the agent that we want to start playing.
    ///    * Note: Gecko component SHOULD call this function first then start to
    ///    *          play audio stream only when return value is true.
    ///    */
    /// ```
    ///

    /// `void notifyStartedPlaying (in uint8_t audible);`
    #[inline]
    pub unsafe fn NotifyStartedPlaying(&self, audible: uint8_t) -> ::nserror::nsresult {
        ((*self.vtable).NotifyStartedPlaying)(self, audible)
    }


    /// ```text
    /// /**
    ///    * Notify the agent we no longer want to play.
    ///    *
    ///    * Note : even if notifyStartedPlaying() returned false, the agent would
    ///    * still be registered with the audio channel service and receive callbacks
    ///    * for status changes. So notifyStoppedPlaying must still eventually be
    ///    * called to unregister the agent with the channel service.
    ///    */
    /// ```
    ///

    /// `void notifyStoppedPlaying ();`
    #[inline]
    pub unsafe fn NotifyStoppedPlaying(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyStoppedPlaying)(self, )
    }


    /// ```text
    /// /**
    ///    * Notify agent that we already start producing audible data.
    ///    *
    ///    * Note : sometime audio might become silent during playing, this method is used to
    ///    * notify the actually audible state to other services which want to know
    ///    * about that, ex. tab sound indicator.
    ///    */
    /// ```
    ///

    /// `void notifyStartedAudible (in uint8_t audible, in uint32_t reason);`
    #[inline]
    pub unsafe fn NotifyStartedAudible(&self, audible: uint8_t, reason: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).NotifyStartedAudible)(self, audible, reason)
    }


}


