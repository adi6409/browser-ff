//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLAbsPosEditor.idl
//


/// `interface nsIHTMLAbsPosEditor : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHTMLAbsPosEditor {
    vtable: *const nsIHTMLAbsPosEditorVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHTMLAbsPosEditor.
unsafe impl XpCom for nsIHTMLAbsPosEditor {
    const IID: nsIID = nsID(0x91375f52, 0x20e6, 0x4757,
        [0x98, 0x35, 0xeb, 0x04, 0xfa, 0xbe, 0x54, 0x98]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHTMLAbsPosEditor {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHTMLAbsPosEditor.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHTMLAbsPosEditorCoerce {
    /// Cheaply cast a value of this type from a `nsIHTMLAbsPosEditor`.
    fn coerce_from(v: &nsIHTMLAbsPosEditor) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHTMLAbsPosEditorCoerce for nsIHTMLAbsPosEditor {
    #[inline]
    fn coerce_from(v: &nsIHTMLAbsPosEditor) -> &Self {
        v
    }
}

impl nsIHTMLAbsPosEditor {
    /// Cast this `nsIHTMLAbsPosEditor` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHTMLAbsPosEditorCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHTMLAbsPosEditor {
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
impl<T: nsISupportsCoerce> nsIHTMLAbsPosEditorCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLAbsPosEditor) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHTMLAbsPosEditor
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHTMLAbsPosEditorVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] attribute boolean absolutePositioningEnabled; */
    pub GetAbsolutePositioningEnabled: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor, aAbsolutePositioningEnabled: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] attribute boolean absolutePositioningEnabled; */
    pub SetAbsolutePositioningEnabled: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor, aAbsolutePositioningEnabled: bool) -> ::nserror::nsresult,

    /* attribute boolean snapToGridEnabled; */
    pub GetSnapToGridEnabled: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor, aSnapToGridEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean snapToGridEnabled; */
    pub SetSnapToGridEnabled: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor, aSnapToGridEnabled: bool) -> ::nserror::nsresult,

    /* attribute unsigned long gridSize; */
    pub GetGridSize: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor, aGridSize: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long gridSize; */
    pub SetGridSize: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor, aGridSize: u32) -> ::nserror::nsresult,

    /* [can_run_script] void refreshGrabber (); */
    pub RefreshGrabber: unsafe extern "system" fn (this: *const nsIHTMLAbsPosEditor) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHTMLAbsPosEditor {

    /// ```text
    /// /**
    ///    * true if Absolute Positioning handling is enabled in the editor
    ///    */
    /// ```
    ///

    /// `[can_run_script] attribute boolean absolutePositioningEnabled;`
    #[inline]
    pub unsafe fn GetAbsolutePositioningEnabled(&self, aAbsolutePositioningEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAbsolutePositioningEnabled)(self, aAbsolutePositioningEnabled)
    }


    /// ```text
    /// /**
    ///    * true if Absolute Positioning handling is enabled in the editor
    ///    */
    /// ```
    ///

    /// `[can_run_script] attribute boolean absolutePositioningEnabled;`
    #[inline]
    pub unsafe fn SetAbsolutePositioningEnabled(&self, aAbsolutePositioningEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAbsolutePositioningEnabled)(self, aAbsolutePositioningEnabled)
    }


    /// ```text
    /// /**
    ///    * true if Snap To Grid is enabled in the editor.
    ///    */
    /// ```
    ///

    /// `attribute boolean snapToGridEnabled;`
    #[inline]
    pub unsafe fn GetSnapToGridEnabled(&self, aSnapToGridEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSnapToGridEnabled)(self, aSnapToGridEnabled)
    }


    /// ```text
    /// /**
    ///    * true if Snap To Grid is enabled in the editor.
    ///    */
    /// ```
    ///

    /// `attribute boolean snapToGridEnabled;`
    #[inline]
    pub unsafe fn SetSnapToGridEnabled(&self, aSnapToGridEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSnapToGridEnabled)(self, aSnapToGridEnabled)
    }


    /// ```text
    /// /**
    ///    * sets the grid size in pixels.
    ///    * @param aSizeInPixels [IN] the size of the grid in pixels
    ///    */
    /// ```
    ///

    /// `attribute unsigned long gridSize;`
    #[inline]
    pub unsafe fn GetGridSize(&self, aGridSize: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetGridSize)(self, aGridSize)
    }


    /// ```text
    /// /**
    ///    * sets the grid size in pixels.
    ///    * @param aSizeInPixels [IN] the size of the grid in pixels
    ///    */
    /// ```
    ///

    /// `attribute unsigned long gridSize;`
    #[inline]
    pub unsafe fn SetGridSize(&self, aGridSize: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetGridSize)(self, aGridSize)
    }


    /// ```text
    /// /**
    ///    * refreshes the grabber if it shown, possibly updating its position or
    ///    * even hiding it.
    ///    * FYI: Current user in script is only BlueGriffon.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void refreshGrabber ();`
    #[inline]
    pub unsafe fn RefreshGrabber(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RefreshGrabber)(self, )
    }


}


