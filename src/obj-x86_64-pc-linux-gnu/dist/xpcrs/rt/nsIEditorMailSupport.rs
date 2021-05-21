//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIEditorMailSupport.idl
//


/// `interface nsIEditorMailSupport : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEditorMailSupport {
    vtable: *const nsIEditorMailSupportVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEditorMailSupport.
unsafe impl XpCom for nsIEditorMailSupport {
    const IID: nsIID = nsID(0xfdf23301, 0x4a94, 0x11d3,
        [0x9c, 0xe4, 0x99, 0x60, 0x49, 0x6c, 0x41, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEditorMailSupport {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEditorMailSupport.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEditorMailSupportCoerce {
    /// Cheaply cast a value of this type from a `nsIEditorMailSupport`.
    fn coerce_from(v: &nsIEditorMailSupport) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEditorMailSupportCoerce for nsIEditorMailSupport {
    #[inline]
    fn coerce_from(v: &nsIEditorMailSupport) -> &Self {
        v
    }
}

impl nsIEditorMailSupport {
    /// Cast this `nsIEditorMailSupport` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEditorMailSupportCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEditorMailSupport {
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
impl<T: nsISupportsCoerce> nsIEditorMailSupportCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditorMailSupport) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEditorMailSupport
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEditorMailSupportVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] Node insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML); */
    pub InsertAsCitedQuotation: unsafe extern "system" fn (this: *const nsIEditorMailSupport, aQuotedText: *const ::nsstring::nsAString, aCitation: *const ::nsstring::nsAString, aInsertHTML: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [can_run_script] void rewrap (in boolean aRespectNewlines); */
    pub Rewrap: unsafe extern "system" fn (this: *const nsIEditorMailSupport, aRespectNewlines: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEditorMailSupport {

    /// ```text
    /// /** Insert a string as quoted text
    ///     * (whose representation is dependant on the editor type),
    ///     * replacing the selected text (if any),
    ///     * including, if possible, a "cite" attribute.
    ///     * @param aQuotedText  The actual text to be quoted
    ///     * @param aCitation    The "mid" URL of the source message
    ///     * @param aInsertHTML  Insert as html?  (vs plaintext)
    ///     * @return             The node which was inserted
    ///     */
    /// ```
    ///

    /// `[can_run_script] Node insertAsCitedQuotation (in AString aQuotedText, in AString aCitation, in boolean aInsertHTML);`
    #[inline]
    pub unsafe fn InsertAsCitedQuotation(&self, aQuotedText: *const ::nsstring::nsAString, aCitation: *const ::nsstring::nsAString, aInsertHTML: bool, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).InsertAsCitedQuotation)(self, aQuotedText, aCitation, aInsertHTML, _retval)
    }


    /// ```text
    /// /**
    ///    * Rewrap the selected part of the document, re-quoting if necessary.
    ///    * @param aRespectNewlines  Try to maintain newlines in the original?
    ///    */
    /// ```
    ///

    /// `[can_run_script] void rewrap (in boolean aRespectNewlines);`
    #[inline]
    pub unsafe fn Rewrap(&self, aRespectNewlines: bool) -> ::nserror::nsresult {
        ((*self.vtable).Rewrap)(self, aRespectNewlines)
    }


}


