//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/netwerk/protocol/http/nsIHttpHeaderVisitor.idl
//


/// `interface nsIHttpHeaderVisitor : nsISupports`
///

/// ```text
/// /**
///  * Implement this interface to visit http headers.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHttpHeaderVisitor {
    vtable: *const nsIHttpHeaderVisitorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHttpHeaderVisitor.
unsafe impl XpCom for nsIHttpHeaderVisitor {
    const IID: nsIID = nsID(0x35412859, 0xb9d9, 0x423c,
        [0x88, 0x66, 0x2d, 0x45, 0x59, 0xfd, 0xd2, 0xbe]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHttpHeaderVisitor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHttpHeaderVisitor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHttpHeaderVisitorCoerce {
    /// Cheaply cast a value of this type from a `nsIHttpHeaderVisitor`.
    fn coerce_from(v: &nsIHttpHeaderVisitor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHttpHeaderVisitorCoerce for nsIHttpHeaderVisitor {
    #[inline]
    fn coerce_from(v: &nsIHttpHeaderVisitor) -> &Self {
        v
    }
}

impl nsIHttpHeaderVisitor {
    /// Cast this `nsIHttpHeaderVisitor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHttpHeaderVisitorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHttpHeaderVisitor {
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
impl<T: nsISupportsCoerce> nsIHttpHeaderVisitorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHttpHeaderVisitor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHttpHeaderVisitor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHttpHeaderVisitorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [must_use] void visitHeader (in ACString aHeader, in ACString aValue); */
    pub VisitHeader: unsafe extern "system" fn (this: *const nsIHttpHeaderVisitor, aHeader: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHttpHeaderVisitor {

    /// ```text
    /// /**
    ///      * Called by the nsIHttpChannel implementation when visiting request and
    ///      * response headers.
    ///      *
    ///      * @param aHeader
    ///      *        the header being visited.
    ///      * @param aValue
    ///      *        the header value (possibly a comma delimited list).
    ///      *
    ///      * @throw any exception to terminate enumeration
    ///      */
    /// ```
    ///

    /// `[must_use] void visitHeader (in ACString aHeader, in ACString aValue);`
    #[inline]
    pub unsafe fn VisitHeader(&self, aHeader: *const ::nsstring::nsACString, aValue: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).VisitHeader)(self, aHeader, aValue)
    }


}


