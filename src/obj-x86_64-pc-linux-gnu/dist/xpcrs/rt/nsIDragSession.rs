//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIDragSession.idl
//


/// `interface nsIDragSession : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDragSession {
    vtable: *const nsIDragSessionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDragSession.
unsafe impl XpCom for nsIDragSession {
    const IID: nsIID = nsID(0x25bce737, 0x73f0, 0x43c7,
        [0xbc, 0x20, 0xc7, 0x10, 0x44, 0xa7, 0x3c, 0x5a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDragSession {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDragSession.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDragSessionCoerce {
    /// Cheaply cast a value of this type from a `nsIDragSession`.
    fn coerce_from(v: &nsIDragSession) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDragSessionCoerce for nsIDragSession {
    #[inline]
    fn coerce_from(v: &nsIDragSession) -> &Self {
        v
    }
}

impl nsIDragSession {
    /// Cast this `nsIDragSession` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDragSessionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDragSession {
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
impl<T: nsISupportsCoerce> nsIDragSessionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDragSession) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDragSession
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDragSessionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean canDrop; */
    pub GetCanDrop: unsafe extern "system" fn (this: *const nsIDragSession, aCanDrop: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean canDrop; */
    pub SetCanDrop: unsafe extern "system" fn (this: *const nsIDragSession, aCanDrop: bool) -> ::nserror::nsresult,

    /* attribute boolean onlyChromeDrop; */
    pub GetOnlyChromeDrop: unsafe extern "system" fn (this: *const nsIDragSession, aOnlyChromeDrop: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean onlyChromeDrop; */
    pub SetOnlyChromeDrop: unsafe extern "system" fn (this: *const nsIDragSession, aOnlyChromeDrop: bool) -> ::nserror::nsresult,

    /* attribute unsigned long dragAction; */
    pub GetDragAction: unsafe extern "system" fn (this: *const nsIDragSession, aDragAction: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long dragAction; */
    pub SetDragAction: unsafe extern "system" fn (this: *const nsIDragSession, aDragAction: u32) -> ::nserror::nsresult,

    /* readonly attribute unsigned long numDropItems; */
    pub GetNumDropItems: unsafe extern "system" fn (this: *const nsIDragSession, aNumDropItems: *mut u32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute Document sourceDocument; */
    pub GetSourceDocument: unsafe extern "system" fn (this: *const nsIDragSession, aSourceDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute Node sourceNode; */
    pub GetSourceNode: unsafe extern "system" fn (this: *const nsIDragSession, aSourceNode: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute nsIPrincipal triggeringPrincipal; */
    pub GetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsIDragSession, aTriggeringPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult,

    /* attribute nsIPrincipal triggeringPrincipal; */
    pub SetTriggeringPrincipal: unsafe extern "system" fn (this: *const nsIDragSession, aTriggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* attribute nsIContentSecurityPolicy csp; */
    pub GetCsp: unsafe extern "system" fn (this: *const nsIDragSession, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* attribute nsIContentSecurityPolicy csp; */
    pub SetCsp: unsafe extern "system" fn (this: *const nsIDragSession, aCsp: *const nsIContentSecurityPolicy) -> ::nserror::nsresult,

    /* [binaryname(DataTransferXPCOM)] attribute DataTransfer dataTransfer; */
    pub GetDataTransferXPCOM: unsafe extern "system" fn (this: *const nsIDragSession, aDataTransfer: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [binaryname(DataTransferXPCOM)] attribute DataTransfer dataTransfer; */
    pub SetDataTransferXPCOM: unsafe extern "system" fn (this: *const nsIDragSession, aDataTransfer: *const libc::c_void) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] DataTransfer getDataTransfer (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetDataTransfer: *const ::libc::c_void,

    /* [nostdcall,notxpcom] void setDataTransfer (in DataTransfer aDataTransfer); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub SetDataTransfer: *const ::libc::c_void,

    /* void getData (in nsITransferable aTransferable, in unsigned long aItemIndex); */
    pub GetData: unsafe extern "system" fn (this: *const nsIDragSession, aTransferable: *const nsITransferable, aItemIndex: u32) -> ::nserror::nsresult,

    /* boolean isDataFlavorSupported (in string aDataFlavor); */
    pub IsDataFlavorSupported: unsafe extern "system" fn (this: *const nsIDragSession, aDataFlavor: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult,

    /* void userCancelled (); */
    pub UserCancelled: unsafe extern "system" fn (this: *const nsIDragSession) -> ::nserror::nsresult,

    /* void dragEventDispatchedToChildProcess (); */
    pub DragEventDispatchedToChildProcess: unsafe extern "system" fn (this: *const nsIDragSession) -> ::nserror::nsresult,

    /* void updateDragEffect (); */
    pub UpdateDragEffect: unsafe extern "system" fn (this: *const nsIDragSession) -> ::nserror::nsresult,

    /* void updateDragImage (in Node aImage, in long aImageX, in long aImageY); */
    pub UpdateDragImage: unsafe extern "system" fn (this: *const nsIDragSession, aImage: *const libc::c_void, aImageX: i32, aImageY: i32) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] unsigned long getEffectAllowedForTests (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetEffectAllowedForTests: *const ::libc::c_void,

    /* [nostdcall,notxpcom] bool isSynthesizedForTests (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub IsSynthesizedForTests: *const ::libc::c_void,

    /* void setDragEndPointForTests (in long aScreenX, in long aScreenY); */
    pub SetDragEndPointForTests: unsafe extern "system" fn (this: *const nsIDragSession, aScreenX: i32, aScreenY: i32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDragSession {

    /// ```text
    /// /**
    ///     * Set the current state of the drag, whether it can be dropped or not.
    ///     * usually the target "frame" sets this so the native system can render the correct feedback
    ///     */
    /// ```
    ///

    /// `attribute boolean canDrop;`
    #[inline]
    pub unsafe fn GetCanDrop(&self, aCanDrop: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetCanDrop)(self, aCanDrop)
    }


    /// ```text
    /// /**
    ///     * Set the current state of the drag, whether it can be dropped or not.
    ///     * usually the target "frame" sets this so the native system can render the correct feedback
    ///     */
    /// ```
    ///

    /// `attribute boolean canDrop;`
    #[inline]
    pub unsafe fn SetCanDrop(&self, aCanDrop: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetCanDrop)(self, aCanDrop)
    }


    /// ```text
    /// /**
    ///    * Indicates if the drop event should be dispatched only to chrome.
    ///    */
    /// ```
    ///

    /// `attribute boolean onlyChromeDrop;`
    #[inline]
    pub unsafe fn GetOnlyChromeDrop(&self, aOnlyChromeDrop: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOnlyChromeDrop)(self, aOnlyChromeDrop)
    }


    /// ```text
    /// /**
    ///    * Indicates if the drop event should be dispatched only to chrome.
    ///    */
    /// ```
    ///

    /// `attribute boolean onlyChromeDrop;`
    #[inline]
    pub unsafe fn SetOnlyChromeDrop(&self, aOnlyChromeDrop: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetOnlyChromeDrop)(self, aOnlyChromeDrop)
    }


    /// ```text
    /// /**
    ///     * Sets the action (copy, move, link, et.c) for the current drag
    ///     */
    /// ```
    ///

    /// `attribute unsigned long dragAction;`
    #[inline]
    pub unsafe fn GetDragAction(&self, aDragAction: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetDragAction)(self, aDragAction)
    }


    /// ```text
    /// /**
    ///     * Sets the action (copy, move, link, et.c) for the current drag
    ///     */
    /// ```
    ///

    /// `attribute unsigned long dragAction;`
    #[inline]
    pub unsafe fn SetDragAction(&self, aDragAction: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetDragAction)(self, aDragAction)
    }


    /// ```text
    /// /**
    ///     * Get the number of items that were dropped
    ///     */
    /// ```
    ///

    /// `readonly attribute unsigned long numDropItems;`
    #[inline]
    pub unsafe fn GetNumDropItems(&self, aNumDropItems: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumDropItems)(self, aNumDropItems)
    }


    /// ```text
    /// /**
    ///     * The document where the drag was started, which will be null if the
    ///     * drag originated outside the application. Useful for determining if a drop
    ///     * originated in the same document.
    ///     */
    /// ```
    ///

    /// `[infallible] readonly attribute Document sourceDocument;`
    #[inline]
    pub unsafe fn GetSourceDocument(&self, aSourceDocument: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceDocument)(self, aSourceDocument)
    }


    /// ```text
    /// /**
    ///     * The dom node that was originally dragged to start the session, which will be null if the
    ///     * drag originated outside the application.
    ///     */
    /// ```
    ///

    /// `readonly attribute Node sourceNode;`
    #[inline]
    pub unsafe fn GetSourceNode(&self, aSourceNode: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetSourceNode)(self, aSourceNode)
    }


    /// ```text
    /// /**
    ///    * the triggering principal.  This may be different than sourceNode's
    ///    * principal when sourceNode is xul:browser and the drag is
    ///    * triggered in a browsing context inside it.
    ///    */
    /// ```
    ///

    /// `attribute nsIPrincipal triggeringPrincipal;`
    #[inline]
    pub unsafe fn GetTriggeringPrincipal(&self, aTriggeringPrincipal: *mut*const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).GetTriggeringPrincipal)(self, aTriggeringPrincipal)
    }


    /// ```text
    /// /**
    ///    * the triggering principal.  This may be different than sourceNode's
    ///    * principal when sourceNode is xul:browser and the drag is
    ///    * triggered in a browsing context inside it.
    ///    */
    /// ```
    ///

    /// `attribute nsIPrincipal triggeringPrincipal;`
    #[inline]
    pub unsafe fn SetTriggeringPrincipal(&self, aTriggeringPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).SetTriggeringPrincipal)(self, aTriggeringPrincipal)
    }


    /// ```text
    /// /**
    ///    * the triggering csp.  This may be different than sourceNode's
    ///    * csp when sourceNode is xul:browser and the drag is
    ///    * triggered in a browsing context inside it.
    ///    */
    /// ```
    ///

    /// `attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn GetCsp(&self, aCsp: *mut*const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).GetCsp)(self, aCsp)
    }


    /// ```text
    /// /**
    ///    * the triggering csp.  This may be different than sourceNode's
    ///    * csp when sourceNode is xul:browser and the drag is
    ///    * triggered in a browsing context inside it.
    ///    */
    /// ```
    ///

    /// `attribute nsIContentSecurityPolicy csp;`
    #[inline]
    pub unsafe fn SetCsp(&self, aCsp: *const nsIContentSecurityPolicy) -> ::nserror::nsresult {
        ((*self.vtable).SetCsp)(self, aCsp)
    }


    /// ```text
    /// /**
    ///    * The data transfer object for the current drag.
    ///    */
    /// ```
    ///

    /// `[binaryname(DataTransferXPCOM)] attribute DataTransfer dataTransfer;`
    #[inline]
    pub unsafe fn GetDataTransferXPCOM(&self, aDataTransfer: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetDataTransferXPCOM)(self, aDataTransfer)
    }


    /// ```text
    /// /**
    ///    * The data transfer object for the current drag.
    ///    */
    /// ```
    ///

    /// `[binaryname(DataTransferXPCOM)] attribute DataTransfer dataTransfer;`
    #[inline]
    pub unsafe fn SetDataTransferXPCOM(&self, aDataTransfer: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetDataTransferXPCOM)(self, aDataTransfer)
    }



    /// `[nostdcall,notxpcom] DataTransfer getDataTransfer ();`
    const _GetDataTransfer: () = ();


    /// `[nostdcall,notxpcom] void setDataTransfer (in DataTransfer aDataTransfer);`
    const _SetDataTransfer: () = ();

    /// ```text
    /// /**
    ///     * Get data from a Drag&Drop. Can be called while the drag is in process
    ///     * or after the drop has completed.
    ///     *
    ///     * @param  aTransferable the transferable for the data to be put into
    ///     * @param  aItemIndex which of multiple drag items, zero-based
    ///     */
    /// ```
    ///

    /// `void getData (in nsITransferable aTransferable, in unsigned long aItemIndex);`
    #[inline]
    pub unsafe fn GetData(&self, aTransferable: *const nsITransferable, aItemIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).GetData)(self, aTransferable, aItemIndex)
    }


    /// ```text
    /// /**
    ///     * Check to set if any of the native data on the clipboard matches this data flavor
    ///     */
    /// ```
    ///

    /// `boolean isDataFlavorSupported (in string aDataFlavor);`
    #[inline]
    pub unsafe fn IsDataFlavorSupported(&self, aDataFlavor: *const libc::c_char, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).IsDataFlavorSupported)(self, aDataFlavor, _retval)
    }



    /// `void userCancelled ();`
    #[inline]
    pub unsafe fn UserCancelled(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UserCancelled)(self, )
    }



    /// `void dragEventDispatchedToChildProcess ();`
    #[inline]
    pub unsafe fn DragEventDispatchedToChildProcess(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DragEventDispatchedToChildProcess)(self, )
    }



    /// `void updateDragEffect ();`
    #[inline]
    pub unsafe fn UpdateDragEffect(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpdateDragEffect)(self, )
    }



    /// `void updateDragImage (in Node aImage, in long aImageX, in long aImageY);`
    #[inline]
    pub unsafe fn UpdateDragImage(&self, aImage: *const libc::c_void, aImageX: i32, aImageY: i32) -> ::nserror::nsresult {
        ((*self.vtable).UpdateDragImage)(self, aImage, aImageX, aImageY)
    }


    /// ```text
    /// /**
    ///    * Returns effects allowed at starting the session for tests.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] unsigned long getEffectAllowedForTests ();`
    const _GetEffectAllowedForTests: () = ();

    /// ```text
    /// /**
    ///    * Returns true if current session was started with synthesized drag start.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] bool isSynthesizedForTests ();`
    const _IsSynthesizedForTests: () = ();

    /// ```text
    /// /**
    ///    * Sets drag end point of synthesized session when the test does not dispatch
    ///    * "drop" event.
    ///    */
    /// ```
    ///

    /// `void setDragEndPointForTests (in long aScreenX, in long aScreenY);`
    #[inline]
    pub unsafe fn SetDragEndPointForTests(&self, aScreenX: i32, aScreenY: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetDragEndPointForTests)(self, aScreenX, aScreenY)
    }


}


