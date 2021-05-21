//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHistoryListener.idl
//


/// `interface nsISHistoryListener : nsISupports`
///

/// ```text
/// /**
///  * nsISHistoryListener defines the interface one can implement to receive
///  * notifications about activities in session history and (for reloads) to be
///  * able to cancel them.
///  *
///  * A session history listener will be notified when pages are added, removed
///  * and loaded from session history. In the case of reloads, it can prevent them
///  * from happening by returning false from the corresponding callback method.
///  *
///  * A session history listener can be registered on a particular nsISHistory
///  * instance via the nsISHistory::addSHistoryListener() method.
///  *
///  * Listener methods should not alter the session history. Things are likely to
///  * go haywire if they do.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISHistoryListener {
    vtable: *const nsISHistoryListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISHistoryListener.
unsafe impl XpCom for nsISHistoryListener {
    const IID: nsIID = nsID(0x125c0833, 0x746a, 0x400e,
        [0x9b, 0x89, 0xd2, 0xd1, 0x85, 0x45, 0xc0, 0x8a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISHistoryListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISHistoryListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISHistoryListenerCoerce {
    /// Cheaply cast a value of this type from a `nsISHistoryListener`.
    fn coerce_from(v: &nsISHistoryListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISHistoryListenerCoerce for nsISHistoryListener {
    #[inline]
    fn coerce_from(v: &nsISHistoryListener) -> &Self {
        v
    }
}

impl nsISHistoryListener {
    /// Cast this `nsISHistoryListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISHistoryListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISHistoryListener {
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
impl<T: nsISupportsCoerce> nsISHistoryListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHistoryListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISHistoryListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISHistoryListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex); */
    pub OnHistoryNewEntry: unsafe extern "system" fn (this: *const nsISHistoryListener, aNewURI: *const nsIURI, aOldIndex: i32) -> ::nserror::nsresult,

    /* boolean OnHistoryReload (); */
    pub OnHistoryReload: unsafe extern "system" fn (this: *const nsISHistoryListener, _retval: *mut bool) -> ::nserror::nsresult,

    /* void OnHistoryGotoIndex (); */
    pub OnHistoryGotoIndex: unsafe extern "system" fn (this: *const nsISHistoryListener) -> ::nserror::nsresult,

    /* void OnHistoryPurge (); */
    pub OnHistoryPurge: unsafe extern "system" fn (this: *const nsISHistoryListener) -> ::nserror::nsresult,

    /* void OnHistoryReplaceEntry (); */
    pub OnHistoryReplaceEntry: unsafe extern "system" fn (this: *const nsISHistoryListener) -> ::nserror::nsresult,

    /* void OnContentViewerEvicted (in unsigned long aNumEvicted); */
    pub OnContentViewerEvicted: unsafe extern "system" fn (this: *const nsISHistoryListener, aNumEvicted: u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISHistoryListener {

    /// ```text
    /// /**
    ///    * Called when a new document is added to session history. New documents are
    ///    * added to session history by docshell when new pages are loaded in a frame
    ///    * or content area, for example via nsIWebNavigation::loadURI()
    ///    *
    ///    * @param aNewURI     The URI of the document to be added to session history.
    ///    * @param aOldIndex   The index of the current history item before the
    ///    *                    operation.
    ///    */
    /// ```
    ///

    /// `void OnHistoryNewEntry (in nsIURI aNewURI, in long aOldIndex);`
    #[inline]
    pub unsafe fn OnHistoryNewEntry(&self, aNewURI: *const nsIURI, aOldIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).OnHistoryNewEntry)(self, aNewURI, aOldIndex)
    }


    /// ```text
    /// /**
    ///    * Called before the current document is reloaded, for example due to a
    ///    * nsIWebNavigation::reload() call.
    ///    */
    /// ```
    ///

    /// `boolean OnHistoryReload ();`
    #[inline]
    pub unsafe fn OnHistoryReload(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).OnHistoryReload)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Called before navigating to a session history entry by index, for example,
    ///    * when nsIWebNavigation::gotoIndex() is called.
    ///    */
    /// ```
    ///

    /// `void OnHistoryGotoIndex ();`
    #[inline]
    pub unsafe fn OnHistoryGotoIndex(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnHistoryGotoIndex)(self, )
    }


    /// ```text
    /// /**
    ///    * Called before entries are removed from session history. Entries can be
    ///    * removed from session history for various reasons, for example to control
    ///    * the memory usage of the browser, to prevent users from loading documents
    ///    * from history, to erase evidence of prior page loads, etc.
    ///    *
    ///    * To purge documents from session history call nsISHistory::PurgeHistory().
    ///    */
    /// ```
    ///

    /// `void OnHistoryPurge ();`
    #[inline]
    pub unsafe fn OnHistoryPurge(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnHistoryPurge)(self, )
    }


    /// ```text
    /// /**
    ///    * Called before an entry is replaced in the session history. Entries are
    ///    * replaced when navigating away from non-persistent history entries (such as
        ///    * about pages) and when history.replaceState is called.
    ///    */
    /// ```
    ///

    /// `void OnHistoryReplaceEntry ();`
    #[inline]
    pub unsafe fn OnHistoryReplaceEntry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnHistoryReplaceEntry)(self, )
    }


    /// ```text
    /// /**
    ///    * Called whenever a content viewer is evicted. A content viewer is evicted
    ///    * whenever a bfcache entry has timed out or the number of total content
    ///    * viewers has exceeded the global max. This is used for testing only.
    ///    *
    ///    * @param aNumEvicted - number of content viewers evicted
    ///    */
    /// ```
    ///

    /// `void OnContentViewerEvicted (in unsigned long aNumEvicted);`
    #[inline]
    pub unsafe fn OnContentViewerEvicted(&self, aNumEvicted: u32) -> ::nserror::nsresult {
        ((*self.vtable).OnContentViewerEvicted)(self, aNumEvicted)
    }


}


