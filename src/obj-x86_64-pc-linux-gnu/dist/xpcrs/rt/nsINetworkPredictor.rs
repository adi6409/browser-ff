//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/base/nsINetworkPredictor.idl
//


/// `typedef uint32_t  PredictorPredictReason;`
///


pub type PredictorPredictReason = u32;


/// `typedef uint32_t  PredictorLearnReason;`
///


pub type PredictorLearnReason = u32;


/// `interface nsINetworkPredictor : nsISupports`
///

/// ```text
/// /**
///  * nsINetworkPredictor - learn about pages users visit, and allow us to take
///  *                       predictive actions upon future visits.
///  *                       NOTE: nsINetworkPredictor should only
///  *                       be used on the main thread.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINetworkPredictor {
    vtable: *const nsINetworkPredictorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINetworkPredictor.
unsafe impl XpCom for nsINetworkPredictor {
    const IID: nsIID = nsID(0xacc88e7c, 0x3f39, 0x42c7,
        [0xac, 0x31, 0x63, 0x77, 0xc2, 0xc3, 0xd7, 0x3e]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINetworkPredictor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINetworkPredictor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINetworkPredictorCoerce {
    /// Cheaply cast a value of this type from a `nsINetworkPredictor`.
    fn coerce_from(v: &nsINetworkPredictor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINetworkPredictorCoerce for nsINetworkPredictor {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictor) -> &Self {
        v
    }
}

impl nsINetworkPredictor {
    /// Cast this `nsINetworkPredictor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINetworkPredictorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINetworkPredictor {
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
impl<T: nsISupportsCoerce> nsINetworkPredictorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINetworkPredictor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINetworkPredictor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINetworkPredictorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [implicit_jscontext] void predict (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in jsval originAttributes, in nsINetworkPredictorVerifier verifier); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Predict: *const ::libc::c_void,

    /* [notxpcom] nsresult predictNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in OriginAttributes originAttributes, in nsINetworkPredictorVerifier verifier); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub PredictNative: *const ::libc::c_void,

    /* [implicit_jscontext] void learn (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in jsval originAttributes); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub Learn: *const ::libc::c_void,

    /* [notxpcom] nsresult learnNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in OriginAttributes originAttributes); */
    /// Unable to generate binding because `native type const mozilla::OriginAttributes unsupported`
    pub LearnNative: *const ::libc::c_void,

    /* void reset (); */
    pub Reset: unsafe extern "system" fn (this: *const nsINetworkPredictor) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINetworkPredictor {
    /// ```text
    /// /**
    ///    * Prediction reasons
    ///    *
    ///    * PREDICT_LINK - we are being asked to take predictive action because
    ///    * the user is hovering over a link.
    ///    *
    ///    * PREDICT_LOAD - we are being asked to take predictive action because
    ///    * the user has initiated a pageload.
    ///    *
    ///    * PREDICT_STARTUP - we are being asked to take predictive action
    ///    * because the browser is starting up.
    ///    */
    /// ```
    ///

    pub const PREDICT_LINK: i64 = 0;


    pub const PREDICT_LOAD: i64 = 1;


    pub const PREDICT_STARTUP: i64 = 2;


    pub const LEARN_LOAD_TOPLEVEL: i64 = 0;


    pub const LEARN_LOAD_SUBRESOURCE: i64 = 1;


    pub const LEARN_LOAD_REDIRECT: i64 = 2;


    pub const LEARN_STARTUP: i64 = 3;

    /// ```text
    /// /**
    ///    * Start taking predictive actions
    ///    *
    ///    * Calling this will cause the predictor to (possibly) start
    ///    * taking actions such as DNS prefetch and/or TCP preconnect based on
    ///    * (1) the host name that we are given, and (2) the reason we are being
    ///    * asked to take actions.
    ///    *
    ///    * @param targetURI - The URI we are being asked to take actions based on.
    ///    * @param sourceURI - The URI that is currently loaded. This is so we can
    ///    *   avoid doing predictive actions for link hover on an HTTPS page (for
        ///    *   example).
    ///    * @param reason - The reason we are being asked to take actions. Can be
    ///    *   any of the PREDICT_* values above.
    ///    *   In the case of PREDICT_LINK, targetURI should be the URI of the link
    ///    *   that is being hovered over, and sourceURI should be the URI of the page
    ///    *   on which the link appears.
    ///    *   In the case of PREDICT_LOAD, targetURI should be the URI of the page that
    ///    *   is being loaded and sourceURI should be null.
    ///    *   In the case of PREDICT_STARTUP, both targetURI and sourceURI should be
    ///    *   null.
    ///    * @param originAttributes - The originAttributes of the page load we are
    ///    *   predicting about.
    ///    * @param verifier - An nsINetworkPredictorVerifier used in testing to ensure
    ///    *   we're predicting the way we expect to. Not necessary (or desired) for
    ///    *   normal operation.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void predict (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in jsval originAttributes, in nsINetworkPredictorVerifier verifier);`
    const _Predict: () = ();


    /// `[notxpcom] nsresult predictNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorPredictReason reason, in OriginAttributes originAttributes, in nsINetworkPredictorVerifier verifier);`
    const _PredictNative: () = ();

    /// ```text
    /// /**
    ///    * Add to our compendium of knowledge
    ///    *
    ///    * This adds to our prediction database to make things (hopefully)
    ///    * smarter next time we predict something.
    ///    *
    ///    * @param targetURI - The URI that was loaded that we are keeping track of.
    ///    * @param sourceURI - The URI that caused targetURI to be loaded (for page
        ///    *   loads). This means the DOCUMENT URI.
    ///    * @param reason - The reason we are learning this bit of knowledge.
    ///    *   Reason can be any of the LEARN_* values.
    ///    *   In the case of LEARN_LOAD_SUBRESOURCE, targetURI should be the URI of a
    ///    *   subresource of a page, and sourceURI should be the top-level URI.
    ///    *   In the case of LEARN_LOAD_REDIRECT, targetURI is the NEW URI of a
    ///    *   top-level resource that was redirected to, and sourceURI is the
    ///    *   ORIGINAL URI of said top-level resource.
    ///    *   In the case of LEARN_STARTUP, targetURI should be the URI of a page
    ///    *   that was loaded immediately after browser startup, and sourceURI should
    ///    *   be null.
    ///    * @param originAttributes - The originAttributes for the page load that we
    ///    *   are learning about.
    ///    */
    /// ```
    ///

    /// `[implicit_jscontext] void learn (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in jsval originAttributes);`
    const _Learn: () = ();


    /// `[notxpcom] nsresult learnNative (in nsIURI targetURI, in nsIURI sourceURI, in PredictorLearnReason reason, in OriginAttributes originAttributes);`
    const _LearnNative: () = ();

    /// ```text
    /// /**
    ///    * Clear out all our learned knowledge
    ///    *
    ///    * This removes everything from our database so that any predictions begun
    ///    * after this completes will start from a blank slate.
    ///    */
    /// ```
    ///

    /// `void reset ();`
    #[inline]
    pub unsafe fn Reset(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Reset)(self, )
    }


}


