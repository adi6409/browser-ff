//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/gmp/mozIGeckoMediaPluginService.idl
//


/// `interface mozIGeckoMediaPluginService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct mozIGeckoMediaPluginService {
    vtable: *const mozIGeckoMediaPluginServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for mozIGeckoMediaPluginService.
unsafe impl XpCom for mozIGeckoMediaPluginService {
    const IID: nsIID = nsID(0x44d362ae, 0x937a, 0x4803,
        [0xbe, 0xe6, 0xf2, 0x51, 0x2a, 0x01, 0x49, 0xd1]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for mozIGeckoMediaPluginService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from mozIGeckoMediaPluginService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait mozIGeckoMediaPluginServiceCoerce {
    /// Cheaply cast a value of this type from a `mozIGeckoMediaPluginService`.
    fn coerce_from(v: &mozIGeckoMediaPluginService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl mozIGeckoMediaPluginServiceCoerce for mozIGeckoMediaPluginService {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginService) -> &Self {
        v
    }
}

impl mozIGeckoMediaPluginService {
    /// Cast this `mozIGeckoMediaPluginService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: mozIGeckoMediaPluginServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for mozIGeckoMediaPluginService {
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
impl<T: nsISupportsCoerce> mozIGeckoMediaPluginServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &mozIGeckoMediaPluginService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every mozIGeckoMediaPluginService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct mozIGeckoMediaPluginServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsIThread thread; */
    pub GetThread: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginService, aThread: *mut *const nsIThread) -> ::nserror::nsresult,

    /* void RunPluginCrashCallbacks (in unsigned long pluginId, in ACString pluginName); */
    pub RunPluginCrashCallbacks: unsafe extern "system" fn (this: *const mozIGeckoMediaPluginService, pluginId: u32, pluginName: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* [noscript] boolean hasPluginForAPI (in ACString api, in TagArray tags); */
    /// Unable to generate binding because `native type nsTArray<nsCString> unsupported`
    pub HasPluginForAPI: *const ::libc::c_void,

    /* [noscript] void getGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoDecoderCallback callback); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetGMPVideoDecoder: *const ::libc::c_void,

    /* [noscript] void getDecryptingGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPVideoDecoderCallback callback, in uint32_t decryptorId); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetDecryptingGMPVideoDecoder: *const ::libc::c_void,

    /* [noscript] void getGMPVideoEncoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoEncoderCallback callback); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetGMPVideoEncoder: *const ::libc::c_void,

    /* [noscript] void getNodeId (in AString origin, in AString topLevelOrigin, in AString gmpName, in GetNodeIdCallback callback); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub GetNodeId: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl mozIGeckoMediaPluginService {

    /// ```text
    /// /**
    ///    * The GMP thread. Callable from any thread.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIThread thread;`
    #[inline]
    pub unsafe fn GetThread(&self, aThread: *mut *const nsIThread) -> ::nserror::nsresult {
        ((*self.vtable).GetThread)(self, aThread)
    }


    /// ```text
    /// /**
    ///    * Run through windows registered registered for pluginId, sending
    ///    * 'PluginCrashed' chrome-only event
    ///    */
    /// ```
    ///

    /// `void RunPluginCrashCallbacks (in unsigned long pluginId, in ACString pluginName);`
    #[inline]
    pub unsafe fn RunPluginCrashCallbacks(&self, pluginId: u32, pluginName: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).RunPluginCrashCallbacks)(self, pluginId, pluginName)
    }


    /// ```text
    /// /**
    ///    * Get a plugin that supports the specified tags.
    ///    * Callable on any thread
    ///    */
    /// ```
    ///

    /// `[noscript] boolean hasPluginForAPI (in ACString api, in TagArray tags);`
    const _HasPluginForAPI: () = ();

    /// ```text
    /// /**
    ///    * Get a video decoder that supports the specified tags.
    ///    * The array of tags should at least contain a codec tag, and optionally
    ///    * other tags such as for EME keysystem.
    ///    * Callable only on GMP thread.
    ///    * This is an asynchronous operation, the Done method of the callback object
    ///    * will be called on the GMP thread with the result (which might be null in
        ///    * the case of failure). This method always takes ownership of the callback
    ///    * object, but if this method returns an error then the Done method of the
    ///    * callback object will not be called at all.
    ///    */
    /// ```
    ///

    /// `[noscript] void getGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoDecoderCallback callback);`
    const _GetGMPVideoDecoder: () = ();

    /// ```text
    /// /**
    ///    * Gets a video decoder as per getGMPVideoDecoder, except it is linked to
    ///    * with a corresponding GMPDecryptor via the decryptor's ID.
    ///    * This is a temporary measure, until we can implement a Chromium CDM
    ///    * GMP protocol which does both decryption and decoding.
    ///    */
    /// ```
    ///

    /// `[noscript] void getDecryptingGMPVideoDecoder (in GMPCrashHelperPtr helper, in TagArray tags, in ACString nodeId, in GetGMPVideoDecoderCallback callback, in uint32_t decryptorId);`
    const _GetDecryptingGMPVideoDecoder: () = ();

    /// ```text
    /// /**
    ///    * Get a video encoder that supports the specified tags.
    ///    * The array of tags should at least contain a codec tag, and optionally
    ///    * other tags.
    ///    * Callable only on GMP thread.
    ///    * This is an asynchronous operation, the Done method of the callback object
    ///    * will be called on the GMP thread with the result (which might be null in
        ///    * the case of failure). This method always takes ownership of the callback
    ///    * object, but if this method returns an error then the Done method of the
    ///    * callback object will not be called at all.
    ///    */
    /// ```
    ///

    /// `[noscript] void getGMPVideoEncoder (in GMPCrashHelperPtr helper, in TagArray tags, [optional] in ACString nodeId, in GetGMPVideoEncoderCallback callback);`
    const _GetGMPVideoEncoder: () = ();

    /// ```text
    /// /**
    ///    * Gets the NodeId for a (origin, urlbarOrigin) pair.
    ///    */
    /// ```
    ///

    /// `[noscript] void getNodeId (in AString origin, in AString topLevelOrigin, in AString gmpName, in GetNodeIdCallback callback);`
    const _GetNodeId: () = ();

}


