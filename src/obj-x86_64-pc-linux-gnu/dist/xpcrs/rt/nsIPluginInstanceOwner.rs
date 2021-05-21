//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/dom/plugins/base/nsIPluginInstanceOwner.idl
//


/// `interface nsIPluginInstanceOwner : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIPluginInstanceOwner {
    vtable: *const nsIPluginInstanceOwnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIPluginInstanceOwner.
unsafe impl XpCom for nsIPluginInstanceOwner {
    const IID: nsIID = nsID(0x7d65452e, 0xc167, 0x4cba,
        [0xa0, 0xe3, 0xdd, 0xc6, 0x1b, 0xdd, 0xe8, 0xc3]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIPluginInstanceOwner {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIPluginInstanceOwner.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIPluginInstanceOwnerCoerce {
    /// Cheaply cast a value of this type from a `nsIPluginInstanceOwner`.
    fn coerce_from(v: &nsIPluginInstanceOwner) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIPluginInstanceOwnerCoerce for nsIPluginInstanceOwner {
    #[inline]
    fn coerce_from(v: &nsIPluginInstanceOwner) -> &Self {
        v
    }
}

impl nsIPluginInstanceOwner {
    /// Cast this `nsIPluginInstanceOwner` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIPluginInstanceOwnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIPluginInstanceOwner {
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
impl<T: nsISupportsCoerce> nsIPluginInstanceOwnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIPluginInstanceOwner) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIPluginInstanceOwner
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIPluginInstanceOwnerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void setInstance (in nsNPAPIPluginInstancePtr aInstance); */
    /// Unable to generate binding because `native type nsNPAPIPluginInstance unsupported`
    pub SetInstance: *const ::libc::c_void,

    /* [nostdcall,notxpcom] nsNPAPIPluginInstancePtr getInstance (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetInstance: *const ::libc::c_void,

    /* void getWindow (in NPWindowStarRef aWindow); */
    /// Unable to generate binding because `native type NPWindow * unsupported`
    pub GetWindow: *const ::libc::c_void,

    /* readonly attribute int32_t mode; */
    pub GetMode: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner, aMode: *mut int32_t) -> ::nserror::nsresult,

    /* void createWidget (); */
    pub CreateWidget: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner) -> ::nserror::nsresult,

    /* readonly attribute Document document; */
    pub GetDocument: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner, aDocument: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* void invalidateRect (in NPRectPtr aRect); */
    /// Unable to generate binding because `native type NPRect unsupported`
    pub InvalidateRect: *const ::libc::c_void,

    /* void invalidateRegion (in NPRegion aRegion); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub InvalidateRegion: *const ::libc::c_void,

    /* void redrawPlugin (); */
    pub RedrawPlugin: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner) -> ::nserror::nsresult,

    /* void getNetscapeWindow (in voidPtr aValue); */
    pub GetNetscapeWindow: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner, aValue: *const libc::c_void) -> ::nserror::nsresult,

    /* void setEventModel (in int32_t eventModel); */
    pub SetEventModel: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner, eventModel: int32_t) -> ::nserror::nsresult,

    /* void callSetWindow (); */
    pub CallSetWindow: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner) -> ::nserror::nsresult,

    /* double getContentsScaleFactor (); */
    pub GetContentsScaleFactor: unsafe extern "system" fn (this: *const nsIPluginInstanceOwner, _retval: *mut libc::c_double) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIPluginInstanceOwner {

    /// ```text
    /// /**
    ///    * Let the owner know what its instance is
    ///    */
    /// ```
    ///

    /// `void setInstance (in nsNPAPIPluginInstancePtr aInstance);`
    const _SetInstance: () = ();

    /// ```text
    /// /**
    ///    * Get the instance associated with this owner.
    ///    */
    /// ```
    ///

    /// `[nostdcall,notxpcom] nsNPAPIPluginInstancePtr getInstance ();`
    const _GetInstance: () = ();

    /// ```text
    /// /**
    ///    * Get a handle to the window structure of the owner.
    ///    * This pointer cannot be made persistent by the caller.
    ///    */
    /// ```
    ///

    /// `void getWindow (in NPWindowStarRef aWindow);`
    const _GetWindow: () = ();

    /// ```text
    /// /**
    ///    * Get the display mode for the plugin instance.
    ///    */
    /// ```
    ///

    /// `readonly attribute int32_t mode;`
    #[inline]
    pub unsafe fn GetMode(&self, aMode: *mut int32_t) -> ::nserror::nsresult {
        ((*self.vtable).GetMode)(self, aMode)
    }


    /// ```text
    /// /**
    ///    * Create a place for the plugin to live in the owner's
    ///    * environment. this may or may not create a window
    ///    * depending on the windowless state of the plugin instance.
    ///    */
    /// ```
    ///

    /// `void createWidget ();`
    #[inline]
    pub unsafe fn CreateWidget(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CreateWidget)(self, )
    }


    /// ```text
    /// /**
    ///    * Get the associated document.
    ///    */
    /// ```
    ///

    /// `readonly attribute Document document;`
    #[inline]
    pub unsafe fn GetDocument(&self, aDocument: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetDocument)(self, aDocument)
    }


    /// ```text
    /// /**
    ///    * Invalidate the rectangle
    ///    */
    /// ```
    ///

    /// `void invalidateRect (in NPRectPtr aRect);`
    const _InvalidateRect: () = ();

    /// ```text
    /// /**
    ///    * Invalidate the region
    ///    */
    /// ```
    ///

    /// `void invalidateRegion (in NPRegion aRegion);`
    const _InvalidateRegion: () = ();

    /// ```text
    /// /**
    ///    * Have the plugin recomposited.
    ///    */
    /// ```
    ///

    /// `void redrawPlugin ();`
    #[inline]
    pub unsafe fn RedrawPlugin(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RedrawPlugin)(self, )
    }


    /// ```text
    /// /**
    ///    * Get NetscapeWindow, corresponds to NPNVnetscapeWindow
    ///    */
    /// ```
    ///

    /// `void getNetscapeWindow (in voidPtr aValue);`
    #[inline]
    pub unsafe fn GetNetscapeWindow(&self, aValue: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetNetscapeWindow)(self, aValue)
    }


    /// ```text
    /// /**
    ///    * Convert between plugin, window, and screen coordinate spaces.
    ///    */
    /// ```
    ///

    /// `void setEventModel (in int32_t eventModel);`
    #[inline]
    pub unsafe fn SetEventModel(&self, eventModel: int32_t) -> ::nserror::nsresult {
        ((*self.vtable).SetEventModel)(self, eventModel)
    }


    /// ```text
    /// /**
    ///    * Call NPP_SetWindow on the plugin.
    ///    */
    /// ```
    ///

    /// `void callSetWindow ();`
    #[inline]
    pub unsafe fn CallSetWindow(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).CallSetWindow)(self, )
    }


    /// ```text
    /// /**
    ///    * Get the contents scale factor for the screen the plugin is
    ///    * drawn on.
    ///    */
    /// ```
    ///

    /// `double getContentsScaleFactor ();`
    #[inline]
    pub unsafe fn GetContentsScaleFactor(&self, _retval: *mut libc::c_double) -> ::nserror::nsresult {
        ((*self.vtable).GetContentsScaleFactor)(self, _retval)
    }


}


