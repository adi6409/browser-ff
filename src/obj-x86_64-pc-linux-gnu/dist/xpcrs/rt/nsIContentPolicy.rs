//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIContentPolicy.idl
//


/// `interface nsIContentPolicy : nsISupports`
///

/// ```text
/// /**
///  * Interface for content policy mechanism.  Implementations of this
///  * interface can be used to control loading of various types of out-of-line
///  * content, or processing of certain types of in-line content.
///  *
///  * WARNING: do not block the caller from shouldLoad or shouldProcess (e.g.,
    ///  * by launching a dialog to prompt the user for something).
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentPolicy {
    vtable: *const nsIContentPolicyVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentPolicy.
unsafe impl XpCom for nsIContentPolicy {
    const IID: nsIID = nsID(0xcaad4f1f, 0xd047, 0x46ac,
        [0xae, 0x9d, 0xdc, 0x59, 0x8e, 0x4f, 0xb9, 0x1b]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentPolicy {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentPolicy.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentPolicyCoerce {
    /// Cheaply cast a value of this type from a `nsIContentPolicy`.
    fn coerce_from(v: &nsIContentPolicy) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentPolicyCoerce for nsIContentPolicy {
    #[inline]
    fn coerce_from(v: &nsIContentPolicy) -> &Self {
        v
    }
}

impl nsIContentPolicy {
    /// Cast this `nsIContentPolicy` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentPolicyCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentPolicy {
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
impl<T: nsISupportsCoerce> nsIContentPolicyCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentPolicy) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentPolicy
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentPolicyVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* short shouldLoad (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeTypeGuess); */
    pub ShouldLoad: unsafe extern "system" fn (this: *const nsIContentPolicy, aContentLocation: *const nsIURI, aLoadInfo: *const nsILoadInfo, aMimeTypeGuess: *const ::nsstring::nsACString, _retval: *mut i16) -> ::nserror::nsresult,

    /* short shouldProcess (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeType); */
    pub ShouldProcess: unsafe extern "system" fn (this: *const nsIContentPolicy, aContentLocation: *const nsIURI, aLoadInfo: *const nsILoadInfo, aMimeType: *const ::nsstring::nsACString, _retval: *mut i16) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentPolicy {
    /// ```text
    /// /**
    ///    * Returned from shouldLoad or shouldProcess if the load or process request
    ///    * is rejected based on details of the request.
    ///    */
    /// ```
    ///

    pub const REJECT_REQUEST: i64 = -1;

    /// ```text
    /// /**
    ///    * Returned from shouldLoad or shouldProcess if the load/process is rejected
    ///    * based solely on its type (of the above flags).
    ///    *
    ///    * NOTE that it is not meant to stop future requests for this type--only the
    ///    * current request.
    ///    */
    /// ```
    ///

    pub const REJECT_TYPE: i64 = -2;

    /// ```text
    /// /**
    ///    * Returned from shouldLoad or shouldProcess if the load/process is rejected
    ///    * based on the server it is hosted on or requested from (aContentLocation or
        ///    * aRequestOrigin), e.g., if you block an IMAGE because it is served from
    ///    * goatse.cx (even if you don't necessarily block other types from that
        ///    * server/domain).
    ///    *
    ///    * NOTE that it is not meant to stop future requests for this server--only the
    ///    * current request.
    ///    */
    /// ```
    ///

    pub const REJECT_SERVER: i64 = -3;

    /// ```text
    /// /**
    ///    * Returned from shouldLoad or shouldProcess if the load/process is rejected
    ///    * based on some other criteria. Mozilla callers will handle this like
    ///    * REJECT_REQUEST; third-party implementors may, for example, use this to
    ///    * direct their own callers to consult the extra parameter for additional
    ///    * details.
    ///    */
    /// ```
    ///

    pub const REJECT_OTHER: i64 = -4;

    /// ```text
    /// /**
    ///    * Returned from shouldLoad or shouldProcess if the load/process is forbiddden
    ///    * based on enterprise policy.
    ///    */
    /// ```
    ///

    pub const REJECT_POLICY: i64 = -5;

    /// ```text
    /// /**
    ///    * Returned from shouldLoad or shouldProcess if the load or process request
    ///    * is not rejected.
    ///    */
    /// ```
    ///

    pub const ACCEPT: i64 = 1;

    /// ```text
    /// /**
    ///    * Should the resource at this location be loaded?
    ///    * ShouldLoad will be called before loading the resource at aContentLocation
    ///    * to determine whether to start the load at all.
    ///    *
    ///    * @param aContentLocation  the location of the content being checked; must
    ///    *                          not be null
    ///    *
    ///    * @param aLoadInfo         the loadinfo of the channel being evaluated.
    ///    *
    ///    * @param aMimeTypeGuess    OPTIONAL. a guess for the requested content's
    ///    *                          MIME type, based on information available to
    ///    *                          the request initiator (e.g., an OBJECT's type
        ///    *                          attribute); does not reliably reflect the
    ///    *                          actual MIME type of the requested content
    ///    *
    ///    * @return ACCEPT or REJECT_*
    ///    *
    ///    * @note shouldLoad can be called while the DOM and layout of the document
    ///    * involved is in an inconsistent state.  This means that implementors of
    ///    * this method MUST NOT do any of the following:
    ///    * 1)  Modify the DOM in any way (e.g. setting attributes is a no-no).
///    * 2)  Query any DOM properties that depend on layout (e.g. offset*
///    *     properties).
///    * 3)  Query any DOM properties that depend on style (e.g. computed style).
///    * 4)  Query any DOM properties that depend on the current state of the DOM
///    *     outside the "context" node (e.g. lengths of node lists).
///    * 5)  [JavaScript implementations only] Access properties of any sort on any
///    *     object without using XPCNativeWrapper (either explicitly or
///    *     implicitly).  Due to various DOM0 things, this leads to item 4.
///    * If you do any of these things in your shouldLoad implementation, expect
///    * unpredictable behavior, possibly including crashes, content not showing
///    * up, content showing up doubled, etc.  If you need to do any of the things
///    * above, do them off timeout or event.
///    */
/// ```
///

/// `short shouldLoad (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeTypeGuess);`
#[inline]
pub unsafe fn ShouldLoad(&self, aContentLocation: *const nsIURI, aLoadInfo: *const nsILoadInfo, aMimeTypeGuess: *const ::nsstring::nsACString, _retval: *mut i16) -> ::nserror::nsresult {
((*self.vtable).ShouldLoad)(self, aContentLocation, aLoadInfo, aMimeTypeGuess, _retval)
}


/// ```text
/// /**
///    * Should the resource be processed?
///    * ShouldProcess will be called once all the information passed to it has
///    * been determined about the resource, typically after part of the resource
///    * has been loaded.
///    *
///    * @param aContentLocation  OPTIONAL; the location of the resource being
///    *                          requested: MAY be, e.g., a post-redirection URI
///    *                          for the resource.
///    *
///    * @param aLoadInfo         the loadinfo of the channel being evaluated.
///    *
///    * @param aMimeType         the MIME type of the requested resource (e.g.,
///    *                          image/png), as reported by the networking library,
///    *                          if available (may be empty if inappropriate for
///    *                          the type).
///    *
///    * @return ACCEPT or REJECT_*
///    *
///    * @note shouldProcess can be called while the DOM and layout of the document
///    * involved is in an inconsistent state.  See the note on shouldLoad to see
///    * what this means for implementors of this method.
///    */
/// ```
///

/// `short shouldProcess (in nsIURI aContentLocation, in nsILoadInfo aLoadInfo, in ACString aMimeType);`
#[inline]
pub unsafe fn ShouldProcess(&self, aContentLocation: *const nsIURI, aLoadInfo: *const nsILoadInfo, aMimeType: *const ::nsstring::nsACString, _retval: *mut i16) -> ::nserror::nsresult {
((*self.vtable).ShouldProcess)(self, aContentLocation, aLoadInfo, aMimeType, _retval)
}


}


/// `typedef nsIContentPolicy::nsContentPolicyType  nsContentPolicyType;`
///


pub type nsContentPolicyType =  u8;


