//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsINavBookmarksService.idl
//


/// `interface nsINavBookmarkObserver : nsISupports`
///

/// ```text
/// /**
///  * Observer for bookmarks changes.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavBookmarkObserver {
    vtable: *const nsINavBookmarkObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavBookmarkObserver.
unsafe impl XpCom for nsINavBookmarkObserver {
    const IID: nsIID = nsID(0x4d00c221, 0x2c4a, 0x47ab,
        [0xa6, 0x17, 0xab, 0xb3, 0x24, 0x11, 0x04, 0x92]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavBookmarkObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavBookmarkObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavBookmarkObserverCoerce {
    /// Cheaply cast a value of this type from a `nsINavBookmarkObserver`.
    fn coerce_from(v: &nsINavBookmarkObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavBookmarkObserverCoerce for nsINavBookmarkObserver {
    #[inline]
    fn coerce_from(v: &nsINavBookmarkObserver) -> &Self {
        v
    }
}

impl nsINavBookmarkObserver {
    /// Cast this `nsINavBookmarkObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavBookmarkObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavBookmarkObserver {
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
impl<T: nsISupportsCoerce> nsINavBookmarkObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavBookmarkObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavBookmarkObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavBookmarkObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean skipTags; */
    pub GetSkipTags: unsafe extern "system" fn (this: *const nsINavBookmarkObserver, aSkipTags: *mut bool) -> ::nserror::nsresult,

    /* void onBeginUpdateBatch (); */
    pub OnBeginUpdateBatch: unsafe extern "system" fn (this: *const nsINavBookmarkObserver) -> ::nserror::nsresult,

    /* void onEndUpdateBatch (); */
    pub OnEndUpdateBatch: unsafe extern "system" fn (this: *const nsINavBookmarkObserver) -> ::nserror::nsresult,

    /* void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource); */
    pub OnItemChanged: unsafe extern "system" fn (this: *const nsINavBookmarkObserver, aItemId: i64, aProperty: *const ::nsstring::nsACString, aIsAnnotationProperty: bool, aNewValue: *const ::nsstring::nsACString, aLastModified: PRTime, aItemType: u16, aParentId: i64, aGuid: *const ::nsstring::nsACString, aParentGuid: *const ::nsstring::nsACString, aOldValue: *const ::nsstring::nsACString, aSource: u16) -> ::nserror::nsresult,

    /* void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource, in AUTF8String aURI); */
    pub OnItemMoved: unsafe extern "system" fn (this: *const nsINavBookmarkObserver, aItemId: i64, aOldParentId: i64, aOldIndex: i32, aNewParentId: i64, aNewIndex: i32, aItemType: u16, aGuid: *const ::nsstring::nsACString, aOldParentGuid: *const ::nsstring::nsACString, aNewParentGuid: *const ::nsstring::nsACString, aSource: u16, aURI: *const ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavBookmarkObserver {


    /// `readonly attribute boolean skipTags;`
    #[inline]
    pub unsafe fn GetSkipTags(&self, aSkipTags: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSkipTags)(self, aSkipTags)
    }


    /// ```text
    /// /**
    ///    * Notifies that a batch transaction has started.
    ///    * Other notifications will be sent during the batch, but the observer is
    ///    * guaranteed that onEndUpdateBatch() will be called at its completion.
    ///    * During a batch the observer should do its best to reduce the work done to
    ///    * handle notifications, since multiple changes are going to happen in a short
    ///    * timeframe.
    ///    */
    /// ```
    ///

    /// `void onBeginUpdateBatch ();`
    #[inline]
    pub unsafe fn OnBeginUpdateBatch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnBeginUpdateBatch)(self, )
    }


    /// ```text
    /// /**
    ///    * Notifies that a batch transaction has ended.
    ///    */
    /// ```
    ///

    /// `void onEndUpdateBatch ();`
    #[inline]
    pub unsafe fn OnEndUpdateBatch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnEndUpdateBatch)(self, )
    }


    /// ```text
    /// /**
    ///    * Notifies that an item's information has changed.  This will be called
    ///    * whenever any attributes like "title" are changed.
    ///    *
    ///    * @param aItemId
    ///    *        The id of the item that was changed.
    ///    * @param aProperty
    ///    *        The property which changed.
    ///    * @param aIsAnnotationProperty
    ///    *        Obsolete and unused.
    ///    * @param aNewValue
    ///    *        For certain properties, this is set to the new value of the
    ///    *        property (see the list below).
    ///    * @param aLastModified
    ///    *        The updated last-modified value.
    ///    * @param aItemType
    ///    *        The type of the item to be removed (see TYPE_* constants below).
    ///    * @param aParentId
    ///    *        The id of the folder containing the item.
    ///    * @param aGuid
    ///    *        The unique ID associated with the item.
    ///    * @param aParentGuid
    ///    *        The unique ID associated with the item's parent.
    ///    * @param aOldValue
    ///    *        For certain properties, this is set to the new value of the
    ///    *        property (see the list below).
    ///    * @param aSource
    ///    *        A change source constant from nsINavBookmarksService::SOURCE_*,
    ///    *        passed to the method that notifies the observer.
    ///    *
    ///    * @note List of values that may be associated with properties:
    ///    *       aProperty     | aNewValue
    ///    *       =====================================================================
    ///    *       guid          | The new bookmark guid.
    ///    *       cleartime     | Empty string (all visits to this item were removed).
    ///    *       title         | The new title.
    ///    *       uri           | new URL.
    ///    *       tags          | Empty string (tags for this item changed)
    ///    *       dateAdded     | PRTime (as string) when the item was first added.
    ///    *       lastModified  | PRTime (as string) when the item was last modified.
    ///    *
    ///    *       aProperty     | aOldValue
    ///    *       =====================================================================
    ///    *       guid          | The old bookmark guid.
    ///    *       cleartime     | Empty string (currently unused).
    ///    *       title         | Empty string (currently unused).
    ///    *       uri           | old URL.
    ///    *       tags          | Empty string (currently unused).
    ///    *       dateAdded     | Empty string (currently unused).
    ///    *       lastModified  | Empty string (currently unused).
    ///    */
    /// ```
    ///

    /// `void onItemChanged (in long long aItemId, in ACString aProperty, in boolean aIsAnnotationProperty, in AUTF8String aNewValue, in PRTime aLastModified, in unsigned short aItemType, in long long aParentId, in ACString aGuid, in ACString aParentGuid, in AUTF8String aOldValue, in unsigned short aSource);`
    #[inline]
    pub unsafe fn OnItemChanged(&self, aItemId: i64, aProperty: *const ::nsstring::nsACString, aIsAnnotationProperty: bool, aNewValue: *const ::nsstring::nsACString, aLastModified: PRTime, aItemType: u16, aParentId: i64, aGuid: *const ::nsstring::nsACString, aParentGuid: *const ::nsstring::nsACString, aOldValue: *const ::nsstring::nsACString, aSource: u16) -> ::nserror::nsresult {
        ((*self.vtable).OnItemChanged)(self, aItemId, aProperty, aIsAnnotationProperty, aNewValue, aLastModified, aItemType, aParentId, aGuid, aParentGuid, aOldValue, aSource)
    }


    /// ```text
    /// /**
    ///    * Notifies that an item has been moved.
    ///    *
    ///    * @param aItemId
    ///    *        The id of the item that was moved.
    ///    * @param aOldParentId
    ///    *        The id of the old parent.
    ///    * @param aOldIndex
    ///    *        The old index inside the old parent.
    ///    * @param aNewParentId
    ///    *        The id of the new parent.
    ///    * @param aNewIndex
    ///    *        The index inside the new parent.
    ///    * @param aItemType
    ///    *        The type of the item to be removed (see TYPE_* constants below).
    ///    * @param aGuid
    ///    *        The unique ID associated with the item.
    ///    * @param aOldParentGuid
    ///    *        The unique ID associated with the old item's parent.
    ///    * @param aNewParentGuid
    ///    *        The unique ID associated with the new item's parent.
    ///    * @param aSource
    ///    *        A change source constant from nsINavBookmarksService::SOURCE_*,
    ///    *        passed to the method that notifies the observer.
    ///    * @param aURI
    ///    *        The URI for this bookmark.
    ///    */
    /// ```
    ///

    /// `void onItemMoved (in long long aItemId, in long long aOldParentId, in long aOldIndex, in long long aNewParentId, in long aNewIndex, in unsigned short aItemType, in ACString aGuid, in ACString aOldParentGuid, in ACString aNewParentGuid, in unsigned short aSource, in AUTF8String aURI);`
    #[inline]
    pub unsafe fn OnItemMoved(&self, aItemId: i64, aOldParentId: i64, aOldIndex: i32, aNewParentId: i64, aNewIndex: i32, aItemType: u16, aGuid: *const ::nsstring::nsACString, aOldParentGuid: *const ::nsstring::nsACString, aNewParentGuid: *const ::nsstring::nsACString, aSource: u16, aURI: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).OnItemMoved)(self, aItemId, aOldParentId, aOldIndex, aNewParentId, aNewIndex, aItemType, aGuid, aOldParentGuid, aNewParentGuid, aSource, aURI)
    }


}


/// `interface nsINavBookmarksService : nsISupports`
///

/// ```text
/// /**
///  * The BookmarksService interface provides methods for managing bookmarked
///  * history items.  Bookmarks consist of a set of user-customizable
///  * folders.  A URI in history can be contained in one or more such folders.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavBookmarksService {
    vtable: *const nsINavBookmarksServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavBookmarksService.
unsafe impl XpCom for nsINavBookmarksService {
    const IID: nsIID = nsID(0x24533891, 0xafa6, 0x4663,
        [0xb7, 0x2d, 0x31, 0x43, 0xd0, 0x3f, 0x1b, 0x04]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavBookmarksService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavBookmarksService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavBookmarksServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINavBookmarksService`.
    fn coerce_from(v: &nsINavBookmarksService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavBookmarksServiceCoerce for nsINavBookmarksService {
    #[inline]
    fn coerce_from(v: &nsINavBookmarksService) -> &Self {
        v
    }
}

impl nsINavBookmarksService {
    /// Cast this `nsINavBookmarksService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavBookmarksServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavBookmarksService {
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
impl<T: nsISupportsCoerce> nsINavBookmarksServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavBookmarksService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavBookmarksService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavBookmarksServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute long long placesRoot; */
    pub GetPlacesRoot: unsafe extern "system" fn (this: *const nsINavBookmarksService, aPlacesRoot: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long bookmarksMenuFolder; */
    pub GetBookmarksMenuFolder: unsafe extern "system" fn (this: *const nsINavBookmarksService, aBookmarksMenuFolder: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long tagsFolder; */
    pub GetTagsFolder: unsafe extern "system" fn (this: *const nsINavBookmarksService, aTagsFolder: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long toolbarFolder; */
    pub GetToolbarFolder: unsafe extern "system" fn (this: *const nsINavBookmarksService, aToolbarFolder: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long totalSyncChanges; */
    pub GetTotalSyncChanges: unsafe extern "system" fn (this: *const nsINavBookmarksService, aTotalSyncChanges: *mut i64) -> ::nserror::nsresult,

    /* [can_run_script] long long insertBookmark (in long long aParentId, in nsIURI aURI, in long aIndex, in AUTF8String aTitle, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    pub InsertBookmark: unsafe extern "system" fn (this: *const nsINavBookmarksService, aParentId: i64, aURI: *const nsIURI, aIndex: i32, aTitle: *const ::nsstring::nsACString, aGuid: *const ::nsstring::nsACString, aSource: u16, _retval: *mut i64) -> ::nserror::nsresult,

    /* [can_run_script] void removeItem (in long long aItemId, [optional] in unsigned short aSource); */
    pub RemoveItem: unsafe extern "system" fn (this: *const nsINavBookmarksService, aItemId: i64, aSource: u16) -> ::nserror::nsresult,

    /* [can_run_script] long long createFolder (in long long aParentFolder, in AUTF8String name, in long index, [optional] in ACString aGuid, [optional] in unsigned short aSource); */
    pub CreateFolder: unsafe extern "system" fn (this: *const nsINavBookmarksService, aParentFolder: i64, name: *const ::nsstring::nsACString, index: i32, aGuid: *const ::nsstring::nsACString, aSource: u16, _retval: *mut i64) -> ::nserror::nsresult,

    /* void setItemTitle (in long long aItemId, in AUTF8String aTitle, [optional] in unsigned short aSource); */
    pub SetItemTitle: unsafe extern "system" fn (this: *const nsINavBookmarksService, aItemId: i64, aTitle: *const ::nsstring::nsACString, aSource: u16) -> ::nserror::nsresult,

    /* AUTF8String getItemTitle (in long long aItemId); */
    pub GetItemTitle: unsafe extern "system" fn (this: *const nsINavBookmarksService, aItemId: i64, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void setItemLastModified (in long long aItemId, in PRTime aLastModified, [optional] in unsigned short aSource); */
    pub SetItemLastModified: unsafe extern "system" fn (this: *const nsINavBookmarksService, aItemId: i64, aLastModified: PRTime, aSource: u16) -> ::nserror::nsresult,

    /* long long getFolderIdForItem (in long long aItemId); */
    pub GetFolderIdForItem: unsafe extern "system" fn (this: *const nsINavBookmarksService, aItemId: i64, _retval: *mut i64) -> ::nserror::nsresult,

    /* void addObserver (in nsINavBookmarkObserver observer, [optional] in boolean ownsWeak); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsINavBookmarksService, observer: *const nsINavBookmarkObserver, ownsWeak: bool) -> ::nserror::nsresult,

    /* void removeObserver (in nsINavBookmarkObserver observer); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsINavBookmarksService, observer: *const nsINavBookmarkObserver) -> ::nserror::nsresult,

    /* Array<nsINavBookmarkObserver> getObservers (); */
    pub GetObservers: unsafe extern "system" fn (this: *const nsINavBookmarksService, _retval: *mut thin_vec::ThinVec<RefPtr<nsINavBookmarkObserver>>) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavBookmarksService {
    /// ```text
    /// /**
    ///    * This value should be used for APIs that allow passing in an index
    ///    * where an index is not known, or not required to be specified.
    ///    * e.g.: When appending an item to a folder.
    ///    */
    /// ```
    ///

    pub const DEFAULT_INDEX: i64 = -1;


    pub const TYPE_BOOKMARK: i64 = 1;


    pub const TYPE_FOLDER: i64 = 2;


    pub const TYPE_SEPARATOR: i64 = 3;


    pub const TYPE_DYNAMIC_CONTAINER: i64 = 4;


    pub const SOURCE_DEFAULT: i64 = 0;


    pub const SOURCE_SYNC: i64 = 1;


    pub const SOURCE_IMPORT: i64 = 2;


    pub const SOURCE_SYNC_REPARENT_REMOVED_FOLDER_CHILDREN: i64 = 4;


    pub const SOURCE_RESTORE: i64 = 5;


    pub const SOURCE_RESTORE_ON_STARTUP: i64 = 6;

    /// ```text
    /// /**
    ///    * Sync status flags, stored in Places for each item. These affect conflict
    ///    * resolution, when an item is changed both locally and remotely; deduping,
    ///    * when a local item matches a remote item with similar contents and different
    ///    * GUIDs; and whether we write a tombstone when an item is deleted locally.
    ///    *
    ///    * Status  | Description               | Conflict   | Can     | Needs
    ///    *         |                           | resolution | dedupe? | tombstone?
    ///    * -----------------------------------------------------------------------
    ///    * UNKNOWN | Automatically restored    | Prefer     | No      | No
    ///    *         | on startup to recover     | deletion   |         |
    ///    *         | from database corruption, |            |         |
    ///    *         | or sync ID change on      |            |         |
    ///    *         | server.                   |            |         |
    ///    * -----------------------------------------------------------------------
    ///    * NEW     | Item not uploaded to      | Prefer     | Yes     | No
    ///    *         | server yet, or Sync       | newer      |         |
    ///    *         | disconnected.             |            |         |
    ///    * -----------------------------------------------------------------------
    ///    * NORMAL  | Item uploaded to server.  | Prefer     | No      | Yes
    ///    *         |                           | newer      |         |
    ///    */
    /// ```
    ///

    pub const SYNC_STATUS_UNKNOWN: i64 = 0;


    pub const SYNC_STATUS_NEW: i64 = 1;


    pub const SYNC_STATUS_NORMAL: i64 = 2;

    /// ```text
    /// /**
    ///    * The item ID of the Places root.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long placesRoot;`
    #[inline]
    pub unsafe fn GetPlacesRoot(&self, aPlacesRoot: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetPlacesRoot)(self, aPlacesRoot)
    }


    /// ```text
    /// /**
    ///    * The item ID of the bookmarks menu folder.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long bookmarksMenuFolder;`
    #[inline]
    pub unsafe fn GetBookmarksMenuFolder(&self, aBookmarksMenuFolder: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetBookmarksMenuFolder)(self, aBookmarksMenuFolder)
    }


    /// ```text
    /// /**
    ///    * The item ID of the top-level folder that contain the tag "folders".
    ///    */
    /// ```
    ///

    /// `readonly attribute long long tagsFolder;`
    #[inline]
    pub unsafe fn GetTagsFolder(&self, aTagsFolder: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetTagsFolder)(self, aTagsFolder)
    }


    /// ```text
    /// /**
    ///    * The item ID of the personal toolbar folder.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long toolbarFolder;`
    #[inline]
    pub unsafe fn GetToolbarFolder(&self, aToolbarFolder: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetToolbarFolder)(self, aToolbarFolder)
    }


    /// ```text
    /// /**
    ///    * The total number of Sync changes (inserts, updates, deletes, merges, and
        ///    * uploads) recorded since Places startup for all bookmarks.
    ///    *
    ///    * Note that this is *not* the number of bookmark syncs. It's a monotonically
    ///    * increasing counter incremented for every change that affects a bookmark's
    ///    * `syncChangeCounter`.
    ///    *
    ///    * The counter can be used to avoid keeping an exclusive transaction open for
    ///    * time-consuming work. One way to do that is to store the current value of
    ///    * the counter, do the work, start a transaction, check the current value
    ///    * again, and compare it to the stored value to determine if the database
    ///    * changed during the work.
    ///    *
    ///    * The bookmarks mirror does this to check for changes between building and
    ///    * applying a merged tree. This avoids blocking the main Places connection
    ///    * during the merge, and ensures that the new tree still applies cleanly.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long totalSyncChanges;`
    #[inline]
    pub unsafe fn GetTotalSyncChanges(&self, aTotalSyncChanges: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetTotalSyncChanges)(self, aTotalSyncChanges)
    }


    /// ```text
    /// /**
    ///    * Inserts a child bookmark into the given folder.
    ///    *
    ///    *  @param aParentId
    ///    *         The id of the parent folder
    ///    *  @param aURI
    ///    *         The URI to insert
    ///    *  @param aIndex
    ///    *         The index to insert at, or DEFAULT_INDEX to append
    ///    *  @param aTitle
    ///    *         The title for the new bookmark
    ///    *  @param [optional] aGuid
    ///    *         The GUID to be set for the new item.  If not set, a new GUID is
    ///    *         generated.  Unless you've a very sound reason, such as an undo
    ///    *         manager implementation, do not pass this argument.
    ///    *  @param [optional] aSource
    ///    *         The change source. This is forwarded to all bookmark observers,
    ///    *         allowing them to distinguish between insertions from different
    ///    *         callers. Defaults to SOURCE_DEFAULT if omitted.
    ///    *  @return The ID of the newly-created bookmark.
    ///    *
    ///    *  @note aTitle will be truncated to TITLE_LENGTH_MAX and
    ///    *        aURI will be truncated to URI_LENGTH_MAX.
    ///    *  @throws if aGuid is malformed.
    ///    */
    /// ```
    ///

    /// `[can_run_script] long long insertBookmark (in long long aParentId, in nsIURI aURI, in long aIndex, in AUTF8String aTitle, [optional] in ACString aGuid, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn InsertBookmark(&self, aParentId: i64, aURI: *const nsIURI, aIndex: i32, aTitle: *const ::nsstring::nsACString, aGuid: *const ::nsstring::nsACString, aSource: u16, _retval: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).InsertBookmark)(self, aParentId, aURI, aIndex, aTitle, aGuid, aSource, _retval)
    }


    /// ```text
    /// /**
    ///    * Removes a child item. Used to delete a bookmark or separator.
    ///    *  @param aItemId
    ///    *         The child item to remove
    ///    *  @param [optional] aSource
    ///    *         The change source, forwarded to all bookmark observers. Defaults
    ///    *         to SOURCE_DEFAULT.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void removeItem (in long long aItemId, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn RemoveItem(&self, aItemId: i64, aSource: u16) -> ::nserror::nsresult {
        ((*self.vtable).RemoveItem)(self, aItemId, aSource)
    }


    /// ```text
    /// /**
    ///    * Creates a new child folder and inserts it under the given parent.
    ///    *  @param aParentFolder
    ///    *         The id of the parent folder
    ///    *  @param aName
    ///    *         The name of the new folder
    ///    *  @param aIndex
    ///    *         The index to insert at, or DEFAULT_INDEX to append
    ///    *  @param [optional] aGuid
    ///    *         The GUID to be set for the new item.  If not set, a new GUID is
    ///    *         generated.  Unless you've a very sound reason, such as an undo
    ///    *         manager implementation, do not pass this argument.
    ///    *  @param [optional] aSource
    ///    *         The change source, forwarded to all bookmark observers. Defaults
    ///    *         to SOURCE_DEFAULT.
    ///    *  @return The ID of the newly-inserted folder.
    ///    *  @throws if aGuid is malformed.
    ///    */
    /// ```
    ///

    /// `[can_run_script] long long createFolder (in long long aParentFolder, in AUTF8String name, in long index, [optional] in ACString aGuid, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn CreateFolder(&self, aParentFolder: i64, name: *const ::nsstring::nsACString, index: i32, aGuid: *const ::nsstring::nsACString, aSource: u16, _retval: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).CreateFolder)(self, aParentFolder, name, index, aGuid, aSource, _retval)
    }


    /// ```text
    /// /**
    ///    * Set the title for an item.
    ///    *  @param aItemId
    ///    *         The id of the item whose title should be updated.
    ///    *  @param aTitle
    ///    *         The new title for the bookmark.
    ///    *  @param [optional] aSource
    ///    *         The change source, forwarded to all bookmark observers. Defaults
    ///    *         to SOURCE_DEFAULT.
    ///    *
    ///    *  @note  aTitle will be truncated to TITLE_LENGTH_MAX.
    ///    */
    /// ```
    ///

    /// `void setItemTitle (in long long aItemId, in AUTF8String aTitle, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn SetItemTitle(&self, aItemId: i64, aTitle: *const ::nsstring::nsACString, aSource: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetItemTitle)(self, aItemId, aTitle, aSource)
    }


    /// ```text
    /// /**
    ///    * Get the title for an item.
    ///    *
    ///    * If no item title is available it will return a void string (null in JS).
    ///    *
    ///    *  @param aItemId
    ///    *         The id of the item whose title should be retrieved
    ///    *  @return The title of the item.
    ///    */
    /// ```
    ///

    /// `AUTF8String getItemTitle (in long long aItemId);`
    #[inline]
    pub unsafe fn GetItemTitle(&self, aItemId: i64, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetItemTitle)(self, aItemId, _retval)
    }


    /// ```text
    /// /**
    ///    * Set the last modified time for an item.
    ///    *
    ///    * @param aItemId
    ///    *        the id of the item whose last modified time should be updated.
    ///    * @param aLastModified
    ///    *        the new last modified value in microseconds.  Note that it is
    ///    *        rounded down to milliseconds precision.
    ///    * @param [optional] aSource
    ///    *        The change source, forwarded to all bookmark observers. Defaults
    ///    *        to SOURCE_DEFAULT.
    ///    *
    ///    * @note This is the only method that will send an itemChanged notification
    ///    *       for the property.  lastModified will still be updated in
    ///    *       any other method that changes an item property, but we will send
    ///    *       the corresponding itemChanged notification instead.
    ///    */
    /// ```
    ///

    /// `void setItemLastModified (in long long aItemId, in PRTime aLastModified, [optional] in unsigned short aSource);`
    #[inline]
    pub unsafe fn SetItemLastModified(&self, aItemId: i64, aLastModified: PRTime, aSource: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetItemLastModified)(self, aItemId, aLastModified, aSource)
    }


    /// ```text
    /// /**
    ///    * Get the parent folder's id for an item.
    ///    */
    /// ```
    ///

    /// `long long getFolderIdForItem (in long long aItemId);`
    #[inline]
    pub unsafe fn GetFolderIdForItem(&self, aItemId: i64, _retval: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetFolderIdForItem)(self, aItemId, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds a bookmark observer. If ownsWeak is false, the bookmark service will
    ///    * keep an owning reference to the observer.  If ownsWeak is true, then
    ///    * aObserver must implement nsISupportsWeakReference, and the bookmark
    ///    * service will keep a weak reference to the observer.
    ///    */
    /// ```
    ///

    /// `void addObserver (in nsINavBookmarkObserver observer, [optional] in boolean ownsWeak);`
    #[inline]
    pub unsafe fn AddObserver(&self, observer: *const nsINavBookmarkObserver, ownsWeak: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, observer, ownsWeak)
    }


    /// ```text
    /// /**
    ///    * Removes a bookmark observer.
    ///    */
    /// ```
    ///

    /// `void removeObserver (in nsINavBookmarkObserver observer);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, observer: *const nsINavBookmarkObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, observer)
    }


    /// ```text
    /// /**
    ///    * Gets an array of registered nsINavBookmarkObserver objects.
    ///    */
    /// ```
    ///

    /// `Array<nsINavBookmarkObserver> getObservers ();`
    #[inline]
    pub unsafe fn GetObservers(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsINavBookmarkObserver>>) -> ::nserror::nsresult {
        ((*self.vtable).GetObservers)(self, _retval)
    }


}


