//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIHTMLObjectResizer.idl
//


/// `interface nsIHTMLObjectResizer : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIHTMLObjectResizer {
    vtable: *const nsIHTMLObjectResizerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIHTMLObjectResizer.
unsafe impl XpCom for nsIHTMLObjectResizer {
    const IID: nsIID = nsID(0x8b396020, 0x69d3, 0x451f,
        [0x80, 0xc1, 0x1a, 0x96, 0xa7, 0xda, 0x25, 0xa9]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIHTMLObjectResizer {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIHTMLObjectResizer.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIHTMLObjectResizerCoerce {
    /// Cheaply cast a value of this type from a `nsIHTMLObjectResizer`.
    fn coerce_from(v: &nsIHTMLObjectResizer) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIHTMLObjectResizerCoerce for nsIHTMLObjectResizer {
    #[inline]
    fn coerce_from(v: &nsIHTMLObjectResizer) -> &Self {
        v
    }
}

impl nsIHTMLObjectResizer {
    /// Cast this `nsIHTMLObjectResizer` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIHTMLObjectResizerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIHTMLObjectResizer {
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
impl<T: nsISupportsCoerce> nsIHTMLObjectResizerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIHTMLObjectResizer) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIHTMLObjectResizer
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIHTMLObjectResizerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] attribute boolean objectResizingEnabled; */
    pub GetObjectResizingEnabled: unsafe extern "system" fn (this: *const nsIHTMLObjectResizer, aObjectResizingEnabled: *mut bool) -> ::nserror::nsresult,

    /* [can_run_script] attribute boolean objectResizingEnabled; */
    pub SetObjectResizingEnabled: unsafe extern "system" fn (this: *const nsIHTMLObjectResizer, aObjectResizingEnabled: bool) -> ::nserror::nsresult,

    /* void hideResizers (); */
    pub HideResizers: unsafe extern "system" fn (this: *const nsIHTMLObjectResizer) -> ::nserror::nsresult,

    /* [can_run_script] void refreshResizers (); */
    pub RefreshResizers: unsafe extern "system" fn (this: *const nsIHTMLObjectResizer) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIHTMLObjectResizer {

    pub const eTopLeft: i64 = 0;


    pub const eTop: i64 = 1;


    pub const eTopRight: i64 = 2;


    pub const eLeft: i64 = 3;


    pub const eRight: i64 = 4;


    pub const eBottomLeft: i64 = 5;


    pub const eBottom: i64 = 6;


    pub const eBottomRight: i64 = 7;

    /// ```text
    /// /**
    ///    * a boolean indicating if object resizing is enabled in the editor
    ///    */
    /// ```
    ///

    /// `[can_run_script] attribute boolean objectResizingEnabled;`
    #[inline]
    pub unsafe fn GetObjectResizingEnabled(&self, aObjectResizingEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetObjectResizingEnabled)(self, aObjectResizingEnabled)
    }


    /// ```text
    /// /**
    ///    * a boolean indicating if object resizing is enabled in the editor
    ///    */
    /// ```
    ///

    /// `[can_run_script] attribute boolean objectResizingEnabled;`
    #[inline]
    pub unsafe fn SetObjectResizingEnabled(&self, aObjectResizingEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetObjectResizingEnabled)(self, aObjectResizingEnabled)
    }


    /// ```text
    /// /**
    ///    * Hide resizers if they are visible.  If this is called while there is no
    ///    * visible resizers, this does not throw exception, just does nothing.
    ///    */
    /// ```
    ///

    /// `void hideResizers ();`
    #[inline]
    pub unsafe fn HideResizers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).HideResizers)(self, )
    }


    /// ```text
    /// /**
    ///    * Refresh positions of resizers.  If you change size of target of resizers,
    ///    * you need to refresh position of resizers with calling this.
    ///    * FYI: Current user in script is only BlueGriffon.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void refreshResizers ();`
    #[inline]
    pub unsafe fn RefreshResizers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RefreshResizers)(self, )
    }


}


