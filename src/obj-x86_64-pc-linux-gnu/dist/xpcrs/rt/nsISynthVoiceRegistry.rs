//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webspeech/synth/nsISynthVoiceRegistry.idl
//


/// `interface nsISynthVoiceRegistry : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISynthVoiceRegistry {
    vtable: *const nsISynthVoiceRegistryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISynthVoiceRegistry.
unsafe impl XpCom for nsISynthVoiceRegistry {
    const IID: nsIID = nsID(0x5d7a0b38, 0x77e5, 0x4ee5,
        [0x89, 0x7c, 0xce, 0x5d, 0xb9, 0xb8, 0x5d, 0x44]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISynthVoiceRegistry {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISynthVoiceRegistry.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISynthVoiceRegistryCoerce {
    /// Cheaply cast a value of this type from a `nsISynthVoiceRegistry`.
    fn coerce_from(v: &nsISynthVoiceRegistry) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISynthVoiceRegistryCoerce for nsISynthVoiceRegistry {
    #[inline]
    fn coerce_from(v: &nsISynthVoiceRegistry) -> &Self {
        v
    }
}

impl nsISynthVoiceRegistry {
    /// Cast this `nsISynthVoiceRegistry` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISynthVoiceRegistryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISynthVoiceRegistry {
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
impl<T: nsISupportsCoerce> nsISynthVoiceRegistryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISynthVoiceRegistry) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISynthVoiceRegistry
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISynthVoiceRegistryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void addVoice (in nsISpeechService aService, in AString aUri, in AString aName, in AString aLang, in boolean aLocalService, in boolean aQueuesUtterances); */
    pub AddVoice: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aService: *const nsISpeechService, aUri: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aLocalService: bool, aQueuesUtterances: bool) -> ::nserror::nsresult,

    /* void removeVoice (in nsISpeechService aService, in AString aUri); */
    pub RemoveVoice: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aService: *const nsISpeechService, aUri: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void notifyVoicesChanged (); */
    pub NotifyVoicesChanged: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry) -> ::nserror::nsresult,

    /* void setDefaultVoice (in AString aUri, in boolean aIsDefault); */
    pub SetDefaultVoice: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aUri: *const ::nsstring::nsAString, aIsDefault: bool) -> ::nserror::nsresult,

    /* readonly attribute uint32_t voiceCount; */
    pub GetVoiceCount: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aVoiceCount: *mut uint32_t) -> ::nserror::nsresult,

    /* AString getVoice (in uint32_t aIndex); */
    pub GetVoice: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aIndex: uint32_t, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* bool isDefaultVoice (in AString aUri); */
    pub IsDefaultVoice: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aUri: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* bool isLocalVoice (in AString aUri); */
    pub IsLocalVoice: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aUri: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString getVoiceLang (in AString aUri); */
    pub GetVoiceLang: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aUri: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* AString getVoiceName (in AString aUri); */
    pub GetVoiceName: unsafe extern "system" fn (this: *const nsISynthVoiceRegistry, aUri: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISynthVoiceRegistry {

    /// ```text
    /// /**
    ///    * Register a speech synthesis voice.
    ///    *
    ///    * @param aService          the service that provides this voice.
    ///    * @param aUri              a unique identifier for this voice.
    ///    * @param aName             human-readable name for this voice.
    ///    * @param aLang             a BCP 47 language tag.
    ///    * @param aLocalService     true if service does not require network.
    ///    * @param aQueuesUtterances true if voice only speaks one utterance at a time
    ///    */
    /// ```
    ///

    /// `void addVoice (in nsISpeechService aService, in AString aUri, in AString aName, in AString aLang, in boolean aLocalService, in boolean aQueuesUtterances);`
    #[inline]
    pub unsafe fn AddVoice(&self, aService: *const nsISpeechService, aUri: *const ::nsstring::nsAString, aName: *const ::nsstring::nsAString, aLang: *const ::nsstring::nsAString, aLocalService: bool, aQueuesUtterances: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddVoice)(self, aService, aUri, aName, aLang, aLocalService, aQueuesUtterances)
    }


    /// ```text
    /// /**
    ///    * Remove a speech synthesis voice.
    ///    *
    ///    * @param aService the service that was used to add the voice.
    ///    * @param aUri     a unique identifier of an existing voice.
    ///    */
    /// ```
    ///

    /// `void removeVoice (in nsISpeechService aService, in AString aUri);`
    #[inline]
    pub unsafe fn RemoveVoice(&self, aService: *const nsISpeechService, aUri: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).RemoveVoice)(self, aService, aUri)
    }


    /// ```text
    /// /**
    ///    * Notify content of voice availability changes. This allows content
    ///    * to be notified of voice catalog changes in real time.
    ///    */
    /// ```
    ///

    /// `void notifyVoicesChanged ();`
    #[inline]
    pub unsafe fn NotifyVoicesChanged(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyVoicesChanged)(self, )
    }


    /// ```text
    /// /**
    ///    * Set a voice as default.
    ///    *
    ///    * @param aUri       a unique identifier of an existing voice.
    ///    * @param aIsDefault true if this voice should be toggled as default.
    ///    */
    /// ```
    ///

    /// `void setDefaultVoice (in AString aUri, in boolean aIsDefault);`
    #[inline]
    pub unsafe fn SetDefaultVoice(&self, aUri: *const ::nsstring::nsAString, aIsDefault: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDefaultVoice)(self, aUri, aIsDefault)
    }



    /// `readonly attribute uint32_t voiceCount;`
    #[inline]
    pub unsafe fn GetVoiceCount(&self, aVoiceCount: *mut uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetVoiceCount)(self, aVoiceCount)
    }



    /// `AString getVoice (in uint32_t aIndex);`
    #[inline]
    pub unsafe fn GetVoice(&self, aIndex: uint32_t, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetVoice)(self, aIndex, _retval)
    }



    /// `bool isDefaultVoice (in AString aUri);`
    #[inline]
    pub unsafe fn IsDefaultVoice(&self, aUri: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDefaultVoice)(self, aUri, _retval)
    }



    /// `bool isLocalVoice (in AString aUri);`
    #[inline]
    pub unsafe fn IsLocalVoice(&self, aUri: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsLocalVoice)(self, aUri, _retval)
    }



    /// `AString getVoiceLang (in AString aUri);`
    #[inline]
    pub unsafe fn GetVoiceLang(&self, aUri: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetVoiceLang)(self, aUri, _retval)
    }



    /// `AString getVoiceName (in AString aUri);`
    #[inline]
    pub unsafe fn GetVoiceName(&self, aUri: *const ::nsstring::nsAString, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetVoiceName)(self, aUri, _retval)
    }


}


