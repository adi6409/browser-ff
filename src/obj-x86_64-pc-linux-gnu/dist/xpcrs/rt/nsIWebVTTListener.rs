//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/media/webvtt/nsIWebVTTListener.idl
//


/// `interface nsIWebVTTListener : nsISupports`
///

/// ```text
/// /**
///  * Listener for a JS WebVTT parser (vtt.js).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIWebVTTListener {
    vtable: *const nsIWebVTTListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIWebVTTListener.
unsafe impl XpCom for nsIWebVTTListener {
    const IID: nsIID = nsID(0x8a2d7780, 0x2045, 0x4a29,
        [0x99, 0xf4, 0xdf, 0x15, 0xca, 0xe5, 0xfc, 0x49]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIWebVTTListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIWebVTTListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIWebVTTListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIWebVTTListener`.
    fn coerce_from(v: &nsIWebVTTListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIWebVTTListenerCoerce for nsIWebVTTListener {
    #[inline]
    fn coerce_from(v: &nsIWebVTTListener) -> &Self {
        v
    }
}

impl nsIWebVTTListener {
    /// Cast this `nsIWebVTTListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIWebVTTListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIWebVTTListener {
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
impl<T: nsISupportsCoerce> nsIWebVTTListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIWebVTTListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIWebVTTListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIWebVTTListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void onCue (in jsval cue); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub OnCue: *const ::libc::c_void,

    /* [implicit_jscontext] void onRegion (in jsval region); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub OnRegion: *const ::libc::c_void,

    /* [implicit_jscontext] void onParsingError (in long errorCode); */
    /// Unable to generate binding because `jscontext is unsupported`
    pub OnParsingError: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIWebVTTListener {

    /// ```text
    /// /**
    ///    * Is called when the WebVTTParser successfully parses a WebVTT cue.
    ///    *
    ///    * @param cue An object representing the data of a parsed WebVTT cue.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void onCue (in jsval cue);`
    const _OnCue: () = ();

    /// ```text
    /// /**
    ///    * Is called when the WebVTT parser successfully parses a WebVTT region.
    ///    *
    ///    * @param region An object representing the data of a parsed
    ///    *               WebVTT region.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void onRegion (in jsval region);`
    const _OnRegion: () = ();

    /// ```text
    /// /**
    ///    * Is called when the WebVTT parser encounters a parsing error.
    ///    *
    ///    * @param error The error code of the ParserError the occured.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void onParsingError (in long errorCode);`
    const _OnParsingError: () = ();

}


