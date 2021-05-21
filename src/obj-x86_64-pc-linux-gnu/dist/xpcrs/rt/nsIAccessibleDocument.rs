//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/accessible/interfaces/nsIAccessibleDocument.idl
//


/// `interface nsIAccessibleDocument : nsISupports`
///

/// ```text
/// /**
///  * An interface for in-process accessibility clients
///  * that wish to retrieve information about a document.
///  * When accessibility is turned on in Gecko,
///  * there is an nsIAccessibleDocument for each document
///  * whether it is XUL, HTML or whatever.
///  * You can QueryInterface to nsIAccessibleDocument from the nsIAccessible for
///  * the root node of a document or you can get one from
///  * nsIAccessible::GetDocument().
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIAccessibleDocument {
    vtable: *const nsIAccessibleDocumentVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIAccessibleDocument.
unsafe impl XpCom for nsIAccessibleDocument {
    const IID: nsIID = nsID(0x5cad5f91, 0xfcce, 0x40e7,
        [0x91, 0x3e, 0x46, 0x71, 0x70, 0x1d, 0x19, 0xb4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIAccessibleDocument {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIAccessibleDocument.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIAccessibleDocumentCoerce {
    /// Cheaply cast a value of this type from a `nsIAccessibleDocument`.
    fn coerce_from(v: &nsIAccessibleDocument) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIAccessibleDocumentCoerce for nsIAccessibleDocument {
    #[inline]
    fn coerce_from(v: &nsIAccessibleDocument) -> &Self {
        v
    }
}

impl nsIAccessibleDocument {
    /// Cast this `nsIAccessibleDocument` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIAccessibleDocumentCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIAccessibleDocument {
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
impl<T: nsISupportsCoerce> nsIAccessibleDocumentCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIAccessibleDocument) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIAccessibleDocument
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIAccessibleDocumentVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString URL; */
    pub GetURL: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString mimeType; */
    pub GetMimeType: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aMimeType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString docType; */
    pub GetDocType: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aDocType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute Document DOMDocument; */
    pub GetDOMDocument: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aDOMDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute mozIDOMWindowProxy window; */
    pub GetWindow: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessibleDocument parentDocument; */
    pub GetParentDocument: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aParentDocument: *mut *const nsIAccessibleDocument) -> ::nserror::nsresult,

    /* readonly attribute unsigned long childDocumentCount; */
    pub GetChildDocumentCount: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aChildDocumentCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute nsIAccessiblePivot virtualCursor; */
    pub GetVirtualCursor: unsafe extern "system" fn (this: *const nsIAccessibleDocument, aVirtualCursor: *mut*const nsIAccessiblePivot) -> ::nserror::nsresult,

    /* nsIAccessibleDocument getChildDocumentAt (in unsigned long index); */
    pub GetChildDocumentAt: unsafe extern "system" fn (this: *const nsIAccessibleDocument, index: u32, _retval: *mut *const nsIAccessibleDocument) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIAccessibleDocument {

    /// ```text
    /// /**
    ///    * The URL of the document
    ///    */
    /// ```
    ///

    /// `readonly attribute AString URL;`
    #[inline]
    pub unsafe fn GetURL(&self, aURL: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetURL)(self, aURL)
    }


    /// ```text
    /// /**
    ///    * The title of the document, as specified in the document.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///    * The mime type of the document
    ///    */
    /// ```
    ///

    /// `readonly attribute AString mimeType;`
    #[inline]
    pub unsafe fn GetMimeType(&self, aMimeType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetMimeType)(self, aMimeType)
    }


    /// ```text
    /// /**
    ///    * The doc type of the document, as specified in the document.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString docType;`
    #[inline]
    pub unsafe fn GetDocType(&self, aDocType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetDocType)(self, aDocType)
    }


    /// ```text
    /// /**
    ///    * The Document interface associated with this document.
    ///    */
    /// ```
    ///

    /// `readonly attribute Document DOMDocument;`
    #[inline]
    pub unsafe fn GetDOMDocument(&self, aDOMDocument: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetDOMDocument)(self, aDOMDocument)
    }


    /// ```text
    /// /**
    ///    * The nsIDOMWindow that the document resides in.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy window;`
    #[inline]
    pub unsafe fn GetWindow(&self, aWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetWindow)(self, aWindow)
    }


    /// ```text
    /// /**
    ///    * Return the parent document accessible.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessibleDocument parentDocument;`
    #[inline]
    pub unsafe fn GetParentDocument(&self, aParentDocument: *mut *const nsIAccessibleDocument) -> ::nserror::nsresult {
        ((*self.vtable).GetParentDocument)(self, aParentDocument)
    }


    /// ```text
    /// /**
    ///    * Return the count of child document accessibles.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long childDocumentCount;`
    #[inline]
    pub unsafe fn GetChildDocumentCount(&self, aChildDocumentCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetChildDocumentCount)(self, aChildDocumentCount)
    }


    /// ```text
    /// /**
    ///    * The virtual cursor pivot this document manages.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsIAccessiblePivot virtualCursor;`
    #[inline]
    pub unsafe fn GetVirtualCursor(&self, aVirtualCursor: *mut*const nsIAccessiblePivot) -> ::nserror::nsresult {
        ((*self.vtable).GetVirtualCursor)(self, aVirtualCursor)
    }


    /// ```text
    /// /**
    ///    * Return the child document accessible at the given index.
    ///    */
    /// ```
    ///

    /// `nsIAccessibleDocument getChildDocumentAt (in unsigned long index);`
    #[inline]
    pub unsafe fn GetChildDocumentAt(&self, index: u32, _retval: *mut *const nsIAccessibleDocument) -> ::nserror::nsresult {
        ((*self.vtable).GetChildDocumentAt)(self, index, _retval)
    }


}


