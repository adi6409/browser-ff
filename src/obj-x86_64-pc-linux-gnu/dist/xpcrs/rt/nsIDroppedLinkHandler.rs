//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/base/nsIDroppedLinkHandler.idl
//


/// `interface nsIDroppedLinkItem : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDroppedLinkItem {
    vtable: *const nsIDroppedLinkItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDroppedLinkItem.
unsafe impl XpCom for nsIDroppedLinkItem {
    const IID: nsIID = nsID(0x69e14f91, 0x2e09, 0x4ca6,
        [0xa5, 0x11, 0xa7, 0x15, 0xc9, 0x9a, 0x28, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDroppedLinkItem {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDroppedLinkItem.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDroppedLinkItemCoerce {
    /// Cheaply cast a value of this type from a `nsIDroppedLinkItem`.
    fn coerce_from(v: &nsIDroppedLinkItem) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDroppedLinkItemCoerce for nsIDroppedLinkItem {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkItem) -> &Self {
        v
    }
}

impl nsIDroppedLinkItem {
    /// Cast this `nsIDroppedLinkItem` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDroppedLinkItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDroppedLinkItem {
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
impl<T: nsISupportsCoerce> nsIDroppedLinkItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkItem) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDroppedLinkItem
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDroppedLinkItemVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute AString url; */
    pub GetUrl: unsafe extern "system" fn (this: *const nsIDroppedLinkItem, aUrl: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIDroppedLinkItem, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute AString type; */
    pub GetType: unsafe extern "system" fn (this: *const nsIDroppedLinkItem, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDroppedLinkItem {

    /// ```text
    /// /**
    ///    * Returns the URL of the link.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString url;`
    #[inline]
    pub unsafe fn GetUrl(&self, aUrl: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetUrl)(self, aUrl)
    }


    /// ```text
    /// /**
    ///    * Returns the link name.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }


    /// ```text
    /// /**
    ///    * Returns the MIME-Type.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


}


/// `interface nsIDroppedLinkHandler : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDroppedLinkHandler {
    vtable: *const nsIDroppedLinkHandlerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDroppedLinkHandler.
unsafe impl XpCom for nsIDroppedLinkHandler {
    const IID: nsIID = nsID(0x21b5c25a, 0x28a9, 0x47bd,
        [0x84, 0x31, 0xfa, 0x91, 0x16, 0x30, 0x5d, 0xed]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDroppedLinkHandler {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDroppedLinkHandler.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDroppedLinkHandlerCoerce {
    /// Cheaply cast a value of this type from a `nsIDroppedLinkHandler`.
    fn coerce_from(v: &nsIDroppedLinkHandler) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDroppedLinkHandlerCoerce for nsIDroppedLinkHandler {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkHandler) -> &Self {
        v
    }
}

impl nsIDroppedLinkHandler {
    /// Cast this `nsIDroppedLinkHandler` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDroppedLinkHandlerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDroppedLinkHandler {
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
impl<T: nsISupportsCoerce> nsIDroppedLinkHandlerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDroppedLinkHandler) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDroppedLinkHandler
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDroppedLinkHandlerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean canDropLink (in DragEvent aEvent, in boolean aAllowSameDocument); */
    pub CanDropLink: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aEvent: *const libc::c_void, aAllowSameDocument: bool, _retval: *mut bool) -> ::nserror::nsresult,

    /* AString dropLink (in DragEvent aEvent, out AString aName, [optional] in boolean aDisallowInherit); */
    pub DropLink: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aEvent: *const libc::c_void, aName: *mut ::nsstring::nsAString, aDisallowInherit: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* Array<nsIDroppedLinkItem> dropLinks (in DragEvent aEvent, [optional] in boolean aDisallowInherit); */
    pub DropLinks: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aEvent: *const libc::c_void, aDisallowInherit: bool, _retval: *mut thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>) -> ::nserror::nsresult,

    /* void validateURIsForDrop (in DragEvent aEvent, in Array<AString> aURIs, [optional] in boolean aDisallowInherit); */
    pub ValidateURIsForDrop: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aEvent: *const libc::c_void, aURIs: *const thin_vec::ThinVec<::nsstring::nsString>, aDisallowInherit: bool) -> ::nserror::nsresult,

    /* Array<nsIDroppedLinkItem> queryLinks (in DataTransfer aDataTransfer); */
    pub QueryLinks: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aDataTransfer: *const libc::c_void, _retval: *mut thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>) -> ::nserror::nsresult,

    /* nsIPrincipal getTriggeringPrincipal (in DragEvent aEvent); */
    pub GetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aEvent: *const libc::c_void, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult,

    /* nsIContentSecurityPolicy getCSP (in DragEvent aEvent); */
    pub GetCSP: unsafe extern "system" fn (this: *const nsIDroppedLinkHandler, aEvent: *const libc::c_void, _retval: *mut *const nsIContentSecurityPolicy) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDroppedLinkHandler {

    /// ```text
    /// /**
    ///    * Determines if a link being dragged can be dropped and returns true if so.
    ///    * aEvent should be a dragenter or dragover event.
    ///    *
    ///    * If aAllowSameDocument is false, drops are only allowed if the document
    ///    * of the source of the drag is different from the destination. This check
    ///    * includes any parent, sibling and child frames in the same content tree.
    ///    * If true, the source is not checked.
    ///    */
    /// ```
    ///

    /// `boolean canDropLink (in DragEvent aEvent, in boolean aAllowSameDocument);`
    #[inline]
    pub unsafe fn CanDropLink(&self, aEvent: *const libc::c_void, aAllowSameDocument: bool, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).CanDropLink)(self, aEvent, aAllowSameDocument, _retval)
    }


    /// ```text
    /// /**
    ///    * Given a drop event aEvent, determines the link being dragged and returns
    ///    * it. If a uri is returned the caller can, for instance, load it. If null
    ///    * is returned, there is no valid link to be dropped.
    ///    *
    ///    * A NS_ERROR_DOM_SECURITY_ERR error will be thrown and the event cancelled if
    ///    * the receiving target should not load the uri for security reasons. This
    ///    * will occur if any of the following conditions are true:
    ///    *  - the source of the drag initiated a link for dragging that
    ///    *    it itself cannot access. This prevents a source document from tricking
    ///    *    the user into a dragging a chrome url, for example.
    ///    *  - aDisallowInherit is true, and the URI being dropped would inherit the
    ///    *    current document's security context (URI_INHERITS_SECURITY_CONTEXT).
    ///    *
    ///    * aName is filled in with the link title if it exists, or an empty string
    ///    * otherwise.
    ///    */
    /// ```
    ///

    /// `AString dropLink (in DragEvent aEvent, out AString aName, [optional] in boolean aDisallowInherit);`
    #[inline]
    pub unsafe fn DropLink(&self, aEvent: *const libc::c_void, aName: *mut ::nsstring::nsAString, aDisallowInherit: bool, _retval: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).DropLink)(self, aEvent, aName, aDisallowInherit, _retval)
    }


    /// ```text
    /// /**
    ///    * Given a drop event aEvent, determines links being dragged and returns
    ///    * them. If links are returned the caller can, for instance, load them. If
    ///    * the returned array is empty, there is no valid link to be dropped.
    ///    *
    ///    * A NS_ERROR_DOM_SECURITY_ERR error will be thrown and the event cancelled if
    ///    * the receiving target should not load the uri for security reasons. This
    ///    * will occur if any of the following conditions are true:
    ///    *  - the source of the drag initiated a link for dragging that
    ///    *    it itself cannot access. This prevents a source document from tricking
    ///    *    the user into a dragging a chrome url, for example.
    ///    *  - aDisallowInherit is true, and the URI being dropped would inherit the
    ///    *    current document's security context (URI_INHERITS_SECURITY_CONTEXT).
    ///    */
    /// ```
    ///

    /// `Array<nsIDroppedLinkItem> dropLinks (in DragEvent aEvent, [optional] in boolean aDisallowInherit);`
    #[inline]
    pub unsafe fn DropLinks(&self, aEvent: *const libc::c_void, aDisallowInherit: bool, _retval: *mut thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>) -> ::nserror::nsresult {
        ((*self.vtable).DropLinks)(self, aEvent, aDisallowInherit, _retval)
    }


    /// ```text
    /// /**
    ///    * Given a drop event aEvent, validate the extra URIs for the event,
    ///    * this is used when the caller extracts yet another URIs from the dropped
    ///    * text, like home button that splits the text with "|".
    ///    */
    /// ```
    ///

    /// `void validateURIsForDrop (in DragEvent aEvent, in Array<AString> aURIs, [optional] in boolean aDisallowInherit);`
    #[inline]
    pub unsafe fn ValidateURIsForDrop(&self, aEvent: *const libc::c_void, aURIs: *const thin_vec::ThinVec<::nsstring::nsString>, aDisallowInherit: bool) -> ::nserror::nsresult {
        ((*self.vtable).ValidateURIsForDrop)(self, aEvent, aURIs, aDisallowInherit)
    }


    /// ```text
    /// /**
    ///    * Given a dataTransfer, allows caller to determine and verify links being
    ///    * dragged. Since drag/drop performs a roundtrip of parent, child, parent,
    ///    * it allows the parent to verify that the child did not modify links
    ///    * being dropped.
    ///    */
    /// ```
    ///

    /// `Array<nsIDroppedLinkItem> queryLinks (in DataTransfer aDataTransfer);`
    #[inline]
    pub unsafe fn QueryLinks(&self, aDataTransfer: *const libc::c_void, _retval: *mut thin_vec::ThinVec<RefPtr<nsIDroppedLinkItem>>) -> ::nserror::nsresult {
        ((*self.vtable).QueryLinks)(self, aDataTransfer, _retval)
    }


    /// ```text
    /// /**
    ///    * Given a drop event aEvent, determines the triggering principal for the
    ///    * event and returns it.
    ///    */
    /// ```
    ///

    /// `nsIPrincipal getTriggeringPrincipal (in DragEvent aEvent);`
    #[inline]
    pub unsafe fn GetTriggeringPrincipal(&self, aEvent: *const libc::c_void, _retval: *mut *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetTriggeringPrincipal)(self, aEvent, _retval)
    }


    /// ```text
    /// /**
    ///    * Given a drop event aEvent, determines the CSP for the event and returns it.
    ///    */
    /// ```
    ///

    /// `nsIContentSecurityPolicy getCSP (in DragEvent aEvent);`
    #[inline]
    pub unsafe fn GetCSP(&self, aEvent: *const libc::c_void, _retval: *mut *const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetCSP)(self, aEvent, _retval)
    }


}


