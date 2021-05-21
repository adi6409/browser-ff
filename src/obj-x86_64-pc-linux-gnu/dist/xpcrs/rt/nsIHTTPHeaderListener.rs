//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIHTTPHeaderListener.idl
//


/// `interface nsIHTTPHeaderListener : nsISupports`
///

/// ```text
/// /**
///  * The nsIHTTPHeaderListener interface allows plugin authors to
///  * access HTTP Response headers after issuing an
///  * nsIPluginHost::{GetURL,PostURL}() call. <P>
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHTTPHeaderListener {
    vtable: *const nsIHTTPHeaderListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHTTPHeaderListener.
unsafe impl XpCom for nsIHTTPHeaderListener {
    const IID: nsIID = nsID(0xea51e0b8, 0x871c, 0x4b85,
        [0x92, 0xda, 0x6f, 0x40, 0x03, 0x94, 0xc5, 0xec]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHTTPHeaderListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHTTPHeaderListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHTTPHeaderListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIHTTPHeaderListener`.
    fn coerce_from(v: &nsIHTTPHeaderListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHTTPHeaderListenerCoerce for nsIHTTPHeaderListener {
    #[inline]
    fn coerce_from(v: &nsIHTTPHeaderListener) -> &Self {
        v
    }
}

impl nsIHTTPHeaderListener {
    /// Cast this `nsIHTTPHeaderListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHTTPHeaderListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHTTPHeaderListener {
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
impl<T: nsISupportsCoerce> nsIHTTPHeaderListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTTPHeaderListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHTTPHeaderListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHTTPHeaderListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void newResponseHeader (in string headerName, in string headerValue); */
    pub NewResponseHeader: unsafe extern "system" fn (this: *const nsIHTTPHeaderListener, headerName: *const libc::c_char, headerValue: *const libc::c_char) -> ::nserror::nsresult,

    /* void statusLine (in string line); */
    pub StatusLine: unsafe extern "system" fn (this: *const nsIHTTPHeaderListener, line: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHTTPHeaderListener {

    /// ```text
    /// /**
    ///    * Called for each HTTP Response header.
    ///    * NOTE: You must copy the values of the params.
    ///    */
    /// ```
    ///

    /// `void newResponseHeader (in string headerName, in string headerValue);`
    #[inline]
    pub unsafe fn NewResponseHeader(&self, headerName: *const libc::c_char, headerValue: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).NewResponseHeader)(self, headerName, headerValue)
    }


    /// ```text
    /// /**
    ///    * Called once for the HTTP Response status line.
    ///    * Value does NOT include a terminating newline.
    ///    * NOTE: You must copy this value.
    ///    */
    /// ```
    ///

    /// `void statusLine (in string line);`
    #[inline]
    pub unsafe fn StatusLine(&self, line: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).StatusLine)(self, line)
    }


}


