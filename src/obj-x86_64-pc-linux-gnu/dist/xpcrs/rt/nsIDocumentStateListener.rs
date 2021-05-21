//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/nsIDocumentStateListener.idl
//


/// `interface nsIDocumentStateListener : nsISupports`
///

/// ```text
/// /**
///  * Due to the historical reason, this listener interface says "document state",
///  * but this listener listens to HTML editor state.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocumentStateListener {
    vtable: *const nsIDocumentStateListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocumentStateListener.
unsafe impl XpCom for nsIDocumentStateListener {
    const IID: nsIID = nsID(0x050cdc00, 0x3b8e, 0x11d3,
        [0x9c, 0xe4, 0xa4, 0x58, 0xf4, 0x54, 0xfc, 0xbc]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocumentStateListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocumentStateListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocumentStateListenerCoerce {
    /// Cheaply cast a value of this type from a `nsIDocumentStateListener`.
    fn coerce_from(v: &nsIDocumentStateListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocumentStateListenerCoerce for nsIDocumentStateListener {
    #[inline]
    fn coerce_from(v: &nsIDocumentStateListener) -> &Self {
        v
    }
}

impl nsIDocumentStateListener {
    /// Cast this `nsIDocumentStateListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocumentStateListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocumentStateListener {
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
impl<T: nsISupportsCoerce> nsIDocumentStateListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocumentStateListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocumentStateListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocumentStateListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void NotifyDocumentWillBeDestroyed (); */
    pub NotifyDocumentWillBeDestroyed: unsafe extern "system" fn (this: *const nsIDocumentStateListener) -> ::nserror::nsresult,

    /* [can_run_script] void NotifyDocumentStateChanged (in boolean aNowDirty); */
    pub NotifyDocumentStateChanged: unsafe extern "system" fn (this: *const nsIDocumentStateListener, aNowDirty: bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocumentStateListener {

    /// ```text
    /// /**
    ///    * NotifyDocumentWillBeDestroyed() is called when HTML editor instance is
    ///    * being destroyed.  Note that related objects may have already gone when
    ///    * this is called because that may cause destroying HTML editor.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void NotifyDocumentWillBeDestroyed ();`
    #[inline]
    pub unsafe fn NotifyDocumentWillBeDestroyed(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).NotifyDocumentWillBeDestroyed)(self, )
    }


    /// ```text
    /// /**
    ///    * NotifyDocumentStateChanged() is called when dirty state of HTML editor
    ///    * is changed.
    ///    *
    ///    * @param aNowDirty   if true, this is called when the HTML editor becomes
    ///    *                    dirty.  Otherwise, called when it becomes not dirty.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void NotifyDocumentStateChanged (in boolean aNowDirty);`
    #[inline]
    pub unsafe fn NotifyDocumentStateChanged(&self, aNowDirty: bool) -> ::nserror::nsresult {
        ((*self.vtable).NotifyDocumentStateChanged)(self, aNowDirty)
    }


}


