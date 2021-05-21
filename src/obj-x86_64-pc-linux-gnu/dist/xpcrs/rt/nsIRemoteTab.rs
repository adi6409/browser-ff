//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/interfaces/base/nsIRemoteTab.idl
//


/// `interface nsIRemoteTab : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIRemoteTab {
    vtable: *const nsIRemoteTabVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIRemoteTab.
unsafe impl XpCom for nsIRemoteTab {
    const IID: nsIID = nsID(0x8e49f7b0, 0x1f98, 0x4939,
        [0xbf, 0x91, 0xe9, 0xc3, 0x9c, 0xd5, 0x64, 0x34]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIRemoteTab {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIRemoteTab.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIRemoteTabCoerce {
    /// Cheaply cast a value of this type from a `nsIRemoteTab`.
    fn coerce_from(v: &nsIRemoteTab) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIRemoteTabCoerce for nsIRemoteTab {
    #[inline]
    fn coerce_from(v: &nsIRemoteTab) -> &Self {
        v
    }
}

impl nsIRemoteTab {
    /// Cast this `nsIRemoteTab` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIRemoteTabCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIRemoteTab {
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
impl<T: nsISupportsCoerce> nsIRemoteTabCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIRemoteTab) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIRemoteTab
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIRemoteTabVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean renderLayers; */
    pub GetRenderLayers: unsafe extern "system" fn (this: *const nsIRemoteTab, aRenderLayers: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean renderLayers; */
    pub SetRenderLayers: unsafe extern "system" fn (this: *const nsIRemoteTab, aRenderLayers: bool) -> ::nserror::nsresult,

    /* readonly attribute boolean hasLayers; */
    pub GetHasLayers: unsafe extern "system" fn (this: *const nsIRemoteTab, aHasLayers: *mut bool) -> ::nserror::nsresult,

    /* void deprioritize (); */
    pub Deprioritize: unsafe extern "system" fn (this: *const nsIRemoteTab) -> ::nserror::nsresult,

    /* void preserveLayers (in boolean aPreserveLayers); */
    pub PreserveLayers: unsafe extern "system" fn (this: *const nsIRemoteTab, aPreserveLayers: bool) -> ::nserror::nsresult,

    /* readonly attribute uint64_t tabId; */
    pub GetTabId: unsafe extern "system" fn (this: *const nsIRemoteTab, aTabId: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute uint64_t contentProcessId; */
    pub GetContentProcessId: unsafe extern "system" fn (this: *const nsIRemoteTab, aContentProcessId: *mut uint64_t) -> ::nserror::nsresult,

    /* readonly attribute int32_t osPid; */
    pub GetOsPid: unsafe extern "system" fn (this: *const nsIRemoteTab, aOsPid: *mut int32_t) -> ::nserror::nsresult,

    /* readonly attribute boolean hasPresented; */
    pub GetHasPresented: unsafe extern "system" fn (this: *const nsIRemoteTab, aHasPresented: *mut bool) -> ::nserror::nsresult,

    /* void transmitPermissionsForPrincipal (in nsIPrincipal aPrincipal); */
    pub TransmitPermissionsForPrincipal: unsafe extern "system" fn (this: *const nsIRemoteTab, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult,

    /* boolean startApzAutoscroll (in float aAnchorX, in float aAnchorY, in nsViewID aScrollId, in uint32_t aPresShellId); */
    pub StartApzAutoscroll: unsafe extern "system" fn (this: *const nsIRemoteTab, aAnchorX: libc::c_float, aAnchorY: libc::c_float, aScrollId: nsViewID, aPresShellId: uint32_t, _retval: *mut bool) -> ::nserror::nsresult,

    /* void stopApzAutoscroll (in nsViewID aScrollId, in uint32_t aPresShellId); */
    pub StopApzAutoscroll: unsafe extern "system" fn (this: *const nsIRemoteTab, aScrollId: nsViewID, aPresShellId: uint32_t) -> ::nserror::nsresult,

    /* [binaryname(MaybeCancelContentJSExecutionFromScript),implicit_jscontext] void maybeCancelContentJSExecution (in nsIRemoteTab_NavigationType aNavigationType, [optional] in jsval aCancelContentJSOptions); */
    /// Unable to generate binding because `specialtype jsval unsupported`
    pub MaybeCancelContentJSExecutionFromScript: *const ::libc::c_void,

    /* [noscript] void notifyResolutionChanged (); */
    pub NotifyResolutionChanged: unsafe extern "system" fn (this: *const nsIRemoteTab) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIRemoteTab {

    /// ```text
    /// /**
    ///    * When set to true, this tells the child to paint and upload layers to
    ///    * the compositor. When set to false, previous layers are cleared from
    ///    * the compositor, but only if preserveLayers is also set to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean renderLayers;`
    #[inline]
    pub unsafe fn GetRenderLayers(&self, aRenderLayers: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetRenderLayers)(self, aRenderLayers)
    }


    /// ```text
    /// /**
    ///    * When set to true, this tells the child to paint and upload layers to
    ///    * the compositor. When set to false, previous layers are cleared from
    ///    * the compositor, but only if preserveLayers is also set to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean renderLayers;`
    #[inline]
    pub unsafe fn SetRenderLayers(&self, aRenderLayers: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetRenderLayers)(self, aRenderLayers)
    }


    /// ```text
    /// /**
    ///    * True if layers are being rendered and the compositor has reported
    ///    * receiving them.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean hasLayers;`
    #[inline]
    pub unsafe fn GetHasLayers(&self, aHasLayers: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasLayers)(self, aHasLayers)
    }


    /// ```text
    /// /**
    ///    * Adjusts the tab's active state in the process priority manager,
    ///    * allowing its process to be given a lower priority.
    ///    */
    /// ```
    ///

    /// `void deprioritize ();`
    #[inline]
    pub unsafe fn Deprioritize(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Deprioritize)(self, )
    }


    /// ```text
    /// /**
    ///    * As an optimisation, setting the docshell's active state to
    ///    * inactive also triggers a layer invalidation to free up some
    ///    * potentially unhelpful memory usage. Calling preserveLayers
    ///    * will cause the layers to be preserved even for inactive
    ///    * docshells.
    ///    */
    /// ```
    ///

    /// `void preserveLayers (in boolean aPreserveLayers);`
    #[inline]
    pub unsafe fn PreserveLayers(&self, aPreserveLayers: bool) -> ::nserror::nsresult {
        ((*self.vtable).PreserveLayers)(self, aPreserveLayers)
    }



    /// `readonly attribute uint64_t tabId;`
    #[inline]
    pub unsafe fn GetTabId(&self, aTabId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetTabId)(self, aTabId)
    }



    /// `readonly attribute uint64_t contentProcessId;`
    #[inline]
    pub unsafe fn GetContentProcessId(&self, aContentProcessId: *mut uint64_t) -> ::nserror::nsresult {
        ((*self.vtable).GetContentProcessId)(self, aContentProcessId)
    }


    /// ```text
    /// /**
    ///    * The OS level process Id of the related child process.
    ///    */
    /// ```
    ///

    /// `readonly attribute int32_t osPid;`
    #[inline]
    pub unsafe fn GetOsPid(&self, aOsPid: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetOsPid)(self, aOsPid)
    }


    /// ```text
    /// /**
    ///    * True if we've previously received layers for this tab when switching to
    ///    * it.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean hasPresented;`
    #[inline]
    pub unsafe fn GetHasPresented(&self, aHasPresented: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasPresented)(self, aHasPresented)
    }


    /// ```text
    /// /**
    ///    * Ensures that the content process which has this remote tab has all of the
    ///    * permissions required to load a document with the given principal.
    ///    */
    /// ```
    ///

    /// `void transmitPermissionsForPrincipal (in nsIPrincipal aPrincipal);`
    #[inline]
    pub unsafe fn TransmitPermissionsForPrincipal(&self, aPrincipal: *const nsIPrincipal) -> ::nserror::nsresult {
        ((*self.vtable).TransmitPermissionsForPrincipal)(self, aPrincipal)
    }


    /// ```text
    /// /**
    ///    * Notify APZ to start autoscrolling.
    ///    * (aAnchorX, aAnchorY) are the coordinates of the autoscroll anchor,
    ///    * in CSS coordinates relative to the screen. aScrollId and
    ///    * aPresShellId identify the scroll frame that content chose to scroll.
    ///    * Returns whether we were successfully able to notify APZ.
    ///    * If this function returns true, APZ (which may live in another process)
    ///    * may still reject the autoscroll, but it's then APZ's reponsibility
    ///    * to notify content via an "autoscroll-rejected-by-apz" message.
    ///    */
    /// ```
    ///

    /// `boolean startApzAutoscroll (in float aAnchorX, in float aAnchorY, in nsViewID aScrollId, in uint32_t aPresShellId);`
    #[inline]
    pub unsafe fn StartApzAutoscroll(&self, aAnchorX: libc::c_float, aAnchorY: libc::c_float, aScrollId: nsViewID, aPresShellId: uint32_t, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).StartApzAutoscroll)(self, aAnchorX, aAnchorY, aScrollId, aPresShellId, _retval)
    }


    /// ```text
    /// /**
    ///    * Notify APZ to stop autoscrolling.
    ///    * aScrollId and aPresShellId identify the scroll frame that is being
    ///    * autoscrolled.
    ///    */
    /// ```
    ///

    /// `void stopApzAutoscroll (in nsViewID aScrollId, in uint32_t aPresShellId);`
    #[inline]
    pub unsafe fn StopApzAutoscroll(&self, aScrollId: nsViewID, aPresShellId: uint32_t) -> ::nserror::nsresult {
        ((*self.vtable).StopApzAutoscroll)(self, aScrollId, aPresShellId)
    }


    /// ```text
    /// /**
    ///    * Interrupt content scripts if possible/needed to allow chrome scripts in the
    ///    * content process to run (in particular, to allow navigating through browser
        ///    * history.
        ///    */
        /// ```
        ///

        /// `[binaryname(MaybeCancelContentJSExecutionFromScript),implicit_jscontext] void maybeCancelContentJSExecution (in nsIRemoteTab_NavigationType aNavigationType, [optional] in jsval aCancelContentJSOptions);`
        const _MaybeCancelContentJSExecutionFromScript: () = ();

        /// ```text
        /// /**
        ///    * Notify the remote tab that the resolution has changed.
        ///    */
        /// ```
        ///

        /// `[noscript] void notifyResolutionChanged ();`
        #[inline]
        pub unsafe fn NotifyResolutionChanged(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).NotifyResolutionChanged)(self, )
        }


    }


