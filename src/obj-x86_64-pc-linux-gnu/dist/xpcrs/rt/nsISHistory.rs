//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/shistory/nsISHistory.idl
//


/// `interface nsISHistory : nsISupports`
///

/// ```text
/// /**
///  * An interface to the primary properties of the Session History
///  * component. In an embedded browser environment, the nsIWebBrowser
///  * object creates an instance of session history for each open window.
///  * A handle to the session history object can be obtained from
///  * nsIWebNavigation. In a non-embedded situation, the  owner of the
///  * session history component must create a instance of it and set
///  * it in the nsIWebNavigation object.
///  * This interface is accessible from javascript.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsISHistory {
    vtable: *const nsISHistoryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsISHistory.
unsafe impl XpCom for nsISHistory {
    const IID: nsIID = nsID(0x7b807041, 0xe60a, 0x4384,
        [0x93, 0x5f, 0xaf, 0x30, 0x61, 0xd8, 0xb8, 0x15]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsISHistory {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsISHistory.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsISHistoryCoerce {
    /// Cheaply cast a value of this type from a `nsISHistory`.
    fn coerce_from(v: &nsISHistory) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsISHistoryCoerce for nsISHistory {
    #[inline]
    fn coerce_from(v: &nsISHistory) -> &Self {
        v
    }
}

impl nsISHistory {
    /// Cast this `nsISHistory` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsISHistoryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsISHistory {
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
impl<T: nsISupportsCoerce> nsISHistoryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsISHistory) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsISHistory
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsISHistoryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [infallible] readonly attribute long count; */
    pub GetCount: unsafe extern "system" fn (this: *const nsISHistory, aCount: *mut i32) -> ::nserror::nsresult,

    /* attribute long index; */
    pub GetIndex: unsafe extern "system" fn (this: *const nsISHistory, aIndex: *mut i32) -> ::nserror::nsresult,

    /* attribute long index; */
    pub SetIndex: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32) -> ::nserror::nsresult,

    /* [infallible] readonly attribute long requestedIndex; */
    pub GetRequestedIndex: unsafe extern "system" fn (this: *const nsISHistory, aRequestedIndex: *mut i32) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void internalSetRequestedIndex (in long aRequestedIndex); */
    pub InternalSetRequestedIndex: unsafe extern "system" fn (this: *const nsISHistory, aRequestedIndex: i32) -> libc::c_void,

    /* nsISHEntry getEntryAtIndex (in long aIndex); */
    pub GetEntryAtIndex: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32, _retval: *mut*const nsISHEntry) -> ::nserror::nsresult,

    /* void purgeHistory (in long aNumEntries); */
    pub PurgeHistory: unsafe extern "system" fn (this: *const nsISHistory, aNumEntries: i32) -> ::nserror::nsresult,

    /* void addSHistoryListener (in nsISHistoryListener aListener); */
    pub AddSHistoryListener: unsafe extern "system" fn (this: *const nsISHistory, aListener: *const nsISHistoryListener) -> ::nserror::nsresult,

    /* void removeSHistoryListener (in nsISHistoryListener aListener); */
    pub RemoveSHistoryListener: unsafe extern "system" fn (this: *const nsISHistory, aListener: *const nsISHistoryListener) -> ::nserror::nsresult,

    /* void reloadCurrentEntry (); */
    pub ReloadCurrentEntry: unsafe extern "system" fn (this: *const nsISHistory) -> ::nserror::nsresult,

    /* [noscript] void gotoIndex (in long aIndex); */
    pub GotoIndex: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32) -> ::nserror::nsresult,

    /* [noscript,notxpcom] boolean hasUserInteractionAtIndex (in long aIndex); */
    pub HasUserInteractionAtIndex: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32) -> bool,

    /* [noscript,notxpcom] long getIndexOfEntry (in nsISHEntry aEntry); */
    pub GetIndexOfEntry: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsISHEntry) -> i32,

    /* void addEntry (in nsISHEntry aEntry, in boolean aPersist); */
    pub AddEntry: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsISHEntry, aPersist: bool) -> ::nserror::nsresult,

    /* void updateIndex (); */
    pub UpdateIndex: unsafe extern "system" fn (this: *const nsISHistory) -> ::nserror::nsresult,

    /* void replaceEntry (in long aIndex, in nsISHEntry aReplaceEntry); */
    pub ReplaceEntry: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32, aReplaceEntry: *const nsISHEntry) -> ::nserror::nsresult,

    /* boolean notifyOnHistoryReload (); */
    pub NotifyOnHistoryReload: unsafe extern "system" fn (this: *const nsISHistory, _retval: *mut bool) -> ::nserror::nsresult,

    /* void evictOutOfRangeContentViewers (in long aIndex); */
    pub EvictOutOfRangeContentViewers: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32) -> ::nserror::nsresult,

    /* void evictExpiredContentViewerForEntry (in nsIBFCacheEntry aEntry); */
    pub EvictExpiredContentViewerForEntry: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsIBFCacheEntry) -> ::nserror::nsresult,

    /* void evictAllContentViewers (); */
    pub EvictAllContentViewers: unsafe extern "system" fn (this: *const nsISHistory) -> ::nserror::nsresult,

    /* [noscript,notxpcom] void addToExpirationTracker (in nsIBFCacheEntry aEntry); */
    pub AddToExpirationTracker: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsIBFCacheEntry) -> libc::c_void,

    /* [noscript,notxpcom] void removeFromExpirationTracker (in nsIBFCacheEntry aEntry); */
    pub RemoveFromExpirationTracker: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsIBFCacheEntry) -> libc::c_void,

    /* [noscript,notxpcom] void RemoveDynEntries (in long aIndex, in nsISHEntry aEntry); */
    pub RemoveDynEntries: unsafe extern "system" fn (this: *const nsISHistory, aIndex: i32, aEntry: *const nsISHEntry) -> libc::c_void,

    /* [noscript,notxpcom] void RemoveDynEntriesForBFCacheEntry (in nsIBFCacheEntry aEntry); */
    pub RemoveDynEntriesForBFCacheEntry: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsIBFCacheEntry) -> libc::c_void,

    /* [noscript,notxpcom] void RemoveEntries (in nsDocshellIDArray aIDs, in long aStartIndex); */
    /// Unable to generate binding because `native type nsTArray<nsID> unsupported`
    pub RemoveEntries: *const ::libc::c_void,

    /* [noscript,notxpcom] void RemoveFrameEntries (in nsISHEntry aEntry); */
    pub RemoveFrameEntries: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsISHEntry) -> libc::c_void,

    /* [noscript] void Reload (in unsigned long aReloadFlags); */
    pub Reload: unsafe extern "system" fn (this: *const nsISHistory, aReloadFlags: u32) -> ::nserror::nsresult,

    /* [notxpcom] void EnsureCorrectEntryAtCurrIndex (in nsISHEntry aEntry); */
    pub EnsureCorrectEntryAtCurrIndex: unsafe extern "system" fn (this: *const nsISHistory, aEntry: *const nsISHEntry) -> libc::c_void,

    /* [notxpcom] void EvictContentViewersOrReplaceEntry (in nsISHEntry aNewSHEntry, in bool aReplace); */
    pub EvictContentViewersOrReplaceEntry: unsafe extern "system" fn (this: *const nsISHistory, aNewSHEntry: *const nsISHEntry, aReplace: bool) -> libc::c_void,

    /* nsISHEntry createEntry (); */
    pub CreateEntry: unsafe extern "system" fn (this: *const nsISHistory, _retval: *mut*const nsISHEntry) -> ::nserror::nsresult,

    /* [noscript] void AddToRootSessionHistory (in bool aCloneChildren, in nsISHEntry aOSHE, in BrowsingContext aRootBC, in nsISHEntry aEntry, in unsigned long aLoadType, in bool aShouldPersist, out MaybeInt32 aPreviousEntryIndex, out MaybeInt32 aLoadedEntryIndex); */
    /// Unable to generate binding because `Rust only supports [ref] / [ptr] native types`
    pub AddToRootSessionHistory: *const ::libc::c_void,

    /* [noscript] void AddChildSHEntryHelper (in nsISHEntry aCloneRef, in nsISHEntry aNewEntry, in BrowsingContext aRootBC, in bool aCloneChildren); */
    pub AddChildSHEntryHelper: unsafe extern "system" fn (this: *const nsISHistory, aCloneRef: *const nsISHEntry, aNewEntry: *const nsISHEntry, aRootBC: *const libc::c_void, aCloneChildren: bool) -> ::nserror::nsresult,

    /* [noscript,notxpcom] boolean isEmptyOrHasEntriesForSingleTopLevelPage (); */
    pub IsEmptyOrHasEntriesForSingleTopLevelPage: unsafe extern "system" fn (this: *const nsISHistory) -> bool,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsISHistory {

    /// ```text
    /// /**
    ///    * A readonly property of the interface that returns
    ///    * the number of toplevel documents currently available
    ///    * in session history.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute long count;`
    #[inline]
    pub unsafe fn GetCount(&self) -> i32 {
        let mut result = <i32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetCount)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * The index of the current document in session history. Not infallible
    ///    * because setting can fail if the assigned value is out of range.
    ///    */
    /// ```
    ///

    /// `attribute long index;`
    #[inline]
    pub unsafe fn GetIndex(&self, aIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetIndex)(self, aIndex)
    }


    /// ```text
    /// /**
    ///    * The index of the current document in session history. Not infallible
    ///    * because setting can fail if the assigned value is out of range.
    ///    */
    /// ```
    ///

    /// `attribute long index;`
    #[inline]
    pub unsafe fn SetIndex(&self, aIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetIndex)(self, aIndex)
    }


    /// ```text
    /// /**
    ///    * A readonly property of the interface that returns
    ///    * the index of the last document that started to load and
    ///    * didn't finished yet. When document finishes the loading
    ///    * value -1 is returned.
    ///    */
    /// ```
    ///

    /// `[infallible] readonly attribute long requestedIndex;`
    #[inline]
    pub unsafe fn GetRequestedIndex(&self) -> i32 {
        let mut result = <i32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetRequestedIndex)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }


    /// ```text
    /// /**
    ///    * Artifically set the |requestedIndex| for this nsISHEntry to the given
    ///    * index. This is used when resuming a cross-process load from a different
    ///    * process.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void internalSetRequestedIndex (in long aRequestedIndex);`
    #[inline]
    pub unsafe fn InternalSetRequestedIndex(&self, aRequestedIndex: i32) -> libc::c_void {
        ((*self.vtable).InternalSetRequestedIndex)(self, aRequestedIndex)
    }


    /// ```text
    /// /**
    ///    * Get the history entry at a given index. Returns non-null on success.
    ///    *
    ///    * @param index             The index value whose entry is requested.
    ///    *                          The oldest entry is located at index == 0.
    ///    * @return                  The found entry; never null.
    ///    */
    /// ```
    ///

    /// `nsISHEntry getEntryAtIndex (in long aIndex);`
    #[inline]
    pub unsafe fn GetEntryAtIndex(&self, aIndex: i32, _retval: *mut*const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).GetEntryAtIndex)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Called to purge older documents from history.
    ///    * Documents can be removed from session history for various
    ///    * reasons. For example to  control memory usage of the browser, to
    ///    * prevent users from loading documents from history, to erase evidence of
    ///    * prior page loads etc...
    ///    *
    ///    * @param numEntries        The number of toplevel documents to be
    ///    *                          purged from history. During purge operation,
    ///    *                          the latest documents are maintained and older
    ///    *                          'numEntries' documents are removed from history.
    ///    * @throws                  <code>NS_SUCCESS_LOSS_OF_INSIGNIFICANT_DATA</code>
    ///    *                          Purge was vetod.
    ///    * @throws                  <code>NS_ERROR_FAILURE</code> numEntries is
    ///    *                          invalid or out of bounds with the size of history.
    ///    */
    /// ```
    ///

    /// `void purgeHistory (in long aNumEntries);`
    #[inline]
    pub unsafe fn PurgeHistory(&self, aNumEntries: i32) -> ::nserror::nsresult {
        ((*self.vtable).PurgeHistory)(self, aNumEntries)
    }


    /// ```text
    /// /**
    ///    * Called to register a listener for the session history component.
    ///    * Listeners are notified when pages are loaded or purged from history.
    ///    *
    ///    * @param aListener         Listener object to be notified for all
    ///    *                          page loads that initiate in session history.
    ///    *
    ///    * @note                    A listener object must implement
    ///    *                          nsISHistoryListener and nsSupportsWeakReference
    ///    *
    ///    * @see nsISHistoryListener
    ///    * @see nsSupportsWeakReference
    ///    */
    /// ```
    ///

    /// `void addSHistoryListener (in nsISHistoryListener aListener);`
    #[inline]
    pub unsafe fn AddSHistoryListener(&self, aListener: *const nsISHistoryListener) -> ::nserror::nsresult {
        ((*self.vtable).AddSHistoryListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * Called to remove a listener for the session history component.
    ///    * Listeners are notified when pages are loaded from history.
    ///    *
    ///    * @param aListener         Listener object to be removed from
    ///    *                          session history.
    ///    *
    ///    * @note                    A listener object must implement
    ///    *                          nsISHistoryListener and nsSupportsWeakReference
    ///    * @see nsISHistoryListener
    ///    * @see nsSupportsWeakReference
    ///    */
    /// ```
    ///

    /// `void removeSHistoryListener (in nsISHistoryListener aListener);`
    #[inline]
    pub unsafe fn RemoveSHistoryListener(&self, aListener: *const nsISHistoryListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveSHistoryListener)(self, aListener)
    }



    /// `void reloadCurrentEntry ();`
    #[inline]
    pub unsafe fn ReloadCurrentEntry(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ReloadCurrentEntry)(self, )
    }


    /// ```text
    /// /**
    ///    * Load the entry at the particular index.
    ///    */
    /// ```
    ///

    /// `[noscript] void gotoIndex (in long aIndex);`
    #[inline]
    pub unsafe fn GotoIndex(&self, aIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).GotoIndex)(self, aIndex)
    }


    /// ```text
    /// /**
    ///    * If an element exists at the particular index and
    ///    * whether it has user interaction.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] boolean hasUserInteractionAtIndex (in long aIndex);`
    #[inline]
    pub unsafe fn HasUserInteractionAtIndex(&self, aIndex: i32) -> bool {
        ((*self.vtable).HasUserInteractionAtIndex)(self, aIndex)
    }


    /// ```text
    /// /**
    ///    * Called to obtain the index to a given history entry.
    ///    *
    ///    * @param aEntry            The entry to obtain the index of.
    ///    *
    ///    * @return                  <code>NS_OK</code> index for the history entry
    ///    *                          is obtained successfully.
    ///    *                          <code>NS_ERROR_FAILURE</code> Error in obtaining
    ///    *                          index for the given history entry.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] long getIndexOfEntry (in nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn GetIndexOfEntry(&self, aEntry: *const nsISHEntry) -> i32 {
        ((*self.vtable).GetIndexOfEntry)(self, aEntry)
    }


    /// ```text
    /// /**
    ///    * Add a new Entry to the History List.
    ///    *
    ///    * @param aEntry            The entry to add.
    ///    * @param aPersist          If true this specifies that the entry should
    ///    *                          persist in the list. If false, this means that
    ///    *                          when new entries are added this element will not
    ///    *                          appear in the session history list.
    ///    */
    /// ```
    ///

    /// `void addEntry (in nsISHEntry aEntry, in boolean aPersist);`
    #[inline]
    pub unsafe fn AddEntry(&self, aEntry: *const nsISHEntry, aPersist: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddEntry)(self, aEntry, aPersist)
    }


    /// ```text
    /// /**
    ///    * Update the index maintained by sessionHistory
    ///    */
    /// ```
    ///

    /// `void updateIndex ();`
    #[inline]
    pub unsafe fn UpdateIndex(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UpdateIndex)(self, )
    }


    /// ```text
    /// /**
    ///    * Replace the nsISHEntry at a particular index
    ///    *
    ///    * @param aIndex            The index at which the entry should be replaced.
    ///    * @param aReplaceEntry     The replacement entry for the index.
    ///    */
    /// ```
    ///

    /// `void replaceEntry (in long aIndex, in nsISHEntry aReplaceEntry);`
    #[inline]
    pub unsafe fn ReplaceEntry(&self, aIndex: i32, aReplaceEntry: *const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).ReplaceEntry)(self, aIndex, aReplaceEntry)
    }


    /// ```text
    /// /**
    ///    * Notifies all registered session history listeners about an impending
    ///    * reload.
    ///    *
    ///    * @return                  Whether the operation can proceed.
    ///    */
    /// ```
    ///

    /// `boolean notifyOnHistoryReload ();`
    #[inline]
    pub unsafe fn NotifyOnHistoryReload(&self, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).NotifyOnHistoryReload)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Evict content viewers which don't lie in the "safe" range around aIndex.
    ///    * In practice, this should leave us with no more than gHistoryMaxViewers
    ///    * viewers associated with this SHistory object.
    ///    *
    ///    * Also make sure that the total number of content viewers in all windows is
    ///    * not greater than our global max; if it is, evict viewers as appropriate.
    ///    *
    ///    * @param aIndex           The index around which the "safe" range is
    ///    *                         centered.  In general, if you just navigated the
    ///    *                         history, aIndex should be the index history was
    ///    *                         navigated to.
    ///    */
    /// ```
    ///

    /// `void evictOutOfRangeContentViewers (in long aIndex);`
    #[inline]
    pub unsafe fn EvictOutOfRangeContentViewers(&self, aIndex: i32) -> ::nserror::nsresult {
        ((*self.vtable).EvictOutOfRangeContentViewers)(self, aIndex)
    }


    /// ```text
    /// /**
    ///    * Evict the content viewer associated with a bfcache entry that has timed
    ///    * out.
    ///    */
    /// ```
    ///

    /// `void evictExpiredContentViewerForEntry (in nsIBFCacheEntry aEntry);`
    #[inline]
    pub unsafe fn EvictExpiredContentViewerForEntry(&self, aEntry: *const nsIBFCacheEntry) -> ::nserror::nsresult {
        ((*self.vtable).EvictExpiredContentViewerForEntry)(self, aEntry)
    }


    /// ```text
    /// /**
    ///    * Evict all the content viewers in this session history
    ///    */
    /// ```
    ///

    /// `void evictAllContentViewers ();`
    #[inline]
    pub unsafe fn EvictAllContentViewers(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).EvictAllContentViewers)(self, )
    }


    /// ```text
    /// /**
    ///    * Add a BFCache entry to expiration tracker so it gets evicted on
    ///    * expiration.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void addToExpirationTracker (in nsIBFCacheEntry aEntry);`
    #[inline]
    pub unsafe fn AddToExpirationTracker(&self, aEntry: *const nsIBFCacheEntry) -> libc::c_void {
        ((*self.vtable).AddToExpirationTracker)(self, aEntry)
    }


    /// ```text
    /// /**
    ///    * Remove a BFCache entry from expiration tracker.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void removeFromExpirationTracker (in nsIBFCacheEntry aEntry);`
    #[inline]
    pub unsafe fn RemoveFromExpirationTracker(&self, aEntry: *const nsIBFCacheEntry) -> libc::c_void {
        ((*self.vtable).RemoveFromExpirationTracker)(self, aEntry)
    }


    /// ```text
    /// /**
    ///    * Remove dynamic entries found at given index.
    ///    *
    ///    * @param aIndex           Index to remove dynamic entries from. It will be
    ///    *                         passed to RemoveEntries as aStartIndex.
    ///    * @param aEntry (optional)  The entry to start looking in for dynamic
    ///    *                         entries. Only the dynamic descendants of the
    ///    *                         entry will be removed. If not given, all dynamic
    ///    *                         entries at the index will be removed.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void RemoveDynEntries (in long aIndex, in nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn RemoveDynEntries(&self, aIndex: i32, aEntry: *const nsISHEntry) -> libc::c_void {
        ((*self.vtable).RemoveDynEntries)(self, aIndex, aEntry)
    }


    /// ```text
    /// /**
    ///    * Similar to RemoveDynEntries, but instead of specifying an index, use the
    ///    * given BFCacheEntry to find the index and remove dynamic entries from the
    ///    * index.
    ///    *
    ///    * The method takes no effect if the bfcache entry is not or no longer hold
    ///    * by the SHistory instance.
    ///    *
    ///    * @param aEntry           The bfcache entry to look up for index to remove
    ///    *                         dynamic entries from.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void RemoveDynEntriesForBFCacheEntry (in nsIBFCacheEntry aEntry);`
    #[inline]
    pub unsafe fn RemoveDynEntriesForBFCacheEntry(&self, aEntry: *const nsIBFCacheEntry) -> libc::c_void {
        ((*self.vtable).RemoveDynEntriesForBFCacheEntry)(self, aEntry)
    }


    /// ```text
    /// /**
    ///    * Removes entries from the history if their docshellID is in
    ///    * aIDs array.
    ///    */
    /// ```
    ///

    /// `[noscript,notxpcom] void RemoveEntries (in nsDocshellIDArray aIDs, in long aStartIndex);`
    const _RemoveEntries: () = ();

    /// ```text
    /// /**
    ///    * Collect docshellIDs from aEntry's children and remove those
    ///    * entries from history.
    ///    *
    ///    * @param aEntry           Children docshellID's will be collected from
    ///    *                         this entry and passed to RemoveEntries as aIDs.
    ///   */
    /// ```
    ///

    /// `[noscript,notxpcom] void RemoveFrameEntries (in nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn RemoveFrameEntries(&self, aEntry: *const nsISHEntry) -> libc::c_void {
        ((*self.vtable).RemoveFrameEntries)(self, aEntry)
    }



    /// `[noscript] void Reload (in unsigned long aReloadFlags);`
    #[inline]
    pub unsafe fn Reload(&self, aReloadFlags: u32) -> ::nserror::nsresult {
        ((*self.vtable).Reload)(self, aReloadFlags)
    }



    /// `[notxpcom] void EnsureCorrectEntryAtCurrIndex (in nsISHEntry aEntry);`
    #[inline]
    pub unsafe fn EnsureCorrectEntryAtCurrIndex(&self, aEntry: *const nsISHEntry) -> libc::c_void {
        ((*self.vtable).EnsureCorrectEntryAtCurrIndex)(self, aEntry)
    }



    /// `[notxpcom] void EvictContentViewersOrReplaceEntry (in nsISHEntry aNewSHEntry, in bool aReplace);`
    #[inline]
    pub unsafe fn EvictContentViewersOrReplaceEntry(&self, aNewSHEntry: *const nsISHEntry, aReplace: bool) -> libc::c_void {
        ((*self.vtable).EvictContentViewersOrReplaceEntry)(self, aNewSHEntry, aReplace)
    }



    /// `nsISHEntry createEntry ();`
    #[inline]
    pub unsafe fn CreateEntry(&self, _retval: *mut*const nsISHEntry) -> ::nserror::nsresult {
        ((*self.vtable).CreateEntry)(self, _retval)
    }



    /// `[noscript] void AddToRootSessionHistory (in bool aCloneChildren, in nsISHEntry aOSHE, in BrowsingContext aRootBC, in nsISHEntry aEntry, in unsigned long aLoadType, in bool aShouldPersist, out MaybeInt32 aPreviousEntryIndex, out MaybeInt32 aLoadedEntryIndex);`
    const _AddToRootSessionHistory: () = ();


    /// `[noscript] void AddChildSHEntryHelper (in nsISHEntry aCloneRef, in nsISHEntry aNewEntry, in BrowsingContext aRootBC, in bool aCloneChildren);`
    #[inline]
    pub unsafe fn AddChildSHEntryHelper(&self, aCloneRef: *const nsISHEntry, aNewEntry: *const nsISHEntry, aRootBC: *const libc::c_void, aCloneChildren: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddChildSHEntryHelper)(self, aCloneRef, aNewEntry, aRootBC, aCloneChildren)
    }



    /// `[noscript,notxpcom] boolean isEmptyOrHasEntriesForSingleTopLevelPage ();`
    #[inline]
    pub unsafe fn IsEmptyOrHasEntriesForSingleTopLevelPage(&self, ) -> bool {
        ((*self.vtable).IsEmptyOrHasEntriesForSingleTopLevelPage)(self, )
    }


}


