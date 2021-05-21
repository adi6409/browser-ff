//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIContentViewer.idl
//


/// `interface nsIContentViewer : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIContentViewer {
    vtable: *const nsIContentViewerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIContentViewer.
unsafe impl XpCom for nsIContentViewer {
    const IID: nsIID = nsID(0x2da17016, 0x7851, 0x4a45,
        [0xa7, 0xa8, 0x00, 0xb3, 0x60, 0xe0, 0x15, 0x95]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIContentViewer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIContentViewer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIContentViewerCoerce {
    /// Cheaply cast a value of this type from a `nsIContentViewer`.
    fn coerce_from(v: &nsIContentViewer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIContentViewerCoerce for nsIContentViewer {
    #[inline]
    fn coerce_from(v: &nsIContentViewer) -> &Self {
        v
    }
}

impl nsIContentViewer {
    /// Cast this `nsIContentViewer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIContentViewerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIContentViewer {
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
impl<T: nsISupportsCoerce> nsIContentViewerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIContentViewer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIContentViewer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIContentViewerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [noscript] void init (in nsIWidgetPtr aParentWidget, [const] in nsIntRectRef aBounds, in WindowGlobalChildPtr aWindowActor); */
    /// Unable to generate binding because `native type nsIWidget unsupported`
    pub Init: *const ::libc::c_void,

    /* attribute nsIDocShell container; */
    pub GetContainer: unsafe extern "system" fn (this: *const nsIContentViewer, aContainer: *mut*const nsIDocShell) -> ::nserror::nsresult,

    /* attribute nsIDocShell container; */
    pub SetContainer: unsafe extern "system" fn (this: *const nsIContentViewer, aContainer: *const nsIDocShell) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] void loadStart (in Document aDoc); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub LoadStart: *const ::libc::c_void,

    /* [can_run_script] void loadComplete (in nsresult aStatus); */
    pub LoadComplete: unsafe extern "system" fn (this: *const nsIContentViewer, aStatus: ::nserror::nsresult) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute boolean loadCompleted; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetLoadCompleted: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute boolean isStopped; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetIsStopped: *const ::libc::c_void,

    /* boolean permitUnload ([optional] in nsIContentViewer_PermitUnloadAction aAction); */
    pub PermitUnload: unsafe extern "system" fn (this: *const nsIContentViewer, aAction:  u8, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute boolean inPermitUnload; */
    pub GetInPermitUnload: unsafe extern "system" fn (this: *const nsIContentViewer, aInPermitUnload: *mut bool) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] nsIContentViewer_PermitUnloadResult dispatchBeforeUnload (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub DispatchBeforeUnload: *const ::libc::c_void,

    /* readonly attribute boolean beforeUnloadFiring; */
    pub GetBeforeUnloadFiring: unsafe extern "system" fn (this: *const nsIContentViewer, aBeforeUnloadFiring: *mut bool) -> ::nserror::nsresult,

    /* void pageHide (in boolean isUnload); */
    pub PageHide: unsafe extern "system" fn (this: *const nsIContentViewer, isUnload: bool) -> ::nserror::nsresult,

    /* void close (in nsISHEntry historyEntry); */
    pub Close: unsafe extern "system" fn (this: *const nsIContentViewer, historyEntry: *const nsISHEntry) -> ::nserror::nsresult,

    /* void destroy (); */
    pub Destroy: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* void stop (); */
    pub Stop: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* readonly attribute Document DOMDocument; */
    pub GetDOMDocument: unsafe extern "system" fn (this: *const nsIContentViewer, aDOMDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] Document getDocument (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetDocument: *const ::libc::c_void,

    /* [noscript,nostdcall] void setDocument (in Document aDocument); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetDocument: *const ::libc::c_void,

    /* [noscript] void getBounds (in nsIntRectRef aBounds); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub GetBounds: *const ::libc::c_void,

    /* [noscript] void setBounds ([const] in nsIntRectRef aBounds); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub SetBounds: *const ::libc::c_void,

    /* [noscript] void setBoundsWithFlags ([const] in nsIntRectRef aBounds, in unsigned long aFlags); */
    /// Unable to generate binding because `native type nsIntRect unsupported`
    pub SetBoundsWithFlags: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsIContentViewer previousViewer; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPreviousViewer: *const ::libc::c_void,

    /* [nostdcall,notxpcom] attribute nsIContentViewer previousViewer; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetPreviousViewer: *const ::libc::c_void,

    /* void move (in long aX, in long aY); */
    pub Move: unsafe extern "system" fn (this: *const nsIContentViewer, aX: i32, aY: i32) -> ::nserror::nsresult,

    /* void show (); */
    pub Show: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* void hide (); */
    pub Hide: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* attribute boolean sticky; */
    pub GetSticky: unsafe extern "system" fn (this: *const nsIContentViewer, aSticky: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean sticky; */
    pub SetSticky: unsafe extern "system" fn (this: *const nsIContentViewer, aSticky: bool) -> ::nserror::nsresult,

    /* void open (in nsISupports aState, in nsISHEntry aSHEntry); */
    pub Open: unsafe extern "system" fn (this: *const nsIContentViewer, aState: *const nsISupports, aSHEntry: *const nsISHEntry) -> ::nserror::nsresult,

    /* void clearHistoryEntry (); */
    pub ClearHistoryEntry: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* void setPageModeForTesting (in boolean aPageMode, in nsIPrintSettings aPrintSettings); */
    pub SetPageModeForTesting: unsafe extern "system" fn (this: *const nsIContentViewer, aPageMode: bool, aPrintSettings: *const nsIPrintSettings) -> ::nserror::nsresult,

    /* readonly attribute nsISHEntry historyEntry; */
    pub GetHistoryEntry: unsafe extern "system" fn (this: *const nsIContentViewer, aHistoryEntry: *mut*const nsISHEntry) -> ::nserror::nsresult,

    /* readonly attribute boolean isTabModalPromptAllowed; */
    pub GetIsTabModalPromptAllowed: unsafe extern "system" fn (this: *const nsIContentViewer, aIsTabModalPromptAllowed: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isHidden; */
    pub GetIsHidden: unsafe extern "system" fn (this: *const nsIContentViewer, aIsHidden: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean isHidden; */
    pub SetIsHidden: unsafe extern "system" fn (this: *const nsIContentViewer, aIsHidden: bool) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] readonly attribute PresShellPtr presShell; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPresShell: *const ::libc::c_void,

    /* [nostdcall,notxpcom] readonly attribute nsPresContextPtr presContext; */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetPresContext: *const ::libc::c_void,

    /* [noscript] void setDocumentInternal (in Document aDocument, in boolean aForceReuseInnerWindow); */
    pub SetDocumentInternal: unsafe extern "system" fn (this: *const nsIContentViewer, aDocument: *const libc::c_void, aForceReuseInnerWindow: bool) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] nsViewPtr findContainerView (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub FindContainerView: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] void setNavigationTiming (in nsDOMNavigationTimingPtr aTiming); */
    /// Unable to generate binding because `native type nsDOMNavigationTiming unsupported`
    pub SetNavigationTiming: *const ::libc::c_void,

    /* readonly attribute float deviceFullZoomForTest; */
    pub GetDeviceFullZoomForTest: unsafe extern "system" fn (this: *const nsIContentViewer, aDeviceFullZoomForTest: *mut libc::c_float) -> ::nserror::nsresult,

    /* attribute boolean authorStyleDisabled; */
    pub GetAuthorStyleDisabled: unsafe extern "system" fn (this: *const nsIContentViewer, aAuthorStyleDisabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean authorStyleDisabled; */
    pub SetAuthorStyleDisabled: unsafe extern "system" fn (this: *const nsIContentViewer, aAuthorStyleDisabled: bool) -> ::nserror::nsresult,

    /* attribute ACString hintCharacterSet; */
    pub GetHintCharacterSet: unsafe extern "system" fn (this: *const nsIContentViewer, aHintCharacterSet: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute ACString hintCharacterSet; */
    pub SetHintCharacterSet: unsafe extern "system" fn (this: *const nsIContentViewer, aHintCharacterSet: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute int32_t hintCharacterSetSource; */
    pub GetHintCharacterSetSource: unsafe extern "system" fn (this: *const nsIContentViewer, aHintCharacterSetSource: *mut int32_t) -> ::nserror::nsresult,

    /* attribute int32_t hintCharacterSetSource; */
    pub SetHintCharacterSetSource: unsafe extern "system" fn (this: *const nsIContentViewer, aHintCharacterSetSource: int32_t) -> ::nserror::nsresult,

    /* void getContentSize (out long width, out long height); */
    pub GetContentSize: unsafe extern "system" fn (this: *const nsIContentViewer, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void getContentSizeConstrained (in long maxWidth, in long maxHeight, out long width, out long height); */
    pub GetContentSizeConstrained: unsafe extern "system" fn (this: *const nsIContentViewer, maxWidth: i32, maxHeight: i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void pausePainting (); */
    pub PausePainting: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* void resumePainting (); */
    pub ResumePainting: unsafe extern "system" fn (this: *const nsIContentViewer) -> ::nserror::nsresult,

    /* [noscript,notxpcom] Encoding getHintCharset (); */
    /// Unable to generate binding because `native type const mozilla::Encoding unsupported`
    pub GetHintCharset: *const ::libc::c_void,

    /* [noscript,notxpcom] void setHintCharset (in Encoding aEncoding); */
    /// Unable to generate binding because `native type const mozilla::Encoding unsupported`
    pub SetHintCharset: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIContentViewer {
    /// ```text
    /// /**
    ///    * The 'aFlags' argument to setBoundsWithFlags is a set of these bits.
    ///    */
    /// ```
    ///

    pub const eDelayResize: i64 = 1;


    /// `[noscript] void init (in nsIWidgetPtr aParentWidget, [const] in nsIntRectRef aBounds, in WindowGlobalChildPtr aWindowActor);`
    const _Init: () = ();


    /// `attribute nsIDocShell container;`
    #[inline]
    pub unsafe fn GetContainer(&self, aContainer: *mut*const nsIDocShell) -> ::nserror::nsresult {
        ((*self.vtable).GetContainer)(self, aContainer)
    }



    /// `attribute nsIDocShell container;`
    #[inline]
    pub unsafe fn SetContainer(&self, aContainer: *const nsIDocShell) -> ::nserror::nsresult {
        ((*self.vtable).SetContainer)(self, aContainer)
    }



    /// `[noscript,nostdcall,notxpcom] void loadStart (in Document aDoc);`
    const _LoadStart: () = ();


    /// `[can_run_script] void loadComplete (in nsresult aStatus);`
    #[inline]
    pub unsafe fn LoadComplete(&self, aStatus: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).LoadComplete)(self, aStatus)
    }



    /// `[nostdcall,notxpcom] readonly attribute boolean loadCompleted;`
    const _GetLoadCompleted: () = ();


    /// `[nostdcall,notxpcom] readonly attribute boolean isStopped;`
    const _GetIsStopped: () = ();

    /// ```text
    /// /**
    ///    * Overload PermitUnload method for C++ consumers with no aPermitUnloadFlags
    ///    * argument.
    ///    */
    /// /**
    ///    * Checks if the document wants to prevent unloading by firing beforeunload on
    ///    * the document.
    ///    * The result is returned.
    ///    */
    /// ```
    ///

    /// `boolean permitUnload ([optional] in nsIContentViewer_PermitUnloadAction aAction);`
    #[inline]
    pub unsafe fn PermitUnload(&self, aAction:  u8, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).PermitUnload)(self, aAction, _retval)
    }


    /// ```text
    /// /**
    ///    * Exposes whether we're blocked in a call to permitUnload.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean inPermitUnload;`
    #[inline]
    pub unsafe fn GetInPermitUnload(&self, aInPermitUnload: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetInPermitUnload)(self, aInPermitUnload)
    }


    /// ```text
    /// /**
    ///    * Dispatches the "beforeunload" event and returns the result, as documented
    ///    * in the `PermitUnloadResult` enum.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] nsIContentViewer_PermitUnloadResult dispatchBeforeUnload ();`
    const _DispatchBeforeUnload: () = ();

    /// ```text
    /// /**
    ///    * Exposes whether we're in the process of firing the beforeunload event.
    ///    * In this case, the corresponding docshell will not allow navigation.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean beforeUnloadFiring;`
    #[inline]
    pub unsafe fn GetBeforeUnloadFiring(&self, aBeforeUnloadFiring: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetBeforeUnloadFiring)(self, aBeforeUnloadFiring)
    }



    /// `void pageHide (in boolean isUnload);`
    #[inline]
    pub unsafe fn PageHide(&self, isUnload: bool) -> ::nserror::nsresult {
        ((*self.vtable).PageHide)(self, isUnload)
    }


    /// ```text
    /// /**
    ///    * All users of a content viewer are responsible for calling both
    ///    * close() and destroy(), in that order.
    ///    *
    ///    * close() should be called when the load of a new page for the next
    ///    * content viewer begins, and destroy() should be called when the next
    ///    * content viewer replaces this one.
    ///    *
    ///    * |historyEntry| sets the session history entry for the content viewer.  If
    ///    * this is null, then Destroy() will be called on the document by close().
    ///    * If it is non-null, the document will not be destroyed, and the following
    ///    * actions will happen when destroy() is called (*):
    ///    *  - Sanitize() will be called on the viewer's document
    ///    *  - The content viewer will set the contentViewer property on the
    ///    *    history entry, and release its reference (ownership reversal).
    ///    *  - hide() will be called, and no further destruction will happen.
    ///    *
    ///    *  (*) unless the document is currently being printed, in which case
    ///    *      it will never be saved in session history.
    ///    *
    ///    */
    /// ```
    ///

    /// `void close (in nsISHEntry historyEntry);`
    #[inline]
    pub unsafe fn Close(&self, historyEntry: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).Close)(self, historyEntry)
    }



    /// `void destroy ();`
    #[inline]
    pub unsafe fn Destroy(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Destroy)(self, )
    }



    /// `void stop ();`
    #[inline]
    pub unsafe fn Stop(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Stop)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns the same thing as getDocument(), but for use from script
    ///    * only.  C++ consumers should use getDocument().
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
    ///    * Returns DOMDocument without addrefing.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] Document getDocument ();`
    const _GetDocument: () = ();

    /// ```text
    /// /**
    ///    * Allows setting the document.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall] void setDocument (in Document aDocument);`
    const _SetDocument: () = ();


    /// `[noscript] void getBounds (in nsIntRectRef aBounds);`
    const _GetBounds: () = ();


    /// `[noscript] void setBounds ([const] in nsIntRectRef aBounds);`
    const _SetBounds: () = ();


    /// `[noscript] void setBoundsWithFlags ([const] in nsIntRectRef aBounds, in unsigned long aFlags);`
    const _SetBoundsWithFlags: () = ();

    /// ```text
    /// /**
    ///    * The previous content viewer, which has been |close|d but not
    ///    * |destroy|ed.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute nsIContentViewer previousViewer;`
    const _GetPreviousViewer: () = ();

    /// ```text
    /// /**
    ///    * The previous content viewer, which has been |close|d but not
    ///    * |destroy|ed.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] attribute nsIContentViewer previousViewer;`
    const _SetPreviousViewer: () = ();


    /// `void move (in long aX, in long aY);`
    #[inline]
    pub unsafe fn Move(&self, aX: i32, aY: i32) -> ::nserror::nsresult {
        ((*self.vtable).Move)(self, aX, aY)
    }



    /// `void show ();`
    #[inline]
    pub unsafe fn Show(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Show)(self, )
    }



    /// `void hide ();`
    #[inline]
    pub unsafe fn Hide(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Hide)(self, )
    }



    /// `attribute boolean sticky;`
    #[inline]
    pub unsafe fn GetSticky(&self, aSticky: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSticky)(self, aSticky)
    }



    /// `attribute boolean sticky;`
    #[inline]
    pub unsafe fn SetSticky(&self, aSticky: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSticky)(self, aSticky)
    }


    /// ```text
    /// /**
    ///    * Attach the content viewer to its DOM window and docshell.
    ///    * @param aState A state object that might be useful in attaching the DOM
    ///    *               window.
    ///    * @param aSHEntry The history entry that the content viewer was stored in.
    ///    *                 The entry must have the docshells for all of the child
    ///    *                 documents stored in its child shell list.
    ///    */
    /// ```
    ///

    /// `void open (in nsISupports aState, in nsISHEntry aSHEntry);`
    #[inline]
    pub unsafe fn Open(&self, aState: *const nsISupports, aSHEntry: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).Open)(self, aState, aSHEntry)
    }


    /// ```text
    /// /**
    ///    * Clears the current history entry.  This is used if we need to clear out
    ///    * the saved presentation state.
    ///    */
    /// ```
    ///

    /// `void clearHistoryEntry ();`
    #[inline]
    pub unsafe fn ClearHistoryEntry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearHistoryEntry)(self, )
    }


    /// ```text
    /// /**
    ///    * Change the layout to view the document with page layout (like print preview), but
    ///    * dynamic and editable (like Galley layout).
    ///    */
    /// ```
    ///

    /// `void setPageModeForTesting (in boolean aPageMode, in nsIPrintSettings aPrintSettings);`
    #[inline]
    pub unsafe fn SetPageModeForTesting(&self, aPageMode: bool, aPrintSettings: *const nsIPrintSettings) -> ::nserror::nsresult {
        ((*self.vtable).SetPageModeForTesting)(self, aPageMode, aPrintSettings)
    }


    /// ```text
    /// /**
    ///    * Get the history entry that this viewer will save itself into when
    ///    * destroyed.  Can return null
    ///    */
    /// ```
    ///

    /// `readonly attribute nsISHEntry historyEntry;`
    #[inline]
    pub unsafe fn GetHistoryEntry(&self, aHistoryEntry: *mut*const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetHistoryEntry)(self, aHistoryEntry)
    }


    /// ```text
    /// /**
    ///    * Indicates when we're in a state where content shouldn't be allowed to
    ///    * trigger a tab-modal prompt (as opposed to a window-modal prompt) because
    ///    * we're part way through some operation (eg beforeunload) that shouldn't be
    ///    * rentrant if the user closes the tab while the prompt is showing.
    ///    * See bug 613800.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isTabModalPromptAllowed;`
    #[inline]
    pub unsafe fn GetIsTabModalPromptAllowed(&self, aIsTabModalPromptAllowed: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsTabModalPromptAllowed)(self, aIsTabModalPromptAllowed)
    }


    /// ```text
    /// /**
    ///    * Returns whether this content viewer is in a hidden state.
    ///    *
    ///    * @note Only Gecko internal code should set the attribute!
    ///    */
    /// ```
    ///

    /// `attribute boolean isHidden;`
    #[inline]
    pub unsafe fn GetIsHidden(&self, aIsHidden: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsHidden)(self, aIsHidden)
    }


    /// ```text
    /// /**
    ///    * Returns whether this content viewer is in a hidden state.
    ///    *
    ///    * @note Only Gecko internal code should set the attribute!
    ///    */
    /// ```
    ///

    /// `attribute boolean isHidden;`
    #[inline]
    pub unsafe fn SetIsHidden(&self, aIsHidden: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIsHidden)(self, aIsHidden)
    }



    /// `[nostdcall,notxpcom] readonly attribute PresShellPtr presShell;`
    const _GetPresShell: () = ();


    /// `[nostdcall,notxpcom] readonly attribute nsPresContextPtr presContext;`
    const _GetPresContext: () = ();


    /// `[noscript] void setDocumentInternal (in Document aDocument, in boolean aForceReuseInnerWindow);`
    #[inline]
    pub unsafe fn SetDocumentInternal(&self, aDocument: *const libc::c_void, aForceReuseInnerWindow: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDocumentInternal)(self, aDocument, aForceReuseInnerWindow)
    }


    /// ```text
    /// /**
    ///    * Find the view to use as the container view for MakeWindow. Returns
    ///    * null if this will be the root of a view manager hierarchy. In that
    ///    * case, if mParentWidget is null then this document should not even
    ///    * be displayed.
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] nsViewPtr findContainerView ();`
    const _FindContainerView: () = ();

    /// ```text
    /// /**
    ///    * Set collector for navigation timing data (load, unload events).
    ///    */
    /// ```
    ///

    /// `[noscript,nostdcall,notxpcom] void setNavigationTiming (in nsDOMNavigationTimingPtr aTiming);`
    const _SetNavigationTiming: () = ();

    /// ```text
    /// /**
    ///    * The actual full zoom in effect, as modified by the device context.
    ///    * For a requested full zoom, the device context may choose a slightly
    ///    * different effectiveFullZoom to accomodate integer rounding of app units
    ///    * per dev pixel. This property returns the actual zoom amount in use,
    ///    * though it may not be good user experience to report that a requested zoom
    ///    * of 90% is actually 89.1%, for example. This value is provided primarily to
    ///    * support media queries of dppx values, because those queries are matched
    ///    * against the actual native device pixel ratio and the actual full zoom.
    ///    *
    ///    * You should only need this for testing.
    ///    */
    /// ```
    ///

    /// `readonly attribute float deviceFullZoomForTest;`
    #[inline]
    pub unsafe fn GetDeviceFullZoomForTest(&self, aDeviceFullZoomForTest: *mut libc::c_float) -> ::nserror::nsresult {
        ((*self.vtable).GetDeviceFullZoomForTest)(self, aDeviceFullZoomForTest)
    }


    /// ```text
    /// /**
    ///    * Disable entire author style level (including HTML presentation hints),
    ///    * for this viewer but not any child viewers.
    ///    */
    /// ```
    ///

    /// `attribute boolean authorStyleDisabled;`
    #[inline]
    pub unsafe fn GetAuthorStyleDisabled(&self, aAuthorStyleDisabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAuthorStyleDisabled)(self, aAuthorStyleDisabled)
    }


    /// ```text
    /// /**
    ///    * Disable entire author style level (including HTML presentation hints),
    ///    * for this viewer but not any child viewers.
    ///    */
    /// ```
    ///

    /// `attribute boolean authorStyleDisabled;`
    #[inline]
    pub unsafe fn SetAuthorStyleDisabled(&self, aAuthorStyleDisabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAuthorStyleDisabled)(self, aAuthorStyleDisabled)
    }


    /// ```text
    /// /**
    ///    * XXX comm-central only: bug 829543.
    ///    */
    /// ```
    ///

    /// `attribute ACString hintCharacterSet;`
    #[inline]
    pub unsafe fn GetHintCharacterSet(&self, aHintCharacterSet: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetHintCharacterSet)(self, aHintCharacterSet)
    }


    /// ```text
    /// /**
    ///    * XXX comm-central only: bug 829543.
    ///    */
    /// ```
    ///

    /// `attribute ACString hintCharacterSet;`
    #[inline]
    pub unsafe fn SetHintCharacterSet(&self, aHintCharacterSet: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetHintCharacterSet)(self, aHintCharacterSet)
    }


    /// ```text
    /// /**
    ///    * XXX comm-central only: bug 829543.
    ///    */
    /// ```
    ///

    /// `attribute int32_t hintCharacterSetSource;`
    #[inline]
    pub unsafe fn GetHintCharacterSetSource(&self, aHintCharacterSetSource: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetHintCharacterSetSource)(self, aHintCharacterSetSource)
    }


    /// ```text
    /// /**
    ///    * XXX comm-central only: bug 829543.
    ///    */
    /// ```
    ///

    /// `attribute int32_t hintCharacterSetSource;`
    #[inline]
    pub unsafe fn SetHintCharacterSetSource(&self, aHintCharacterSetSource: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetHintCharacterSetSource)(self, aHintCharacterSetSource)
    }


    /// ```text
    /// /**
    ///    * Requests the size of the content to the container.
    ///    */
    /// ```
    ///

    /// `void getContentSize (out long width, out long height);`
    #[inline]
    pub unsafe fn GetContentSize(&self, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetContentSize)(self, width, height)
    }


    /// ```text
    /// /**
    ///    * Returns the preferred width and height of the content, constrained to the
    ///    * given maximum values. If either maxWidth or maxHeight is less than zero,
    ///    * that dimension is not constrained.
    ///    *
    ///    * All input and output values are in device pixels, rather than CSS pixels.
    ///    */
    /// ```
    ///

    /// `void getContentSizeConstrained (in long maxWidth, in long maxHeight, out long width, out long height);`
    #[inline]
    pub unsafe fn GetContentSizeConstrained(&self, maxWidth: i32, maxHeight: i32, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetContentSizeConstrained)(self, maxWidth, maxHeight, width, height)
    }


    /// ```text
    /// /**
    ///    * Instruct the refresh driver to discontinue painting until further
    ///    * notice.
    ///    */
    /// ```
    ///

    /// `void pausePainting ();`
    #[inline]
    pub unsafe fn PausePainting(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).PausePainting)(self, )
    }


    /// ```text
    /// /**
    ///    * Instruct the refresh driver to resume painting after a previous call to
    ///    * pausePainting().
    ///    */
    /// ```
    ///

    /// `void resumePainting ();`
    #[inline]
    pub unsafe fn ResumePainting(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ResumePainting)(self, )
    }



    /// `[noscript,notxpcom] Encoding getHintCharset ();`
    const _GetHintCharset: () = ();


    /// `[noscript,notxpcom] void setHintCharset (in Encoding aEncoding);`
    const _SetHintCharset: () = ();

}


