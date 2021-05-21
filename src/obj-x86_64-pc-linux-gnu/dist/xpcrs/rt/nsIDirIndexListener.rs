//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/streamconv/nsIDirIndexListener.idl
//


/// `interface nsIDirIndexListener : nsISupports`
///

/// ```text
/// /**
///  * This interface is used to receive contents of directory index listings
///  * from a protocol. They can then be transformed into an output format
///  * (such as rdf, html, etc)
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirIndexListener {
    vtable: *const nsIDirIndexListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirIndexListener.
unsafe impl XpCom for nsIDirIndexListener {
    const IID: nsIID = nsID(0xfae4e9a8, 0x1dd1, 0x11b2,
        [0xb5, 0x3c, 0x8f, 0x3a, 0xa1, 0xbb, 0xf8, 0xf5]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirIndexListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirIndexListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirIndexListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDirIndexListener`.
    fn coerce_from(v: &nsIDirIndexListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirIndexListenerCoerce for nsIDirIndexListener {
    #[inline]
    fn coerce_from(v: &nsIDirIndexListener) -> &Self {
        v
    }
}

impl nsIDirIndexListener {
    /// Cast this `nsIDirIndexListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirIndexListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirIndexListener {
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
impl<T: nsISupportsCoerce> nsIDirIndexListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirIndexListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirIndexListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirIndexListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex); */
    pub OnIndexAvailable: unsafe extern "system" fn (this: *const nsIDirIndexListener, aRequest: *const nsIRequest, aCtxt: *const nsISupports, aIndex: *const nsIDirIndex) -> ::nserror::nsresult,

    /* void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo); */
    pub OnInformationAvailable: unsafe extern "system" fn (this: *const nsIDirIndexListener, aRequest: *const nsIRequest, aCtxt: *const nsISupports, aInfo: *const ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirIndexListener {

    /// ```text
    /// /**
    ///      * Called for each directory entry
    ///      *
    ///      * @param request - the request
    ///      * @param ctxt - opaque parameter
    ///      * @param index - new index to add
    ///      */
    /// ```
    ///

    /// `void onIndexAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in nsIDirIndex aIndex);`
    #[inline]
    pub unsafe fn OnIndexAvailable(&self, aRequest: *const nsIRequest, aCtxt: *const nsISupports, aIndex: *const nsIDirIndex) -> ::nserror::nsresult {
        ((*self.vtable).OnIndexAvailable)(self, aRequest, aCtxt, aIndex)
    }


    /// ```text
    /// /**
    ///      * Called for each information line
    ///      *
    ///      * @param request - the request
    ///      * @param ctxt - opaque parameter
    ///      * @param info - new info to add
    ///      */
    /// ```
    ///

    /// `void onInformationAvailable (in nsIRequest aRequest, in nsISupports aCtxt, in AString aInfo);`
    #[inline]
    pub unsafe fn OnInformationAvailable(&self, aRequest: *const nsIRequest, aCtxt: *const nsISupports, aInfo: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnInformationAvailable)(self, aRequest, aCtxt, aInfo)
    }


}


/// `interface nsIDirIndexParser : nsIStreamListener`
///

/// ```text
/// /**
///  * A parser for application/http-index-format
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDirIndexParser {
    vtable: *const nsIDirIndexParserVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDirIndexParser.
unsafe impl XpCom for nsIDirIndexParser {
    const IID: nsIID = nsID(0x38e3066c, 0x1dd2, 0x11b2,
        [0x9b, 0x59, 0x8b, 0xe5, 0x15, 0xc1, 0xee, 0x3f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDirIndexParser {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDirIndexParser.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDirIndexParserCoerce {
    /// Cheaply cast a value of this type from a `nsIDirIndexParser`.
    fn coerce_from(v: &nsIDirIndexParser) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDirIndexParserCoerce for nsIDirIndexParser {
    #[inline]
    fn coerce_from(v: &nsIDirIndexParser) -> &Self {
        v
    }
}

impl nsIDirIndexParser {
    /// Cast this `nsIDirIndexParser` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDirIndexParserCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDirIndexParser {
    type Target = nsIStreamListener;
    #[inline]
    fn deref(&self) -> &nsIStreamListener {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsIStreamListenerCoerce> nsIDirIndexParserCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDirIndexParser) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDirIndexParser
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDirIndexParserVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsIStreamListenerVTable,

    /* attribute nsIDirIndexListener listener; */
    pub GetListener: unsafe extern "system" fn (this: *const nsIDirIndexParser, aListener: *mut *const nsIDirIndexListener) -> ::nserror::nsresult,

    /* attribute nsIDirIndexListener listener; */
    pub SetListener: unsafe extern "system" fn (this: *const nsIDirIndexParser, aListener: *const nsIDirIndexListener) -> ::nserror::nsresult,

    /* readonly attribute string comment; */
    pub GetComment: unsafe extern "system" fn (this: *const nsIDirIndexParser, aComment: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* attribute string encoding; */
    pub GetEncoding: unsafe extern "system" fn (this: *const nsIDirIndexParser, aEncoding: *mut *const libc::c_char) -> ::nserror::nsresult,

    /* attribute string encoding; */
    pub SetEncoding: unsafe extern "system" fn (this: *const nsIDirIndexParser, aEncoding: *const libc::c_char) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDirIndexParser {

    /// ```text
    /// /**
    ///      * The interface to use as a callback for new entries
    ///      */
    /// ```
    ///

    /// `attribute nsIDirIndexListener listener;`
    #[inline]
    pub unsafe fn GetListener(&self, aListener: *mut *const nsIDirIndexListener) -> ::nserror::nsresult {
        ((*self.vtable).GetListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///      * The interface to use as a callback for new entries
    ///      */
    /// ```
    ///

    /// `attribute nsIDirIndexListener listener;`
    #[inline]
    pub unsafe fn SetListener(&self, aListener: *const nsIDirIndexListener) -> ::nserror::nsresult {
        ((*self.vtable).SetListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///      * The comment given, if any
    ///      * This result is only valid _after_ OnStopRequest has occurred,
    ///      * because it can occur anywhere in the datastream
    ///      */
    /// ```
    ///

    /// `readonly attribute string comment;`
    #[inline]
    pub unsafe fn GetComment(&self, aComment: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetComment)(self, aComment)
    }


    /// ```text
    /// /**
    ///      * The encoding to use
    ///      */
    /// ```
    ///

    /// `attribute string encoding;`
    #[inline]
    pub unsafe fn GetEncoding(&self, aEncoding: *mut *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).GetEncoding)(self, aEncoding)
    }


    /// ```text
    /// /**
    ///      * The encoding to use
    ///      */
    /// ```
    ///

    /// `attribute string encoding;`
    #[inline]
    pub unsafe fn SetEncoding(&self, aEncoding: *const libc::c_char) -> ::nserror::nsresult {
        ((*self.vtable).SetEncoding)(self, aEncoding)
    }


}


