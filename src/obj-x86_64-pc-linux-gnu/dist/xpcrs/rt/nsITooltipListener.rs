//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsITooltipListener.idl
//


/// `interface nsITooltipListener : nsISupports`
///

/// ```text
/// /**
///  * An optional interface for embedding clients wishing to receive
///  * notifications for when a tooltip should be displayed or removed.
///  * The embedder implements this interface on the web browser chrome
///  * object associated with the window that notifications are required
///  * for.
///  *
///  * @see nsITooltipTextProvider
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITooltipListener {
    vtable: *const nsITooltipListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITooltipListener.
unsafe impl XpCom for nsITooltipListener {
    const IID: nsIID = nsID(0x44b78386, 0x1dd2, 0x11b2,
        [0x9a, 0xd2, 0xe4, 0xee, 0xe2, 0xca, 0x19, 0x16]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITooltipListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITooltipListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITooltipListenerCoerce {
    /// Cheaply cast a value of this type from a `nsITooltipListener`.
    fn coerce_from(v: &nsITooltipListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITooltipListenerCoerce for nsITooltipListener {
    #[inline]
    fn coerce_from(v: &nsITooltipListener) -> &Self {
        v
    }
}

impl nsITooltipListener {
    /// Cast this `nsITooltipListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITooltipListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITooltipListener {
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
impl<T: nsISupportsCoerce> nsITooltipListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITooltipListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITooltipListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITooltipListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onShowTooltip (in long aXCoords, in long aYCoords, in AString aTipText, in AString aTipDir); */
    pub OnShowTooltip: unsafe extern "system" fn (this: *const nsITooltipListener, aXCoords: i32, aYCoords: i32, aTipText: *const ::nsstring::nsAString, aTipDir: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* void onHideTooltip (); */
    pub OnHideTooltip: unsafe extern "system" fn (this: *const nsITooltipListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITooltipListener {

    /// ```text
    /// /**
    ///      * Called when a tooltip should be displayed.
    ///      *
    ///      * @param aXCoords The tooltip left edge X coordinate.
    ///      * @param aYCoords The tooltip top edge Y coordinate.
    ///      * @param aTipText The text to display in the tooltip, typically obtained
    ///      *        from the TITLE attribute of the node (or containing parent)
    ///      *        over which the pointer has been positioned.
    ///      * @param aTipDir  The direction (ltr or rtl) in which to display the text
    ///      *
    ///      * @note
    ///      * Coordinates are specified in pixels, relative to the top-left
    ///      * corner of the browser area.
    ///      *
    ///      * @return <code>NS_OK</code> if the tooltip was displayed.
    ///      */
    /// ```
    ///

    /// `void onShowTooltip (in long aXCoords, in long aYCoords, in AString aTipText, in AString aTipDir);`
    #[inline]
    pub unsafe fn OnShowTooltip(&self, aXCoords: i32, aYCoords: i32, aTipText: *const ::nsstring::nsAString, aTipDir: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).OnShowTooltip)(self, aXCoords, aYCoords, aTipText, aTipDir)
    }


    /// ```text
    /// /**
    ///      * Called when the tooltip should be hidden, either because the pointer
    ///      * has moved or the tooltip has timed out.
    ///      */
    /// ```
    ///

    /// `void onHideTooltip ();`
    #[inline]
    pub unsafe fn OnHideTooltip(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnHideTooltip)(self, )
    }


}


