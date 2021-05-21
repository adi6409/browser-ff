//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/composer/nsIEditingSession.idl
//


/// `interface nsIEditingSession : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIEditingSession {
    vtable: *const nsIEditingSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIEditingSession.
unsafe impl XpCom for nsIEditingSession {
    const IID: nsIID = nsID(0x24f963d1, 0xe6fc, 0x43ea,
        [0xa2, 0x06, 0x99, 0xac, 0x5f, 0xcc, 0x52, 0x65]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIEditingSession {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIEditingSession.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIEditingSessionCoerce {
    /// Cheaply cast a value of this type from a `nsIEditingSession`.
    fn coerce_from(v: &nsIEditingSession) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIEditingSessionCoerce for nsIEditingSession {
    #[inline]
    fn coerce_from(v: &nsIEditingSession) -> &Self {
        v
    }
}

impl nsIEditingSession {
    /// Cast this `nsIEditingSession` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIEditingSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIEditingSession {
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
impl<T: nsISupportsCoerce> nsIEditingSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIEditingSession) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIEditingSession
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIEditingSessionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned long editorStatus; */
    pub GetEditorStatus: unsafe extern "system" fn (this: *const nsIEditingSession, aEditorStatus: *mut u32) -> ::nserror::nsresult,

    /* [can_run_script] void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive); */
    pub MakeWindowEditable: unsafe extern "system" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy, aEditorType: *const libc::c_char, doAfterUriLoad: bool, aMakeWholeDocumentEditable: bool, aInteractive: bool) -> ::nserror::nsresult,

    /* boolean windowIsEditable (in mozIDOMWindowProxy window); */
    pub WindowIsEditable: unsafe extern "system" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsIEditor getEditorForWindow (in mozIDOMWindowProxy window); */
    pub GetEditorForWindow: unsafe extern "system" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy, _retval: *mut*const nsIEditor) -> ::nserror::nsresult,

    /* [noscript] void tearDownEditorOnWindow (in mozIDOMWindowProxy window); */
    pub TearDownEditorOnWindow: unsafe extern "system" fn (this: *const nsIEditingSession, window: *const mozIDOMWindowProxy) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIEditingSession {
    /// ```text
    /// /**
    ///    *  Error codes when we fail to create an editor
    ///    *  is placed in attribute editorStatus
    ///    */
    /// ```
    ///

    pub const eEditorOK: i64 = 0;


    pub const eEditorCreationInProgress: i64 = 1;


    pub const eEditorErrorCantEditMimeType: i64 = 2;


    pub const eEditorErrorFileNotFound: i64 = 3;


    pub const eEditorErrorCantEditFramesets: i64 = 8;


    pub const eEditorErrorUnknown: i64 = 9;

    /// ```text
    /// /**
    ///    *  Status after editor creation and document loading
    ///    *  Value is one of the above error codes
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long editorStatus;`
    #[inline]
    pub unsafe fn GetEditorStatus(&self, aEditorStatus: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEditorStatus)(self, aEditorStatus)
    }


    /// ```text
    /// /**
    ///    *  Make this window editable
    ///    *  @param aWindow nsIDOMWindow, the window the embedder needs to make editable
    ///    *  @param aEditorType string, "html" "htmlsimple" "text" "textsimple"
    ///    *  @param aMakeWholeDocumentEditable if PR_TRUE make the whole document in
    ///    *                                    aWindow editable, otherwise it's the
    ///    *                                    embedder who should make the document
    ///    *                                    (or part of it) editable.
    ///    *  @param aInteractive if PR_FALSE turn off scripting and plugins
    ///    */
    /// ```
    ///

    /// `[can_run_script] void makeWindowEditable (in mozIDOMWindowProxy window, in string aEditorType, in boolean doAfterUriLoad, in boolean aMakeWholeDocumentEditable, in boolean aInteractive);`
    #[inline]
    pub unsafe fn MakeWindowEditable(&self, window: *const mozIDOMWindowProxy, aEditorType: *const libc::c_char, doAfterUriLoad: bool, aMakeWholeDocumentEditable: bool, aInteractive: bool) -> ::nserror::nsresult {
        ((*self.vtable).MakeWindowEditable)(self, window, aEditorType, doAfterUriLoad, aMakeWholeDocumentEditable, aInteractive)
    }


    /// ```text
    /// /**
    ///    *  Test whether a specific window has had its editable flag set; it may have an editor
    ///    *  now, or will get one after the uri load.
    ///    *
    ///    *  Use this, passing the content root window, to test if we've set up editing
    ///    *  for this content.
    ///    */
    /// ```
    ///

    /// `boolean windowIsEditable (in mozIDOMWindowProxy window);`
    #[inline]
    pub unsafe fn WindowIsEditable(&self, window: *const mozIDOMWindowProxy, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WindowIsEditable)(self, window, _retval)
    }


    /// ```text
    /// /**
    ///    *  Get the editor for this window. May return null
    ///    */
    /// ```
    ///

    /// `nsIEditor getEditorForWindow (in mozIDOMWindowProxy window);`
    #[inline]
    pub unsafe fn GetEditorForWindow(&self, window: *const mozIDOMWindowProxy, _retval: *mut*const nsIEditor) -> ::nserror::nsresult {
        ((*self.vtable).GetEditorForWindow)(self, window, _retval)
    }


    /// ```text
    /// /**
    ///    *   Destroy editor and related support objects
    ///    */
    /// ```
    ///

    /// `[noscript] void tearDownEditorOnWindow (in mozIDOMWindowProxy window);`
    #[inline]
    pub unsafe fn TearDownEditorOnWindow(&self, window: *const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).TearDownEditorOnWindow)(self, window)
    }


}


