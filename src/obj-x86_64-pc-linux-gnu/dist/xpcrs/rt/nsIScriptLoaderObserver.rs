//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/script/nsIScriptLoaderObserver.idl
//


/// `interface nsIScriptLoaderObserver : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIScriptLoaderObserver {
    vtable: *const nsIScriptLoaderObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIScriptLoaderObserver.
unsafe impl XpCom for nsIScriptLoaderObserver {
    const IID: nsIID = nsID(0x7b787204, 0x76fb, 0x4764,
        [0x96, 0xf1, 0xfb, 0x7a, 0x66, 0x6d, 0xb4, 0xf4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIScriptLoaderObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIScriptLoaderObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIScriptLoaderObserverCoerce {
    /// Cheaply cast a value of this type from a `nsIScriptLoaderObserver`.
    fn coerce_from(v: &nsIScriptLoaderObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIScriptLoaderObserverCoerce for nsIScriptLoaderObserver {
    #[inline]
    fn coerce_from(v: &nsIScriptLoaderObserver) -> &Self {
        v
    }
}

impl nsIScriptLoaderObserver {
    /// Cast this `nsIScriptLoaderObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIScriptLoaderObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIScriptLoaderObserver {
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
impl<T: nsISupportsCoerce> nsIScriptLoaderObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIScriptLoaderObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIScriptLoaderObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIScriptLoaderObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInlineClassicScript, in nsIURI aURI, in int32_t aLineNo); */
    pub ScriptAvailable: unsafe extern "system" fn (this: *const nsIScriptLoaderObserver, aResult: ::nserror::nsresult, aElement: *const nsIScriptElement, aIsInlineClassicScript: bool, aURI: *const nsIURI, aLineNo: int32_t) -> ::nserror::nsresult,

    /* void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline); */
    pub ScriptEvaluated: unsafe extern "system" fn (this: *const nsIScriptLoaderObserver, aResult: ::nserror::nsresult, aElement: *const nsIScriptElement, aIsInline: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIScriptLoaderObserver {

    /// ```text
    /// /**
    ///    * The script is available for evaluation. For inline scripts, this
    ///    * method will be called synchronously. For externally loaded scripts,
    ///    * this method will be called when the load completes.
    ///    *
    ///    * @param aResult A result code representing the result of loading
    ///    *        a script. If this is a failure code, script evaluation
    ///    *        will not occur.
    ///    * @param aElement The element being processed.
    ///    * @param aIsInline Is this an inline classic script (as opposed to an
        ///    *        externally loaded classic script or module script)?
    ///    * @param aURI What is the URI of the script (the document URI if
        ///    *        it is inline).
    ///    * @param aLineNo At what line does the script appear (generally 1
        ///    *        if it is a loaded script).
    ///    */
    /// ```
    ///

    /// `void scriptAvailable (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInlineClassicScript, in nsIURI aURI, in int32_t aLineNo);`
    #[inline]
    pub unsafe fn ScriptAvailable(&self, aResult: ::nserror::nsresult, aElement: *const nsIScriptElement, aIsInlineClassicScript: bool, aURI: *const nsIURI, aLineNo: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).ScriptAvailable)(self, aResult, aElement, aIsInlineClassicScript, aURI, aLineNo)
    }


    /// ```text
    /// /**
    ///    * The script has been evaluated.
    ///    *
    ///    * @param aResult A result code representing the success or failure of
    ///    *        the script evaluation.
    ///    * @param aElement The element being processed.
    ///    * @param aIsInline Is this an inline script or externally loaded?
    ///    */
    /// ```
    ///

    /// `void scriptEvaluated (in nsresult aResult, in nsIScriptElement aElement, in boolean aIsInline);`
    #[inline]
    pub unsafe fn ScriptEvaluated(&self, aResult: ::nserror::nsresult, aElement: *const nsIScriptElement, aIsInline: bool) -> ::nserror::nsresult {
        ((*self.vtable).ScriptEvaluated)(self, aResult, aElement, aIsInline)
    }


}


