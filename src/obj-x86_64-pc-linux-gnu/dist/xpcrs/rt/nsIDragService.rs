//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/widget/nsIDragService.idl
//


/// `interface nsIDragService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDragService {
    vtable: *const nsIDragServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDragService.
unsafe impl XpCom for nsIDragService {
    const IID: nsIID = nsID(0xebd6b3a2, 0xaf16, 0x43af,
        [0xa6, 0x98, 0x30, 0x91, 0xa0, 0x87, 0xdd, 0x62]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDragService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDragService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDragServiceCoerce {
    /// Cheaply cast a value of this type from a `nsIDragService`.
    fn coerce_from(v: &nsIDragService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDragServiceCoerce for nsIDragService {
    #[inline]
    fn coerce_from(v: &nsIDragService) -> &Self {
        v
    }
}

impl nsIDragService {
    /// Cast this `nsIDragService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDragServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDragService {
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
impl<T: nsISupportsCoerce> nsIDragServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDragService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDragService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDragServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void invokeDragSession (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferables, in unsigned long aActionType, [optional] in nsContentPolicyType aContentPolicyType); */
    pub InvokeDragSession: unsafe extern "system" fn (this: *const nsIDragService, aDOMNode: *const libc::c_void, aPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, aCookieJarSettings: *const nsICookieJarSettings, aTransferables: *const nsIArray, aActionType: u32, aContentPolicyType: nsContentPolicyType) -> ::nserror::nsresult,

    /* [can_run_script,noscript] void invokeDragSessionWithImage (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in Node aImage, in long aImageX, in long aImageY, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer); */
    /// Unable to generate binding because `native type mozilla::dom::DataTransfer unsupported`
    pub InvokeDragSessionWithImage: *const ::libc::c_void,

    /* [can_run_script,noscript] void invokeDragSessionWithRemoteImage (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in RemoteDragStartDataPtr aDragStartData, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer); */
    /// Unable to generate binding because `native type mozilla::dom::RemoteDragStartData unsupported`
    pub InvokeDragSessionWithRemoteImage: *const ::libc::c_void,

    /* [can_run_script] void invokeDragSessionWithSelection (in Selection aSelection, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer); */
    /// Unable to generate binding because `native type mozilla::dom::DataTransfer unsupported`
    pub InvokeDragSessionWithSelection: *const ::libc::c_void,

    /* nsIDragSession getCurrentSession (); */
    pub GetCurrentSession: unsafe extern "system" fn (this: *const nsIDragService, _retval: *mut *const nsIDragSession) -> ::nserror::nsresult,

    /* void startDragSession (); */
    pub StartDragSession: unsafe extern "system" fn (this: *const nsIDragService) -> ::nserror::nsresult,

    /* void startDragSessionForTests (in unsigned long aAllowedEffect); */
    pub StartDragSessionForTests: unsafe extern "system" fn (this: *const nsIDragService, aAllowedEffect: u32) -> ::nserror::nsresult,

    /* [can_run_script] void endDragSession (in boolean aDoneDrag, [optional] in unsigned long aKeyModifiers); */
    pub EndDragSession: unsafe extern "system" fn (this: *const nsIDragService, aDoneDrag: bool, aKeyModifiers: u32) -> ::nserror::nsresult,

    /* [can_run_script,noscript] void fireDragEventAtSource (in EventMessage aEventMessage, in unsigned long aKeyModifiers); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub FireDragEventAtSource: *const ::libc::c_void,

    /* [can_run_script] void suppress (); */
    pub Suppress: unsafe extern "system" fn (this: *const nsIDragService) -> ::nserror::nsresult,

    /* void unsuppress (); */
    pub Unsuppress: unsafe extern "system" fn (this: *const nsIDragService) -> ::nserror::nsresult,

    /* [noscript] void dragMoved (in long aX, in long aY); */
    pub DragMoved: unsafe extern "system" fn (this: *const nsIDragService, aX: i32, aY: i32) -> ::nserror::nsresult,

    /* [nostdcall,notxpcom] boolean maybeAddChildProcess (in ContentParentPtr aChild); */
    /// Unable to generate binding because `native type mozilla::dom::ContentParent unsupported`
    pub MaybeAddChildProcess: *const ::libc::c_void,

    /* [nostdcall,notxpcom] boolean removeAllChildProcesses (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub RemoveAllChildProcesses: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDragService {

    pub const DRAGDROP_ACTION_NONE: i64 = 0;


    pub const DRAGDROP_ACTION_COPY: i64 = 1;


    pub const DRAGDROP_ACTION_MOVE: i64 = 2;


    pub const DRAGDROP_ACTION_LINK: i64 = 4;


    pub const DRAGDROP_ACTION_UNINITIALIZED: i64 = 64;

    /// ```text
    /// /**
    ///     * Starts a modal drag session with an array of transaferables.
    ///     *
    ///     * Note: This method is deprecated for non-native code.
    ///     *
    ///     * @param  aPrincipal - the triggering principal of the drag, or null if
    ///     *                      it's from browser chrome or OS
    ///     * @param aCsp - The csp of the triggering Document
    ///     * @param  aTransferables - an array of transferables to be dragged
    ///     * @param  aActionType - specified which of copy/move/link are allowed
    ///     * @param  aContentPolicyType - the contentPolicyType that will be
    ///     *           passed to the loadInfo when creating a new channel
    ///     *           (defaults to TYPE_OTHER)
    ///     */
    /// ```
    ///

    /// `[can_run_script] void invokeDragSession (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferables, in unsigned long aActionType, [optional] in nsContentPolicyType aContentPolicyType);`
    #[inline]
    pub unsafe fn InvokeDragSession(&self, aDOMNode: *const libc::c_void, aPrincipal: *const nsIPrincipal, aCsp: *const nsIContentSecurityPolicy, aCookieJarSettings: *const nsICookieJarSettings, aTransferables: *const nsIArray, aActionType: u32, aContentPolicyType: nsContentPolicyType) -> ::nserror::nsresult {
        ((*self.vtable).InvokeDragSession)(self, aDOMNode, aPrincipal, aCsp, aCookieJarSettings, aTransferables, aActionType, aContentPolicyType)
    }


    /// ```text
    /// /**
    ///    * Starts a modal drag session using an image. The first four arguments are
    ///    * the same as invokeDragSession.
    ///    *
    ///    * Note: This method is deprecated for non-native code.
    ///    *
    ///    * A custom image may be specified using the aImage argument. If this is
    ///    * supplied, the aImageX and aImageY arguments specify the offset within
    ///    * the image where the cursor would be positioned. That is, when the image
    ///    * is drawn, it is offset up and left the amount so that the cursor appears
    ///    * at that location within the image.
    ///    *
    ///    * If aImage is null, aImageX and aImageY are not used and the image is instead
    ///    * determined from the source node aDOMNode, and the offset calculated so that
    ///    * the initial location for the image appears in the same screen position as
    ///    * where the element is located. The node must be within a document.
    ///    *
    ///    * Currently, supported images are all DOM nodes. If this is an HTML <image> or
    ///    * <canvas>, the drag image is taken from the image data. If the element is in
    ///    * a document, it will be rendered at its displayed size, othewise, it will be
    ///    * rendered at its real size. For other types of elements, the element is
    ///    * rendered into an offscreen buffer in the same manner as it is currently
    ///    * displayed. The document selection is hidden while drawing.
    ///    *
    ///    * The aDragEvent must be supplied as the current screen coordinates of the
    ///    * event are needed to calculate the image location.
    ///    */
    /// ```
    ///

    /// `[can_run_script,noscript] void invokeDragSessionWithImage (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in Node aImage, in long aImageX, in long aImageY, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer);`
    const _InvokeDragSessionWithImage: () = ();

    /// ```text
    /// /** Start a drag session with the data in aDragStartData from a child process.
    ///    *  Other arguments are the same as invokeDragSessionWithImage.
    ///    */
    /// ```
    ///

    /// `[can_run_script,noscript] void invokeDragSessionWithRemoteImage (in Node aDOMNode, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in RemoteDragStartDataPtr aDragStartData, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer);`
    const _InvokeDragSessionWithRemoteImage: () = ();

    /// ```text
    /// /**
    ///    * Start a modal drag session using the selection as the drag image.
    ///    * The aDragEvent must be supplied as the current screen coordinates of the
    ///    * event are needed to calculate the image location.
    ///    *
    ///    * Note: This method is deprecated for non-native code.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void invokeDragSessionWithSelection (in Selection aSelection, in nsIPrincipal aPrincipal, in nsIContentSecurityPolicy aCsp, in nsICookieJarSettings aCookieJarSettings, in nsIArray aTransferableArray, in unsigned long aActionType, in DragEvent aDragEvent, in DataTransferPtr aDataTransfer);`
    const _InvokeDragSessionWithSelection: () = ();

    /// ```text
    /// /**
    ///     * Returns the current Drag Session
    ///     */
    /// ```
    ///

    /// `nsIDragSession getCurrentSession ();`
    #[inline]
    pub unsafe fn GetCurrentSession(&self, _retval: *mut *const nsIDragSession) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentSession)(self, _retval)
    }


    /// ```text
    /// /**
    ///     * Tells the Drag Service to start a drag session. This is called when
    ///     * an external drag occurs
    ///     */
    /// ```
    ///

    /// `void startDragSession ();`
    #[inline]
    pub unsafe fn StartDragSession(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).StartDragSession)(self, )
    }


    /// ```text
    /// /**
    ///    * Similar to startDragSession(), automated tests may want to start
    ///    * session for emulating an external drag.  At that time, this should
    ///    * be used instead of startDragSession().
    ///    *
    ///    * @param aAllowedEffect Set default drag action which means allowed effects
    ///    *                       in the session and every DnD events are initialized
    ///    *                       with one of specified value.  So, the value can be
    ///    *                       DRAGDROP_ACTION_NONE, or one or more values of
    ///    *                       DRAGDROP_ACTION_(MOVE|COPY|LINK).
    ///    */
    /// ```
    ///

    /// `void startDragSessionForTests (in unsigned long aAllowedEffect);`
    #[inline]
    pub unsafe fn StartDragSessionForTests(&self, aAllowedEffect: u32) -> ::nserror::nsresult {
        ((*self.vtable).StartDragSessionForTests)(self, aAllowedEffect)
    }


    /// ```text
    /// /**
    ///     * Tells the Drag Service to end a drag session. This is called when
    ///     * an external drag occurs
    ///     *
    ///     * If aDoneDrag is true, the drag has finished, otherwise the drag has
    ///     * just left the window.
    ///     */
    /// ```
    ///

    /// `[can_run_script] void endDragSession (in boolean aDoneDrag, [optional] in unsigned long aKeyModifiers);`
    #[inline]
    pub unsafe fn EndDragSession(&self, aDoneDrag: bool, aKeyModifiers: u32) -> ::nserror::nsresult {
        ((*self.vtable).EndDragSession)(self, aDoneDrag, aKeyModifiers)
    }


    /// ```text
    /// /**
    ///    * Fire a drag event at the source of the drag
    ///    */
    /// ```
    ///

    /// `[can_run_script,noscript] void fireDragEventAtSource (in EventMessage aEventMessage, in unsigned long aKeyModifiers);`
    const _FireDragEventAtSource: () = ();

    /// ```text
    /// /**
    ///    * Increase/decrease dragging suppress level by one.
    ///    * If level is greater than one, dragging is disabled.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void suppress ();`
    #[inline]
    pub unsafe fn Suppress(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Suppress)(self, )
    }



    /// `void unsuppress ();`
    #[inline]
    pub unsafe fn Unsuppress(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Unsuppress)(self, )
    }


    /// ```text
    /// /**
    ///     * aX and aY are in LayoutDevice pixels.
    ///     */
    /// ```
    ///

    /// `[noscript] void dragMoved (in long aX, in long aY);`
    #[inline]
    pub unsafe fn DragMoved(&self, aX: i32, aY: i32) -> ::nserror::nsresult {
        ((*self.vtable).DragMoved)(self, aX, aY)
    }



    /// `[nostdcall,notxpcom] boolean maybeAddChildProcess (in ContentParentPtr aChild);`
    const _MaybeAddChildProcess: () = ();


    /// `[nostdcall,notxpcom] boolean removeAllChildProcesses ();`
    const _RemoveAllChildProcesses: () = ();

}


