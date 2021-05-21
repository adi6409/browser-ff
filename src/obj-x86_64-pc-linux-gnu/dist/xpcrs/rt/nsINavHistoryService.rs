//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/toolkit/components/places/nsINavHistoryService.idl
//


/// `interface nsINavHistoryResultNode : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryResultNode {
    vtable: *const nsINavHistoryResultNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryResultNode.
unsafe impl XpCom for nsINavHistoryResultNode {
    const IID: nsIID = nsID(0x91d104bb, 0x17ef, 0x404b,
        [0x9f, 0x9a, 0xd9, 0xed, 0x8d, 0xe6, 0x82, 0x4c]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryResultNode {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryResultNode.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryResultNodeCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryResultNode`.
    fn coerce_from(v: &nsINavHistoryResultNode) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryResultNodeCoerce for nsINavHistoryResultNode {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultNode) -> &Self {
        v
    }
}

impl nsINavHistoryResultNode {
    /// Cast this `nsINavHistoryResultNode` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryResultNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryResultNode {
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
impl<T: nsISupportsCoerce> nsINavHistoryResultNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultNode) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryResultNode
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryResultNodeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute nsINavHistoryContainerResultNode parent; */
    pub GetParent: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aParent: *mut*const nsINavHistoryContainerResultNode) -> ::nserror::nsresult,

    /* readonly attribute nsINavHistoryResult parentResult; */
    pub GetParentResult: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aParentResult: *mut*const nsINavHistoryResult) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String uri; */
    pub GetUri: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aUri: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long type; */
    pub GetType: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aType: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String title; */
    pub GetTitle: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aTitle: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute unsigned long accessCount; */
    pub GetAccessCount: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aAccessCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute PRTime time; */
    pub GetTime: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aTime: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute AUTF8String icon; */
    pub GetIcon: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aIcon: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long indentLevel; */
    pub GetIndentLevel: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aIndentLevel: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long bookmarkIndex; */
    pub GetBookmarkIndex: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aBookmarkIndex: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long long itemId; */
    pub GetItemId: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aItemId: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute PRTime dateAdded; */
    pub GetDateAdded: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aDateAdded: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute PRTime lastModified; */
    pub GetLastModified: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aLastModified: *mut PRTime) -> ::nserror::nsresult,

    /* readonly attribute AString tags; */
    pub GetTags: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aTags: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute ACString pageGuid; */
    pub GetPageGuid: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aPageGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute ACString bookmarkGuid; */
    pub GetBookmarkGuid: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aBookmarkGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute long long visitId; */
    pub GetVisitId: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aVisitId: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute long long fromVisitId; */
    pub GetFromVisitId: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aFromVisitId: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute unsigned long visitType; */
    pub GetVisitType: unsafe extern "system" fn (this: *const nsINavHistoryResultNode, aVisitType: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryResultNode {
    /// ```text
    /// /**
    ///    * Identifies the type of this node. This node can then be QI-ed to the
    ///    * corresponding specialized result node interface.
    ///    */
    /// ```
    ///

    pub const RESULT_TYPE_URI: i64 = 0;


    pub const RESULT_TYPE_QUERY: i64 = 5;


    pub const RESULT_TYPE_FOLDER: i64 = 6;


    pub const RESULT_TYPE_SEPARATOR: i64 = 7;


    pub const RESULT_TYPE_FOLDER_SHORTCUT: i64 = 9;

    /// ```text
    /// /**
    ///    * Indentifies the parent result node in the result set. This is null for
    ///    * top level nodes.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsINavHistoryContainerResultNode parent;`
    #[inline]
    pub unsafe fn GetParent(&self, aParent: *mut*const nsINavHistoryContainerResultNode) -> ::nserror::nsresult {
        ((*self.vtable).GetParent)(self, aParent)
    }


    /// ```text
    /// /**
    ///    * The history-result to which this node belongs.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsINavHistoryResult parentResult;`
    #[inline]
    pub unsafe fn GetParentResult(&self, aParentResult: *mut*const nsINavHistoryResult) -> ::nserror::nsresult {
        ((*self.vtable).GetParentResult)(self, aParentResult)
    }


    /// ```text
    /// /**
    ///    * URI of the resource in question. For visits and URLs, this is the URL of
    ///    * the page. For folders and queries, this is the place: URI of the
    ///    * corresponding folder or query. This may be empty for other types of
    ///    * objects like host containers.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String uri;`
    #[inline]
    pub unsafe fn GetUri(&self, aUri: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetUri)(self, aUri)
    }



    /// `readonly attribute unsigned long type;`
    #[inline]
    pub unsafe fn GetType(&self, aType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetType)(self, aType)
    }


    /// ```text
    /// /**
    ///    * Title of the web page, or of the node's query (day, host, folder, etc)
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String title;`
    #[inline]
    pub unsafe fn GetTitle(&self, aTitle: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTitle)(self, aTitle)
    }


    /// ```text
    /// /**
    ///    * Total number of times the URI has ever been accessed. For hosts, this
    ///    * is the total of the children under it, NOT the total times the host has
    ///    * been accessed (this would require an additional query, so is not given
        ///    * by default when most of the time it is never needed).
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long accessCount;`
    #[inline]
    pub unsafe fn GetAccessCount(&self, aAccessCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetAccessCount)(self, aAccessCount)
    }


    /// ```text
    /// /**
    ///    * This is the time the user accessed the page.
    ///    *
    ///    * If this is a visit, it is the exact time that the page visit occurred.
    ///    *
    ///    * If this is a URI, it is the most recent time that the URI was visited.
    ///    * Even if you ask for all URIs for a given date range long ago, this might
    ///    * contain today's date if the URI was visited today.
    ///    *
    ///    * For hosts, or other node types with children, this is the most recent
    ///    * access time for any of the children.
    ///    *
    ///    * For days queries this is the respective endTime - a maximum possible
    ///    * visit time to fit in the day range.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime time;`
    #[inline]
    pub unsafe fn GetTime(&self, aTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetTime)(self, aTime)
    }


    /// ```text
    /// /**
    ///    * This URI can be used as an image source URI and will give you the favicon
    ///    * for the page. It is *not* the URI of the favicon, but rather something
    ///    * that will resolve to the actual image.
    ///    *
    ///    * In most cases, this is an annotation URI that will query the favicon
    ///    * service. If the entry has no favicon, this is the chrome URI of the
    ///    * default favicon. If the favicon originally lived in chrome, this will
    ///    * be the original chrome URI of the icon.
    ///    */
    /// ```
    ///

    /// `readonly attribute AUTF8String icon;`
    #[inline]
    pub unsafe fn GetIcon(&self, aIcon: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetIcon)(self, aIcon)
    }


    /// ```text
    /// /**
    ///    * This is the number of levels between this node and the top of the
    ///    * hierarchy. The members of result.children have indentLevel = 0, their
    ///    * children have indentLevel = 1, etc. The indent level of the root node is
    ///    * set to -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long indentLevel;`
    #[inline]
    pub unsafe fn GetIndentLevel(&self, aIndentLevel: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetIndentLevel)(self, aIndentLevel)
    }


    /// ```text
    /// /**
    ///    * When this item is in a bookmark folder (parent is of type folder), this is
    ///    * the index into that folder of this node. These indices start at 0 and
    ///    * increase in the order that they appear in the bookmark folder. For items
    ///    * that are not in a bookmark folder, this value is -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long bookmarkIndex;`
    #[inline]
    pub unsafe fn GetBookmarkIndex(&self, aBookmarkIndex: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetBookmarkIndex)(self, aBookmarkIndex)
    }


    /// ```text
    /// /**
    ///    * If the node is an item (bookmark, folder or a separator) this value is the
    ///    * row ID of that bookmark in the database. For other nodes, this value is
    ///    * set to -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long itemId;`
    #[inline]
    pub unsafe fn GetItemId(&self, aItemId: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetItemId)(self, aItemId)
    }


    /// ```text
    /// /**
    ///    * If the node is an item (bookmark, folder or a separator) this value is the
    ///    * time that the item was created. For other nodes, this value is 0.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime dateAdded;`
    #[inline]
    pub unsafe fn GetDateAdded(&self, aDateAdded: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetDateAdded)(self, aDateAdded)
    }


    /// ```text
    /// /**
    ///    * If the node is an item (bookmark, folder or a separator) this value is the
    ///    * time that the item was last modified. For other nodes, this value is 0.
    ///    *
    ///    *  @note When an item is added lastModified is set to the same value as
    ///    *        dateAdded.
    ///    */
    /// ```
    ///

    /// `readonly attribute PRTime lastModified;`
    #[inline]
    pub unsafe fn GetLastModified(&self, aLastModified: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetLastModified)(self, aLastModified)
    }


    /// ```text
    /// /**
    ///    * For uri nodes, this is a sorted list of the tags, delimited with commans,
    ///    * for the uri represented by this node. Otherwise this is an empty string.
    ///    */
    /// ```
    ///

    /// `readonly attribute AString tags;`
    #[inline]
    pub unsafe fn GetTags(&self, aTags: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetTags)(self, aTags)
    }


    /// ```text
    /// /**
    ///    * The unique ID associated with the page. It my return an empty string
    ///    * if the result node is a non-URI node.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString pageGuid;`
    #[inline]
    pub unsafe fn GetPageGuid(&self, aPageGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetPageGuid)(self, aPageGuid)
    }


    /// ```text
    /// /**
    ///    * The unique ID associated with the bookmark. It returns an empty string
    ///    * if the result node is not associated with a bookmark, a folder or a
    ///    * separator.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString bookmarkGuid;`
    #[inline]
    pub unsafe fn GetBookmarkGuid(&self, aBookmarkGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetBookmarkGuid)(self, aBookmarkGuid)
    }


    /// ```text
    /// /**
    ///    * The unique ID associated with the history visit. For node types other than
    ///    * history visit nodes, this value is -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long visitId;`
    #[inline]
    pub unsafe fn GetVisitId(&self, aVisitId: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetVisitId)(self, aVisitId)
    }


    /// ```text
    /// /**
    ///    * The unique ID associated with visit node which was the referrer of this
    ///    * history visit. For node types other than history visit nodes, or visits
    ///    * without any known referrer, this value is -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long fromVisitId;`
    #[inline]
    pub unsafe fn GetFromVisitId(&self, aFromVisitId: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetFromVisitId)(self, aFromVisitId)
    }


    /// ```text
    /// /**
    ///    * The transition type associated with this visit. For node types other than
    ///    * history visit nodes, this value is 0.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long visitType;`
    #[inline]
    pub unsafe fn GetVisitType(&self, aVisitType: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetVisitType)(self, aVisitType)
    }


}


/// `interface nsINavHistoryContainerResultNode : nsINavHistoryResultNode`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryContainerResultNode {
    vtable: *const nsINavHistoryContainerResultNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryContainerResultNode.
unsafe impl XpCom for nsINavHistoryContainerResultNode {
    const IID: nsIID = nsID(0x3e9cc95f, 0x0d93, 0x45f1,
        [0x89, 0x4f, 0x90, 0x8e, 0xeb, 0x98, 0x66, 0xd7]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryContainerResultNode {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryContainerResultNode.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryContainerResultNodeCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryContainerResultNode`.
    fn coerce_from(v: &nsINavHistoryContainerResultNode) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryContainerResultNodeCoerce for nsINavHistoryContainerResultNode {
    #[inline]
    fn coerce_from(v: &nsINavHistoryContainerResultNode) -> &Self {
        v
    }
}

impl nsINavHistoryContainerResultNode {
    /// Cast this `nsINavHistoryContainerResultNode` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryContainerResultNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryContainerResultNode {
    type Target = nsINavHistoryResultNode;
    #[inline]
    fn deref(&self) -> &nsINavHistoryResultNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsINavHistoryResultNodeCoerce> nsINavHistoryContainerResultNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryContainerResultNode) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryContainerResultNode
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryContainerResultNodeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsINavHistoryResultNodeVTable,

    /* attribute boolean containerOpen; */
    pub GetContainerOpen: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aContainerOpen: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean containerOpen; */
    pub SetContainerOpen: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aContainerOpen: bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned short state; */
    pub GetState: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aState: *mut u16) -> ::nserror::nsresult,

    /* readonly attribute boolean hasChildren; */
    pub GetHasChildren: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aHasChildren: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long childCount; */
    pub GetChildCount: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aChildCount: *mut u32) -> ::nserror::nsresult,

    /* nsINavHistoryResultNode getChild (in unsigned long aIndex); */
    pub GetChild: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aIndex: u32, _retval: *mut *const nsINavHistoryResultNode) -> ::nserror::nsresult,

    /* unsigned long getChildIndex (in nsINavHistoryResultNode aNode); */
    pub GetChildIndex: unsafe extern "system" fn (this: *const nsINavHistoryContainerResultNode, aNode: *const nsINavHistoryResultNode, _retval: *mut u32) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryContainerResultNode {

    pub const STATE_CLOSED: i64 = 0;


    pub const STATE_LOADING: i64 = 1;


    pub const STATE_OPENED: i64 = 2;

    /// ```text
    /// /**
    ///  * Base class for container results. This includes all types of groupings.
    ///  * Bookmark folders and places queries will be QueryResultNodes which extends
    ///  * these items.
    ///  */
    /// /**
    ///    * Set this to allow descent into the container. When closed, attempting
    ///    * to call getChildren or childCount will result in an error. You should
    ///    * set this to false when you are done reading.
    ///    *
    ///    * For HOST and DAY groupings, doing this is free since the children have
    ///    * been precomputed. For queries and bookmark folders, being open means they
    ///    * will keep themselves up-to-date by listening for updates and re-querying
    ///    * as needed.
    ///    */
    /// ```
    ///

    /// `attribute boolean containerOpen;`
    #[inline]
    pub unsafe fn GetContainerOpen(&self, aContainerOpen: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetContainerOpen)(self, aContainerOpen)
    }


    /// ```text
    /// /**
    ///  * Base class for container results. This includes all types of groupings.
    ///  * Bookmark folders and places queries will be QueryResultNodes which extends
    ///  * these items.
    ///  */
    /// /**
    ///    * Set this to allow descent into the container. When closed, attempting
    ///    * to call getChildren or childCount will result in an error. You should
    ///    * set this to false when you are done reading.
    ///    *
    ///    * For HOST and DAY groupings, doing this is free since the children have
    ///    * been precomputed. For queries and bookmark folders, being open means they
    ///    * will keep themselves up-to-date by listening for updates and re-querying
    ///    * as needed.
    ///    */
    /// ```
    ///

    /// `attribute boolean containerOpen;`
    #[inline]
    pub unsafe fn SetContainerOpen(&self, aContainerOpen: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetContainerOpen)(self, aContainerOpen)
    }


    /// ```text
    /// /**
    ///    * Indicates whether the container is closed, loading, or opened.  Loading
    ///    * implies that the container has been opened asynchronously and has not yet
    ///    * fully opened.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned short state;`
    #[inline]
    pub unsafe fn GetState(&self, aState: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetState)(self, aState)
    }


    /// ```text
    /// /**
    ///    * This indicates whether this node "may" have children, and can be used
    ///    * when the container is open or closed. When the container is closed, it
    ///    * will give you an exact answer if the node can easily be populated (for
        ///    * example, a bookmark folder). If not (for example, a complex history query),
    ///    * it will return true. When the container is open, it will always be
    ///    * accurate. It is intended to be used to see if we should draw the "+" next
    ///    * to a tree item.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean hasChildren;`
    #[inline]
    pub unsafe fn GetHasChildren(&self, aHasChildren: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasChildren)(self, aHasChildren)
    }


    /// ```text
    /// /**
    ///    * This gives you the children of the nodes. It is preferrable to use this
    ///    * interface over the array one, since it avoids creating an nsIArray object
    ///    * and the interface is already the correct type.
    ///    *
    ///    * @throws NS_ERROR_NOT_AVAILABLE if containerOpen is false.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long childCount;`
    #[inline]
    pub unsafe fn GetChildCount(&self, aChildCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetChildCount)(self, aChildCount)
    }



    /// `nsINavHistoryResultNode getChild (in unsigned long aIndex);`
    #[inline]
    pub unsafe fn GetChild(&self, aIndex: u32, _retval: *mut *const nsINavHistoryResultNode) -> ::nserror::nsresult {
        ((*self.vtable).GetChild)(self, aIndex, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the index of a direct child in this container.
    ///    *
    ///    * @param aNode
    ///    *        a result node.
    ///    *
    ///    * @return aNode's index in this container.
    ///    * @throws NS_ERROR_NOT_AVAILABLE if containerOpen is false.
    ///    * @throws NS_ERROR_INVALID_ARG if aNode isn't a direct child of this
    ///    * container.
    ///    */
    /// ```
    ///

    /// `unsigned long getChildIndex (in nsINavHistoryResultNode aNode);`
    #[inline]
    pub unsafe fn GetChildIndex(&self, aNode: *const nsINavHistoryResultNode, _retval: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetChildIndex)(self, aNode, _retval)
    }


}


/// `interface nsINavHistoryQueryResultNode : nsINavHistoryContainerResultNode`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryQueryResultNode {
    vtable: *const nsINavHistoryQueryResultNodeVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryQueryResultNode.
unsafe impl XpCom for nsINavHistoryQueryResultNode {
    const IID: nsIID = nsID(0x62817759, 0x4fee, 0x44a3,
        [0xb5, 0x8c, 0x3e, 0x2f, 0x5a, 0xfc, 0x9d, 0x0a]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryQueryResultNode {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryQueryResultNode.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryQueryResultNodeCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryQueryResultNode`.
    fn coerce_from(v: &nsINavHistoryQueryResultNode) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryQueryResultNodeCoerce for nsINavHistoryQueryResultNode {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryResultNode) -> &Self {
        v
    }
}

impl nsINavHistoryQueryResultNode {
    /// Cast this `nsINavHistoryQueryResultNode` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryQueryResultNodeCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryQueryResultNode {
    type Target = nsINavHistoryContainerResultNode;
    #[inline]
    fn deref(&self) -> &nsINavHistoryContainerResultNode {
        unsafe {
            ::std::mem::transmute(self)
        }
    }
}

// Ensure we can use .coerce() to cast to our base types as well. Any type which
// our base interface can coerce from should be coercable from us as well.
impl<T: nsINavHistoryContainerResultNodeCoerce> nsINavHistoryQueryResultNodeCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryResultNode) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryQueryResultNode
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryQueryResultNodeVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsINavHistoryContainerResultNodeVTable,

    /* readonly attribute nsINavHistoryQuery query; */
    pub GetQuery: unsafe extern "system" fn (this: *const nsINavHistoryQueryResultNode, aQuery: *mut*const nsINavHistoryQuery) -> ::nserror::nsresult,

    /* readonly attribute nsINavHistoryQueryOptions queryOptions; */
    pub GetQueryOptions: unsafe extern "system" fn (this: *const nsINavHistoryQueryResultNode, aQueryOptions: *mut*const nsINavHistoryQueryOptions) -> ::nserror::nsresult,

    /* readonly attribute long long folderItemId; */
    pub GetFolderItemId: unsafe extern "system" fn (this: *const nsINavHistoryQueryResultNode, aFolderItemId: *mut i64) -> ::nserror::nsresult,

    /* readonly attribute ACString targetFolderGuid; */
    pub GetTargetFolderGuid: unsafe extern "system" fn (this: *const nsINavHistoryQueryResultNode, aTargetFolderGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryQueryResultNode {

    /// ```text
    /// /**
    ///  * Used for places queries and as a base for bookmark folders.
    ///  *
    ///  * Note that if you request places to *not* be expanded in the options that
    ///  * generated this node, this item will report it has no children and never try
    ///  * to populate itself.
    ///  */
    /// /**
    ///    * Get the query which builds this node's children.
    ///    * Only valid for RESULT_TYPE_QUERY nodes.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsINavHistoryQuery query;`
    #[inline]
    pub unsafe fn GetQuery(&self, aQuery: *mut*const nsINavHistoryQuery) -> ::nserror::nsresult {
        ((*self.vtable).GetQuery)(self, aQuery)
    }


    /// ```text
    /// /**
    ///    * Get the options which group this node's children.
    ///    * Only valid for RESULT_TYPE_QUERY nodes.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsINavHistoryQueryOptions queryOptions;`
    #[inline]
    pub unsafe fn GetQueryOptions(&self, aQueryOptions: *mut*const nsINavHistoryQueryOptions) -> ::nserror::nsresult {
        ((*self.vtable).GetQueryOptions)(self, aQueryOptions)
    }


    /// ```text
    /// /**
    ///    * For both simple folder queries and folder shortcut queries, this is set to
    ///    * the concrete itemId of the folder (i.e. for folder shortcuts it's the
        ///    * target folder id).  Otherwise, this is set to -1.
    ///    */
    /// ```
    ///

    /// `readonly attribute long long folderItemId;`
    #[inline]
    pub unsafe fn GetFolderItemId(&self, aFolderItemId: *mut i64) -> ::nserror::nsresult {
        ((*self.vtable).GetFolderItemId)(self, aFolderItemId)
    }


    /// ```text
    /// /**
    ///    * For both simple folder queries and folder shortcut queries, this is set to
    ///    * the concrete guid of the folder (i.e. for folder shortcuts it's the target
        ///    * folder guid). Otherwise, this is set to an empty string.
    ///    */
    /// ```
    ///

    /// `readonly attribute ACString targetFolderGuid;`
    #[inline]
    pub unsafe fn GetTargetFolderGuid(&self, aTargetFolderGuid: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetTargetFolderGuid)(self, aTargetFolderGuid)
    }


}


/// `interface nsINavHistoryResultObserver : nsISupports`
///

/// ```text
/// /**
///  * Allows clients to observe what is happening to a result as it updates itself
///  * according to history and bookmark system events. Register this observer on a
///  * result using nsINavHistoryResult::addObserver.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryResultObserver {
    vtable: *const nsINavHistoryResultObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryResultObserver.
unsafe impl XpCom for nsINavHistoryResultObserver {
    const IID: nsIID = nsID(0xf62d8b6b, 0x3c4e, 0x4a9f,
        [0xa8, 0x97, 0xdb, 0x60, 0x5d, 0x0b, 0x7a, 0x0f]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryResultObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryResultObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryResultObserverCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryResultObserver`.
    fn coerce_from(v: &nsINavHistoryResultObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryResultObserverCoerce for nsINavHistoryResultObserver {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultObserver) -> &Self {
        v
    }
}

impl nsINavHistoryResultObserver {
    /// Cast this `nsINavHistoryResultObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryResultObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryResultObserver {
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
impl<T: nsISupportsCoerce> nsINavHistoryResultObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResultObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryResultObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryResultObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute boolean observeHistoryDetails; */
    pub GetObserveHistoryDetails: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aObserveHistoryDetails: *mut bool) -> ::nserror::nsresult,

    /* void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex); */
    pub NodeInserted: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aParent: *const nsINavHistoryContainerResultNode, aNode: *const nsINavHistoryResultNode, aNewIndex: u32) -> ::nserror::nsresult,

    /* void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex); */
    pub NodeRemoved: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aParent: *const nsINavHistoryContainerResultNode, aItem: *const nsINavHistoryResultNode, aOldIndex: u32) -> ::nserror::nsresult,

    /* void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex); */
    pub NodeMoved: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aOldParent: *const nsINavHistoryContainerResultNode, aOldIndex: u32, aNewParent: *const nsINavHistoryContainerResultNode, aNewIndex: u32) -> ::nserror::nsresult,

    /* void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle); */
    pub NodeTitleChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewTitle: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aOldURI); */
    pub NodeURIChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aOldURI: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void nodeIconChanged (in nsINavHistoryResultNode aNode); */
    pub NodeIconChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode) -> ::nserror::nsresult,

    /* void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aOldVisitDate, in unsigned long aOldAccessCount); */
    pub NodeHistoryDetailsChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aOldVisitDate: PRTime, aOldAccessCount: u32) -> ::nserror::nsresult,

    /* void nodeTagsChanged (in nsINavHistoryResultNode aNode); */
    pub NodeTagsChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode) -> ::nserror::nsresult,

    /* void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword); */
    pub NodeKeywordChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewKeyword: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
    pub NodeDateAddedChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewValue: PRTime) -> ::nserror::nsresult,

    /* void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue); */
    pub NodeLastModifiedChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aNode: *const nsINavHistoryResultNode, aNewValue: PRTime) -> ::nserror::nsresult,

    /* void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState); */
    pub ContainerStateChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aContainerNode: *const nsINavHistoryContainerResultNode, aOldState: u32, aNewState: u32) -> ::nserror::nsresult,

    /* void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode); */
    pub InvalidateContainer: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aContainerNode: *const nsINavHistoryContainerResultNode) -> ::nserror::nsresult,

    /* void sortingChanged (in unsigned short sortingMode); */
    pub SortingChanged: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, sortingMode: u16) -> ::nserror::nsresult,

    /* void batching (in boolean aToggleMode); */
    pub Batching: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aToggleMode: bool) -> ::nserror::nsresult,

    /* attribute nsINavHistoryResult result; */
    pub GetResult: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aResult: *mut*const nsINavHistoryResult) -> ::nserror::nsresult,

    /* attribute nsINavHistoryResult result; */
    pub SetResult: unsafe extern "system" fn (this: *const nsINavHistoryResultObserver, aResult: *const nsINavHistoryResult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryResultObserver {

    /// ```text
    /// /**
    ///    * Whether the observer is interested into history details changes.
    ///    * Those include visits additions and removals. If the observer doesn't
    ///    * provide this attribute, it will default to true.
    ///    * In practice, the observer won't receive nodeHistoryDetailsChanged.
    ///    * Note: this is only read when the observer is added, it cannot be changed
    ///    * dynamically.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean observeHistoryDetails;`
    #[inline]
    pub unsafe fn GetObserveHistoryDetails(&self, aObserveHistoryDetails: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetObserveHistoryDetails)(self, aObserveHistoryDetails)
    }


    /// ```text
    /// /**
    ///    * Called when 'aItem' is inserted into 'aParent' at index 'aNewIndex'.
    ///    * The item previously at index (if any) and everything below it will have
    ///    * been shifted down by one. The item may be a container or a leaf.
    ///    */
    /// ```
    ///

    /// `void nodeInserted (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aNode, in unsigned long aNewIndex);`
    #[inline]
    pub unsafe fn NodeInserted(&self, aParent: *const nsINavHistoryContainerResultNode, aNode: *const nsINavHistoryResultNode, aNewIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).NodeInserted)(self, aParent, aNode, aNewIndex)
    }


    /// ```text
    /// /**
    ///    * Called whan 'aItem' is removed from 'aParent' at 'aOldIndex'. The item
    ///    * may be a container or a leaf. This function will be called after the item
    ///    * has been removed from its parent list, but before anything else (including
        ///    * NULLing out the item's parent) has happened.
    ///    */
    /// ```
    ///

    /// `void nodeRemoved (in nsINavHistoryContainerResultNode aParent, in nsINavHistoryResultNode aItem, in unsigned long aOldIndex);`
    #[inline]
    pub unsafe fn NodeRemoved(&self, aParent: *const nsINavHistoryContainerResultNode, aItem: *const nsINavHistoryResultNode, aOldIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).NodeRemoved)(self, aParent, aItem, aOldIndex)
    }


    /// ```text
    /// /**
    ///    * Called whan 'aItem' is moved from 'aOldParent' at 'aOldIndex' to
    ///    * aNewParent at aNewIndex. The item may be a container or a leaf.
    ///    *
    ///    * XXX: at the moment, this method is called only when an item is moved
    ///    * within the same container. When an item is moved between containers,
    ///    * a new node is created for the item, and the itemRemoved/itemAdded methods
    ///    * are used.
    ///    */
    /// ```
    ///

    /// `void nodeMoved (in nsINavHistoryResultNode aNode, in nsINavHistoryContainerResultNode aOldParent, in unsigned long aOldIndex, in nsINavHistoryContainerResultNode aNewParent, in unsigned long aNewIndex);`
    #[inline]
    pub unsafe fn NodeMoved(&self, aNode: *const nsINavHistoryResultNode, aOldParent: *const nsINavHistoryContainerResultNode, aOldIndex: u32, aNewParent: *const nsINavHistoryContainerResultNode, aNewIndex: u32) -> ::nserror::nsresult {
        ((*self.vtable).NodeMoved)(self, aNode, aOldParent, aOldIndex, aNewParent, aNewIndex)
    }


    /// ```text
    /// /**
    ///    * Called right after aNode's title has changed.
    ///    *
    ///    * @param aNode
    ///    *        a result node
    ///    * @param aNewTitle
    ///    *        the new title
    ///    */
    /// ```
    ///

    /// `void nodeTitleChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewTitle);`
    #[inline]
    pub unsafe fn NodeTitleChanged(&self, aNode: *const nsINavHistoryResultNode, aNewTitle: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).NodeTitleChanged)(self, aNode, aNewTitle)
    }


    /// ```text
    /// /**
    ///    * Called right after aNode's uri property has changed.
    ///    *
    ///    * @param aNode
    ///    *        a result node
    ///    * @param aNewURI
    ///    *        the old uri
    ///    */
    /// ```
    ///

    /// `void nodeURIChanged (in nsINavHistoryResultNode aNode, in AUTF8String aOldURI);`
    #[inline]
    pub unsafe fn NodeURIChanged(&self, aNode: *const nsINavHistoryResultNode, aOldURI: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).NodeURIChanged)(self, aNode, aOldURI)
    }


    /// ```text
    /// /**
    ///    * Called right after aNode's icon property has changed.
    ///    *
    ///    * @param aNode
    ///    *        a result node
    ///    *
    ///    * @note: The new icon is accessible through aNode.icon.
    ///    */
    /// ```
    ///

    /// `void nodeIconChanged (in nsINavHistoryResultNode aNode);`
    #[inline]
    pub unsafe fn NodeIconChanged(&self, aNode: *const nsINavHistoryResultNode) -> ::nserror::nsresult {
        ((*self.vtable).NodeIconChanged)(self, aNode)
    }


    /// ```text
    /// /**
    ///    * Called right after aNode's time property or accessCount property, or both,
    ///    * have changed.
    ///    *
    ///    * @param aNode
    ///    *        a uri result node
    ///    * @param aOldVisitDate
    ///    *        the old visit date
    ///    * @param aOldAccessCount
    ///    *        the old access-count
    ///    */
    /// ```
    ///

    /// `void nodeHistoryDetailsChanged (in nsINavHistoryResultNode aNode, in PRTime aOldVisitDate, in unsigned long aOldAccessCount);`
    #[inline]
    pub unsafe fn NodeHistoryDetailsChanged(&self, aNode: *const nsINavHistoryResultNode, aOldVisitDate: PRTime, aOldAccessCount: u32) -> ::nserror::nsresult {
        ((*self.vtable).NodeHistoryDetailsChanged)(self, aNode, aOldVisitDate, aOldAccessCount)
    }


    /// ```text
    /// /**
    ///    * Called when the tags set on the uri represented by aNode have changed.
    ///    *
    ///    * @param aNode
    ///    *        a uri result node
    ///    *
    ///    * @note: The new tags list is accessible through aNode.tags.
    ///    */
    /// ```
    ///

    /// `void nodeTagsChanged (in nsINavHistoryResultNode aNode);`
    #[inline]
    pub unsafe fn NodeTagsChanged(&self, aNode: *const nsINavHistoryResultNode) -> ::nserror::nsresult {
        ((*self.vtable).NodeTagsChanged)(self, aNode)
    }


    /// ```text
    /// /**
    ///    * Called right after the aNode's keyword property has changed.
    ///    *
    ///    * @param aNode
    ///    *        a uri result node
    ///    * @param aNewKeyword
    ///    *        the new keyword
    ///    */
    /// ```
    ///

    /// `void nodeKeywordChanged (in nsINavHistoryResultNode aNode, in AUTF8String aNewKeyword);`
    #[inline]
    pub unsafe fn NodeKeywordChanged(&self, aNode: *const nsINavHistoryResultNode, aNewKeyword: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).NodeKeywordChanged)(self, aNode, aNewKeyword)
    }


    /// ```text
    /// /**
    ///    * Called right after aNode's dateAdded property has changed.
    ///    *
    ///    * @param aNode
    ///    *        a result node
    ///    * @param aNewValue
    ///    *        the new value of the dateAdded property
    ///    */
    /// ```
    ///

    /// `void nodeDateAddedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue);`
    #[inline]
    pub unsafe fn NodeDateAddedChanged(&self, aNode: *const nsINavHistoryResultNode, aNewValue: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).NodeDateAddedChanged)(self, aNode, aNewValue)
    }


    /// ```text
    /// /**
    ///    * Called right after aNode's dateModified property has changed.
    ///    *
    ///    * @param aNode
    ///    *        a result node
    ///    * @param aNewValue
    ///    *        the new value of the dateModified property
    ///    */
    /// ```
    ///

    /// `void nodeLastModifiedChanged (in nsINavHistoryResultNode aNode, in PRTime aNewValue);`
    #[inline]
    pub unsafe fn NodeLastModifiedChanged(&self, aNode: *const nsINavHistoryResultNode, aNewValue: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).NodeLastModifiedChanged)(self, aNode, aNewValue)
    }


    /// ```text
    /// /**
    ///    * Called after a container changes state.
    ///    *
    ///    * @param aContainerNode
    ///    *        The container that has changed state.
    ///    * @param aOldState
    ///    *        The state that aContainerNode has transitioned out of.
    ///    * @param aNewState
    ///    *        The state that aContainerNode has transitioned into.
    ///    */
    /// ```
    ///

    /// `void containerStateChanged (in nsINavHistoryContainerResultNode aContainerNode, in unsigned long aOldState, in unsigned long aNewState);`
    #[inline]
    pub unsafe fn ContainerStateChanged(&self, aContainerNode: *const nsINavHistoryContainerResultNode, aOldState: u32, aNewState: u32) -> ::nserror::nsresult {
        ((*self.vtable).ContainerStateChanged)(self, aContainerNode, aOldState, aNewState)
    }


    /// ```text
    /// /**
    ///    * Called when something significant has happened within the container. The
    ///    * contents of the container should be re-built.
    ///    *
    ///    * @param aContainerNode
    ///    *        the container node to invalidate
    ///    */
    /// ```
    ///

    /// `void invalidateContainer (in nsINavHistoryContainerResultNode aContainerNode);`
    #[inline]
    pub unsafe fn InvalidateContainer(&self, aContainerNode: *const nsINavHistoryContainerResultNode) -> ::nserror::nsresult {
        ((*self.vtable).InvalidateContainer)(self, aContainerNode)
    }


    /// ```text
    /// /**
    ///    * This is called to indicate to the UI that the sort has changed to the
    ///    * given mode. For trees, for example, this would update the column headers
    ///    * to reflect the sorting. For many other types of views, this won't be
    ///    * applicable.
    ///    *
    ///    * @param sortingMode  One of nsINavHistoryQueryOptions.SORT_BY_* that
    ///    *                     indicates the new sorting mode.
    ///    *
    ///    * This only is expected to update the sorting UI. invalidateAll() will also
    ///    * get called if the sorting changes to update everything.
    ///    */
    /// ```
    ///

    /// `void sortingChanged (in unsigned short sortingMode);`
    #[inline]
    pub unsafe fn SortingChanged(&self, sortingMode: u16) -> ::nserror::nsresult {
        ((*self.vtable).SortingChanged)(self, sortingMode)
    }


    /// ```text
    /// /**
    ///    * This is called to indicate that a batch operation is about to start or end.
    ///    * The observer could want to disable some events or updates during batches,
    ///    * since multiple operations are packed in a short time.
    ///    * For example treeviews could temporarily suppress select notifications.
    ///    *
    ///    * @param aToggleMode
    ///    *        true if a batch is starting, false if it's ending.
    ///    */
    /// ```
    ///

    /// `void batching (in boolean aToggleMode);`
    #[inline]
    pub unsafe fn Batching(&self, aToggleMode: bool) -> ::nserror::nsresult {
        ((*self.vtable).Batching)(self, aToggleMode)
    }


    /// ```text
    /// /**
    ///    * Called by the result when this observer is added.
    ///    */
    /// ```
    ///

    /// `attribute nsINavHistoryResult result;`
    #[inline]
    pub unsafe fn GetResult(&self, aResult: *mut*const nsINavHistoryResult) -> ::nserror::nsresult {
        ((*self.vtable).GetResult)(self, aResult)
    }


    /// ```text
    /// /**
    ///    * Called by the result when this observer is added.
    ///    */
    /// ```
    ///

    /// `attribute nsINavHistoryResult result;`
    #[inline]
    pub unsafe fn SetResult(&self, aResult: *const nsINavHistoryResult) -> ::nserror::nsresult {
        ((*self.vtable).SetResult)(self, aResult)
    }


}


/// `interface nsINavHistoryResult : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryResult {
    vtable: *const nsINavHistoryResultVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryResult.
unsafe impl XpCom for nsINavHistoryResult {
    const IID: nsIID = nsID(0xc2229ce3, 0x2159, 0x4001,
        [0x85, 0x9c, 0x70, 0x13, 0xc5, 0x2f, 0x76, 0x19]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryResult {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryResult.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryResultCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryResult`.
    fn coerce_from(v: &nsINavHistoryResult) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryResultCoerce for nsINavHistoryResult {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResult) -> &Self {
        v
    }
}

impl nsINavHistoryResult {
    /// Cast this `nsINavHistoryResult` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryResultCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryResult {
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
impl<T: nsISupportsCoerce> nsINavHistoryResultCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryResult) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryResult
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryResultVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned short sortingMode; */
    pub GetSortingMode: unsafe extern "system" fn (this: *const nsINavHistoryResult, aSortingMode: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short sortingMode; */
    pub SetSortingMode: unsafe extern "system" fn (this: *const nsINavHistoryResult, aSortingMode: u16) -> ::nserror::nsresult,

    /* attribute boolean suppressNotifications; */
    pub GetSuppressNotifications: unsafe extern "system" fn (this: *const nsINavHistoryResult, aSuppressNotifications: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean suppressNotifications; */
    pub SetSuppressNotifications: unsafe extern "system" fn (this: *const nsINavHistoryResult, aSuppressNotifications: bool) -> ::nserror::nsresult,

    /* void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsINavHistoryResult, aObserver: *const nsINavHistoryResultObserver, aOwnsWeak: bool) -> ::nserror::nsresult,

    /* void removeObserver (in nsINavHistoryResultObserver aObserver); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsINavHistoryResult, aObserver: *const nsINavHistoryResultObserver) -> ::nserror::nsresult,

    /* readonly attribute nsINavHistoryContainerResultNode root; */
    pub GetRoot: unsafe extern "system" fn (this: *const nsINavHistoryResult, aRoot: *mut *const nsINavHistoryContainerResultNode) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryResult {

    /// ```text
    /// /**
    ///  * The result of a history/bookmark query.
    ///  */
    /// /**
    ///    * Sorts all nodes recursively by the given parameter, one of
    ///    * nsINavHistoryQueryOptions.SORT_BY_*  This will update the corresponding
    ///    * options for this result, so that re-using the current options/queries will
    ///    * always give you the current view.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short sortingMode;`
    #[inline]
    pub unsafe fn GetSortingMode(&self, aSortingMode: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetSortingMode)(self, aSortingMode)
    }


    /// ```text
    /// /**
    ///  * The result of a history/bookmark query.
    ///  */
    /// /**
    ///    * Sorts all nodes recursively by the given parameter, one of
    ///    * nsINavHistoryQueryOptions.SORT_BY_*  This will update the corresponding
    ///    * options for this result, so that re-using the current options/queries will
    ///    * always give you the current view.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short sortingMode;`
    #[inline]
    pub unsafe fn SetSortingMode(&self, aSortingMode: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetSortingMode)(self, aSortingMode)
    }


    /// ```text
    /// /**
    ///    * Whether or not notifications on result changes are suppressed.
    ///    * Initially set to false.
    ///    *
    ///    * Use this to avoid flickering and to improve performance when you
    ///    * do temporary changes to the result structure (e.g. when searching for a
        ///    * node recursively).
    ///    */
    /// ```
    ///

    /// `attribute boolean suppressNotifications;`
    #[inline]
    pub unsafe fn GetSuppressNotifications(&self, aSuppressNotifications: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetSuppressNotifications)(self, aSuppressNotifications)
    }


    /// ```text
    /// /**
    ///    * Whether or not notifications on result changes are suppressed.
    ///    * Initially set to false.
    ///    *
    ///    * Use this to avoid flickering and to improve performance when you
    ///    * do temporary changes to the result structure (e.g. when searching for a
        ///    * node recursively).
    ///    */
    /// ```
    ///

    /// `attribute boolean suppressNotifications;`
    #[inline]
    pub unsafe fn SetSuppressNotifications(&self, aSuppressNotifications: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetSuppressNotifications)(self, aSuppressNotifications)
    }


    /// ```text
    /// /**
    ///    * Adds an observer for changes done in the result.
    ///    *
    ///    * @param aObserver
    ///    *        a result observer.
    ///    * @param aOwnsWeak
    ///    *        If false, the result will keep an owning reference to the observer,
    ///    *        which must be removed using removeObserver.
    ///    *        If true, the result will keep a weak reference to the observer, which
    ///    *        must implement nsISupportsWeakReference.
    ///    *
    ///    * @see nsINavHistoryResultObserver
    ///    */
    /// ```
    ///

    /// `void addObserver (in nsINavHistoryResultObserver aObserver, [optional] in boolean aOwnsWeak);`
    #[inline]
    pub unsafe fn AddObserver(&self, aObserver: *const nsINavHistoryResultObserver, aOwnsWeak: bool) -> ::nserror::nsresult {
        ((*self.vtable).AddObserver)(self, aObserver, aOwnsWeak)
    }


    /// ```text
    /// /**
    ///    * Removes an observer that was added by addObserver.
    ///    *
    ///    * @param aObserver
    ///    *        a result observer that was added by addObserver.
    ///    */
    /// ```
    ///

    /// `void removeObserver (in nsINavHistoryResultObserver aObserver);`
    #[inline]
    pub unsafe fn RemoveObserver(&self, aObserver: *const nsINavHistoryResultObserver) -> ::nserror::nsresult {
        ((*self.vtable).RemoveObserver)(self, aObserver)
    }


    /// ```text
    /// /**
    ///    * This is the root of the results. Remember that you need to open all
    ///    * containers for their contents to be valid.
    ///    *
    ///    * When a result goes out of scope it will continue to observe changes till
    ///    * it is cycle collected.  While the result waits to be collected it will stay
    ///    * in memory, and continue to update itself, potentially causing unwanted
    ///    * additional work.  When you close the root node the result will stop
    ///    * observing changes, so it is good practice to close the root node when you
    ///    * are done with a result, since that will avoid unwanted performance hits.
    ///    */
    /// ```
    ///

    /// `readonly attribute nsINavHistoryContainerResultNode root;`
    #[inline]
    pub unsafe fn GetRoot(&self, aRoot: *mut *const nsINavHistoryContainerResultNode) -> ::nserror::nsresult {
        ((*self.vtable).GetRoot)(self, aRoot)
    }


}


/// `interface nsINavHistoryObserver : nsISupports`
///

/// ```text
/// /**
///  * DANGER! If you are in the middle of a batch transaction, there may be a
///  * database transaction active. You can still access the DB, but be careful.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryObserver {
    vtable: *const nsINavHistoryObserverVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryObserver.
unsafe impl XpCom for nsINavHistoryObserver {
    const IID: nsIID = nsID(0x0f0f45b0, 0x13a1, 0x44ae,
        [0xa0, 0xab, 0xc6, 0x04, 0x6e, 0xc6, 0xd4, 0xda]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryObserver {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryObserver.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryObserverCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryObserver`.
    fn coerce_from(v: &nsINavHistoryObserver) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryObserverCoerce for nsINavHistoryObserver {
    #[inline]
    fn coerce_from(v: &nsINavHistoryObserver) -> &Self {
        v
    }
}

impl nsINavHistoryObserver {
    /// Cast this `nsINavHistoryObserver` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryObserverCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryObserver {
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
impl<T: nsISupportsCoerce> nsINavHistoryObserverCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryObserver) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryObserver
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryObserverVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void onBeginUpdateBatch (); */
    pub OnBeginUpdateBatch: unsafe extern "system" fn (this: *const nsINavHistoryObserver) -> ::nserror::nsresult,

    /* void onEndUpdateBatch (); */
    pub OnEndUpdateBatch: unsafe extern "system" fn (this: *const nsINavHistoryObserver) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryObserver {

    /// ```text
    /// /**
    ///    * Notifies you that a bunch of things are about to change, don't do any
    ///    * heavy-duty processing until onEndUpdateBatch is called.
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
    ///    * Notifies you that we are done doing a bunch of things and you should go
    ///    * ahead and update UI, etc.
    ///    */
    /// ```
    ///

    /// `void onEndUpdateBatch ();`
    #[inline]
    pub unsafe fn OnEndUpdateBatch(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).OnEndUpdateBatch)(self, )
    }


}


/// `interface nsINavHistoryQuery : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryQuery {
    vtable: *const nsINavHistoryQueryVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryQuery.
unsafe impl XpCom for nsINavHistoryQuery {
    const IID: nsIID = nsID(0xdc87ae79, 0x22f1, 0x4dcf,
        [0x97, 0x5b, 0x85, 0x2b, 0x01, 0xd2, 0x10, 0xcb]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryQuery {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryQuery.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryQueryCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryQuery`.
    fn coerce_from(v: &nsINavHistoryQuery) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryQueryCoerce for nsINavHistoryQuery {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQuery) -> &Self {
        v
    }
}

impl nsINavHistoryQuery {
    /// Cast this `nsINavHistoryQuery` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryQueryCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryQuery {
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
impl<T: nsISupportsCoerce> nsINavHistoryQueryCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQuery) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryQuery
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryQueryVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute PRTime beginTime; */
    pub GetBeginTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aBeginTime: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime beginTime; */
    pub SetBeginTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aBeginTime: PRTime) -> ::nserror::nsresult,

    /* attribute unsigned long beginTimeReference; */
    pub GetBeginTimeReference: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aBeginTimeReference: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long beginTimeReference; */
    pub SetBeginTimeReference: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aBeginTimeReference: u32) -> ::nserror::nsresult,

    /* readonly attribute boolean hasBeginTime; */
    pub GetHasBeginTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aHasBeginTime: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute PRTime absoluteBeginTime; */
    pub GetAbsoluteBeginTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aAbsoluteBeginTime: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime endTime; */
    pub GetEndTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* attribute PRTime endTime; */
    pub SetEndTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aEndTime: PRTime) -> ::nserror::nsresult,

    /* attribute unsigned long endTimeReference; */
    pub GetEndTimeReference: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aEndTimeReference: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long endTimeReference; */
    pub SetEndTimeReference: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aEndTimeReference: u32) -> ::nserror::nsresult,

    /* readonly attribute boolean hasEndTime; */
    pub GetHasEndTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aHasEndTime: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute PRTime absoluteEndTime; */
    pub GetAbsoluteEndTime: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aAbsoluteEndTime: *mut PRTime) -> ::nserror::nsresult,

    /* attribute AString searchTerms; */
    pub GetSearchTerms: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aSearchTerms: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString searchTerms; */
    pub SetSearchTerms: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aSearchTerms: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* readonly attribute boolean hasSearchTerms; */
    pub GetHasSearchTerms: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aHasSearchTerms: *mut bool) -> ::nserror::nsresult,

    /* attribute long minVisits; */
    pub GetMinVisits: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aMinVisits: *mut i32) -> ::nserror::nsresult,

    /* attribute long minVisits; */
    pub SetMinVisits: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aMinVisits: i32) -> ::nserror::nsresult,

    /* attribute long maxVisits; */
    pub GetMaxVisits: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aMaxVisits: *mut i32) -> ::nserror::nsresult,

    /* attribute long maxVisits; */
    pub SetMaxVisits: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aMaxVisits: i32) -> ::nserror::nsresult,

    /* void setTransitions (in Array<unsigned long> transitions); */
    pub SetTransitions: unsafe extern "system" fn (this: *const nsINavHistoryQuery, transitions: *const thin_vec::ThinVec<u32>) -> ::nserror::nsresult,

    /* Array<unsigned long> getTransitions (); */
    pub GetTransitions: unsafe extern "system" fn (this: *const nsINavHistoryQuery, _retval: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult,

    /* readonly attribute unsigned long transitionCount; */
    pub GetTransitionCount: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aTransitionCount: *mut u32) -> ::nserror::nsresult,

    /* attribute boolean onlyBookmarked; */
    pub GetOnlyBookmarked: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aOnlyBookmarked: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean onlyBookmarked; */
    pub SetOnlyBookmarked: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aOnlyBookmarked: bool) -> ::nserror::nsresult,

    /* attribute boolean domainIsHost; */
    pub GetDomainIsHost: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aDomainIsHost: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean domainIsHost; */
    pub SetDomainIsHost: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aDomainIsHost: bool) -> ::nserror::nsresult,

    /* attribute AUTF8String domain; */
    pub GetDomain: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aDomain: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String domain; */
    pub SetDomain: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aDomain: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean hasDomain; */
    pub GetHasDomain: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aHasDomain: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIURI uri; */
    pub GetUri: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aUri: *mut*const nsIURI) -> ::nserror::nsresult,

    /* attribute nsIURI uri; */
    pub SetUri: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aUri: *const nsIURI) -> ::nserror::nsresult,

    /* readonly attribute boolean hasUri; */
    pub GetHasUri: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aHasUri: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean annotationIsNot; */
    pub GetAnnotationIsNot: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aAnnotationIsNot: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean annotationIsNot; */
    pub SetAnnotationIsNot: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aAnnotationIsNot: bool) -> ::nserror::nsresult,

    /* attribute AUTF8String annotation; */
    pub GetAnnotation: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aAnnotation: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* attribute AUTF8String annotation; */
    pub SetAnnotation: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aAnnotation: *const ::nsstring::nsACString) -> ::nserror::nsresult,

    /* readonly attribute boolean hasAnnotation; */
    pub GetHasAnnotation: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aHasAnnotation: *mut bool) -> ::nserror::nsresult,

    /* attribute nsIVariant tags; */
    pub GetTags: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aTags: *mut*const nsIVariant) -> ::nserror::nsresult,

    /* attribute nsIVariant tags; */
    pub SetTags: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aTags: *const nsIVariant) -> ::nserror::nsresult,

    /* attribute boolean tagsAreNot; */
    pub GetTagsAreNot: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aTagsAreNot: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean tagsAreNot; */
    pub SetTagsAreNot: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aTagsAreNot: bool) -> ::nserror::nsresult,

    /* Array<ACString> getParents (); */
    pub GetParents: unsafe extern "system" fn (this: *const nsINavHistoryQuery, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* readonly attribute unsigned long parentCount; */
    pub GetParentCount: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aParentCount: *mut u32) -> ::nserror::nsresult,

    /* void setParents (in Array<ACString> aGuids); */
    pub SetParents: unsafe extern "system" fn (this: *const nsINavHistoryQuery, aGuids: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult,

    /* nsINavHistoryQuery clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsINavHistoryQuery, _retval: *mut *const nsINavHistoryQuery) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryQuery {
    /// ```text
    /// /**
    ///  * This object encapsulates all the query parameters you're likely to need
    ///  * when building up history UI. All parameters are ANDed together.
    ///  *
    ///  * This is not intended to be a super-general query mechanism. This was designed
    ///  * so that most queries can be done in only one SQL query. This is important
    ///  * because, if the user has their profile on a networked drive, query latency
    ///  * can be non-negligible.
    ///  */
    /// /**
    ///    * Time range for results (INCLUSIVE). The *TimeReference is one of the
    ///    * constants TIME_RELATIVE_* which indicates how to interpret the
    ///    * corresponding time value.
    ///    *   TIME_RELATIVE_EPOCH (default):
    ///    *     The time is relative to Jan 1 1970 GMT, (this is a normal PRTime)
    ///    *   TIME_RELATIVE_TODAY:
    ///    *     The time is relative to this morning at midnight. Normally used for
    ///    *     queries relative to today. For example, a "past week" query would be
    ///    *     today-6 days -> today+1 day
    ///    *   TIME_RELATIVE_NOW:
    ///    *     The time is relative to right now.
    ///    *
    ///    * Note: PRTime is in MICROseconds since 1 Jan 1970. Javascript date objects
    ///    * are expressed in MILLIseconds since 1 Jan 1970.
    ///    *
    ///    * As a special case, a 0 time relative to TIME_RELATIVE_EPOCH indicates that
    ///    * the time is not part of the query. This is the default, so an empty query
    ///    * will match any time. The has* functions return whether the corresponding
    ///    * time is considered.
    ///    *
    ///    * You can read absolute*Time to get the time value that the currently loaded
    ///    * reference points + offset resolve to.
    ///    */
    /// ```
    ///

    pub const TIME_RELATIVE_EPOCH: i64 = 0;


    pub const TIME_RELATIVE_TODAY: i64 = 1;


    pub const TIME_RELATIVE_NOW: i64 = 2;


    /// `attribute PRTime beginTime;`
    #[inline]
    pub unsafe fn GetBeginTime(&self, aBeginTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetBeginTime)(self, aBeginTime)
    }



    /// `attribute PRTime beginTime;`
    #[inline]
    pub unsafe fn SetBeginTime(&self, aBeginTime: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetBeginTime)(self, aBeginTime)
    }



    /// `attribute unsigned long beginTimeReference;`
    #[inline]
    pub unsafe fn GetBeginTimeReference(&self, aBeginTimeReference: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetBeginTimeReference)(self, aBeginTimeReference)
    }



    /// `attribute unsigned long beginTimeReference;`
    #[inline]
    pub unsafe fn SetBeginTimeReference(&self, aBeginTimeReference: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetBeginTimeReference)(self, aBeginTimeReference)
    }



    /// `readonly attribute boolean hasBeginTime;`
    #[inline]
    pub unsafe fn GetHasBeginTime(&self, aHasBeginTime: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasBeginTime)(self, aHasBeginTime)
    }



    /// `readonly attribute PRTime absoluteBeginTime;`
    #[inline]
    pub unsafe fn GetAbsoluteBeginTime(&self, aAbsoluteBeginTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetAbsoluteBeginTime)(self, aAbsoluteBeginTime)
    }



    /// `attribute PRTime endTime;`
    #[inline]
    pub unsafe fn GetEndTime(&self, aEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetEndTime)(self, aEndTime)
    }



    /// `attribute PRTime endTime;`
    #[inline]
    pub unsafe fn SetEndTime(&self, aEndTime: PRTime) -> ::nserror::nsresult {
        ((*self.vtable).SetEndTime)(self, aEndTime)
    }



    /// `attribute unsigned long endTimeReference;`
    #[inline]
    pub unsafe fn GetEndTimeReference(&self, aEndTimeReference: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetEndTimeReference)(self, aEndTimeReference)
    }



    /// `attribute unsigned long endTimeReference;`
    #[inline]
    pub unsafe fn SetEndTimeReference(&self, aEndTimeReference: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetEndTimeReference)(self, aEndTimeReference)
    }



    /// `readonly attribute boolean hasEndTime;`
    #[inline]
    pub unsafe fn GetHasEndTime(&self, aHasEndTime: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasEndTime)(self, aHasEndTime)
    }



    /// `readonly attribute PRTime absoluteEndTime;`
    #[inline]
    pub unsafe fn GetAbsoluteEndTime(&self, aAbsoluteEndTime: *mut PRTime) -> ::nserror::nsresult {
        ((*self.vtable).GetAbsoluteEndTime)(self, aAbsoluteEndTime)
    }


    /// ```text
    /// /**
    ///    * Text search terms.
    ///    */
    /// ```
    ///

    /// `attribute AString searchTerms;`
    #[inline]
    pub unsafe fn GetSearchTerms(&self, aSearchTerms: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetSearchTerms)(self, aSearchTerms)
    }


    /// ```text
    /// /**
    ///    * Text search terms.
    ///    */
    /// ```
    ///

    /// `attribute AString searchTerms;`
    #[inline]
    pub unsafe fn SetSearchTerms(&self, aSearchTerms: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetSearchTerms)(self, aSearchTerms)
    }



    /// `readonly attribute boolean hasSearchTerms;`
    #[inline]
    pub unsafe fn GetHasSearchTerms(&self, aHasSearchTerms: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasSearchTerms)(self, aHasSearchTerms)
    }


    /// ```text
    /// /**
    ///    * Set lower or upper limits for how many times an item has been
    ///    * visited.  The default is -1, and in that case all items are
    ///    * matched regardless of their visit count.
    ///    */
    /// ```
    ///

    /// `attribute long minVisits;`
    #[inline]
    pub unsafe fn GetMinVisits(&self, aMinVisits: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetMinVisits)(self, aMinVisits)
    }


    /// ```text
    /// /**
    ///    * Set lower or upper limits for how many times an item has been
    ///    * visited.  The default is -1, and in that case all items are
    ///    * matched regardless of their visit count.
    ///    */
    /// ```
    ///

    /// `attribute long minVisits;`
    #[inline]
    pub unsafe fn SetMinVisits(&self, aMinVisits: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetMinVisits)(self, aMinVisits)
    }



    /// `attribute long maxVisits;`
    #[inline]
    pub unsafe fn GetMaxVisits(&self, aMaxVisits: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxVisits)(self, aMaxVisits)
    }



    /// `attribute long maxVisits;`
    #[inline]
    pub unsafe fn SetMaxVisits(&self, aMaxVisits: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetMaxVisits)(self, aMaxVisits)
    }


    /// ```text
    /// /**
    ///    * When the set of transitions is nonempty, results are limited to pages which
    ///    * have at least one visit for each of the transition types.
    ///    * @note: For searching on more than one transition this can be very slow.
    ///    *
    ///    * Limit results to the specified list of transition types.
    ///    */
    /// ```
    ///

    /// `void setTransitions (in Array<unsigned long> transitions);`
    #[inline]
    pub unsafe fn SetTransitions(&self, transitions: *const thin_vec::ThinVec<u32>) -> ::nserror::nsresult {
        ((*self.vtable).SetTransitions)(self, transitions)
    }


    /// ```text
    /// /**
    ///    * Get the transitions set for this query.
    ///    */
    /// ```
    ///

    /// `Array<unsigned long> getTransitions ();`
    #[inline]
    pub unsafe fn GetTransitions(&self, _retval: *mut thin_vec::ThinVec<u32>) -> ::nserror::nsresult {
        ((*self.vtable).GetTransitions)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Get the count of the set query transitions.
    ///    */
    /// ```
    ///

    /// `readonly attribute unsigned long transitionCount;`
    #[inline]
    pub unsafe fn GetTransitionCount(&self, aTransitionCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTransitionCount)(self, aTransitionCount)
    }


    /// ```text
    /// /**
    ///    * When set, returns only bookmarked items, when unset, returns anything. Setting this
    ///    * is equivalent to listing all bookmark folders in the 'folders' parameter.
    ///    */
    /// ```
    ///

    /// `attribute boolean onlyBookmarked;`
    #[inline]
    pub unsafe fn GetOnlyBookmarked(&self, aOnlyBookmarked: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetOnlyBookmarked)(self, aOnlyBookmarked)
    }


    /// ```text
    /// /**
    ///    * When set, returns only bookmarked items, when unset, returns anything. Setting this
    ///    * is equivalent to listing all bookmark folders in the 'folders' parameter.
    ///    */
    /// ```
    ///

    /// `attribute boolean onlyBookmarked;`
    #[inline]
    pub unsafe fn SetOnlyBookmarked(&self, aOnlyBookmarked: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetOnlyBookmarked)(self, aOnlyBookmarked)
    }


    /// ```text
    /// /**
    ///    * This controls the meaning of 'domain', and whether it is an exact match
    ///    * 'domainIsHost' = true, or hierarchical (= false).
    ///    */
    /// ```
    ///

    /// `attribute boolean domainIsHost;`
    #[inline]
    pub unsafe fn GetDomainIsHost(&self, aDomainIsHost: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetDomainIsHost)(self, aDomainIsHost)
    }


    /// ```text
    /// /**
    ///    * This controls the meaning of 'domain', and whether it is an exact match
    ///    * 'domainIsHost' = true, or hierarchical (= false).
    ///    */
    /// ```
    ///

    /// `attribute boolean domainIsHost;`
    #[inline]
    pub unsafe fn SetDomainIsHost(&self, aDomainIsHost: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetDomainIsHost)(self, aDomainIsHost)
    }


    /// ```text
    /// /**
    ///    * This is the host or domain name (controlled by domainIsHost). When
    ///    * domainIsHost, domain only does exact matching on host names. Otherwise,
    ///    * it will return anything whose host name ends in 'domain'.
    ///    *
    ///    * This one is a little different than most. Setting it to an empty string
    ///    * is a real query and will match any URI that has no host name (local files
        ///    * and such). Set this to NULL (in C++ use SetIsVoid) if you don't want
    ///    * domain matching.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String domain;`
    #[inline]
    pub unsafe fn GetDomain(&self, aDomain: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetDomain)(self, aDomain)
    }


    /// ```text
    /// /**
    ///    * This is the host or domain name (controlled by domainIsHost). When
    ///    * domainIsHost, domain only does exact matching on host names. Otherwise,
    ///    * it will return anything whose host name ends in 'domain'.
    ///    *
    ///    * This one is a little different than most. Setting it to an empty string
    ///    * is a real query and will match any URI that has no host name (local files
        ///    * and such). Set this to NULL (in C++ use SetIsVoid) if you don't want
    ///    * domain matching.
    ///    */
    /// ```
    ///

    /// `attribute AUTF8String domain;`
    #[inline]
    pub unsafe fn SetDomain(&self, aDomain: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetDomain)(self, aDomain)
    }



    /// `readonly attribute boolean hasDomain;`
    #[inline]
    pub unsafe fn GetHasDomain(&self, aHasDomain: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasDomain)(self, aHasDomain)
    }


    /// ```text
    /// /**
    ///    * This is a URI to match, to, for example, find out every time you visited
    ///    * a given URI. This is an exact match.
    ///    */
    /// ```
    ///

    /// `attribute nsIURI uri;`
    #[inline]
    pub unsafe fn GetUri(&self, aUri: *mut*const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).GetUri)(self, aUri)
    }


    /// ```text
    /// /**
    ///    * This is a URI to match, to, for example, find out every time you visited
    ///    * a given URI. This is an exact match.
    ///    */
    /// ```
    ///

    /// `attribute nsIURI uri;`
    #[inline]
    pub unsafe fn SetUri(&self, aUri: *const nsIURI) -> ::nserror::nsresult {
        ((*self.vtable).SetUri)(self, aUri)
    }



    /// `readonly attribute boolean hasUri;`
    #[inline]
    pub unsafe fn GetHasUri(&self, aHasUri: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasUri)(self, aHasUri)
    }


    /// ```text
    /// /**
    ///    * Test for existence or non-existence of a given annotation. We don't
    ///    * currently support >1 annotation name per query. If 'annotationIsNot' is
    ///    * true, we test for the non-existence of the specified annotation.
    ///    *
    ///    * Testing for not annotation will do the same thing as a normal query and
    ///    * remove everything that doesn't have that annotation. Asking for things
    ///    * that DO have a given annotation is a little different. It also includes
    ///    * things that have never been visited. This allows place queries to be
    ///    * returned as well as anything else that may have been tagged with an
    ///    * annotation. This will only work for RESULTS_AS_URI since there will be
    ///    * no visits for these items.
    ///    */
    /// ```
    ///

    /// `attribute boolean annotationIsNot;`
    #[inline]
    pub unsafe fn GetAnnotationIsNot(&self, aAnnotationIsNot: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAnnotationIsNot)(self, aAnnotationIsNot)
    }


    /// ```text
    /// /**
    ///    * Test for existence or non-existence of a given annotation. We don't
    ///    * currently support >1 annotation name per query. If 'annotationIsNot' is
    ///    * true, we test for the non-existence of the specified annotation.
    ///    *
    ///    * Testing for not annotation will do the same thing as a normal query and
    ///    * remove everything that doesn't have that annotation. Asking for things
    ///    * that DO have a given annotation is a little different. It also includes
    ///    * things that have never been visited. This allows place queries to be
    ///    * returned as well as anything else that may have been tagged with an
    ///    * annotation. This will only work for RESULTS_AS_URI since there will be
    ///    * no visits for these items.
    ///    */
    /// ```
    ///

    /// `attribute boolean annotationIsNot;`
    #[inline]
    pub unsafe fn SetAnnotationIsNot(&self, aAnnotationIsNot: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAnnotationIsNot)(self, aAnnotationIsNot)
    }



    /// `attribute AUTF8String annotation;`
    #[inline]
    pub unsafe fn GetAnnotation(&self, aAnnotation: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).GetAnnotation)(self, aAnnotation)
    }



    /// `attribute AUTF8String annotation;`
    #[inline]
    pub unsafe fn SetAnnotation(&self, aAnnotation: *const ::nsstring::nsACString) -> ::nserror::nsresult {
        ((*self.vtable).SetAnnotation)(self, aAnnotation)
    }



    /// `readonly attribute boolean hasAnnotation;`
    #[inline]
    pub unsafe fn GetHasAnnotation(&self, aHasAnnotation: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasAnnotation)(self, aHasAnnotation)
    }


    /// ```text
    /// /**
    ///    * Limit results to items that are tagged with all of the given tags.  This
    ///    * attribute must be set to an array of strings.  When called as a getter it
    ///    * will return an array of strings sorted ascending in lexicographical order.
    ///    * The array may be empty in either case.  Duplicate tags may be specified
    ///    * when setting the attribute, but the getter returns only unique tags.
    ///    */
    /// ```
    ///

    /// `attribute nsIVariant tags;`
    #[inline]
    pub unsafe fn GetTags(&self, aTags: *mut*const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).GetTags)(self, aTags)
    }


    /// ```text
    /// /**
    ///    * Limit results to items that are tagged with all of the given tags.  This
    ///    * attribute must be set to an array of strings.  When called as a getter it
    ///    * will return an array of strings sorted ascending in lexicographical order.
    ///    * The array may be empty in either case.  Duplicate tags may be specified
    ///    * when setting the attribute, but the getter returns only unique tags.
    ///    */
    /// ```
    ///

    /// `attribute nsIVariant tags;`
    #[inline]
    pub unsafe fn SetTags(&self, aTags: *const nsIVariant) -> ::nserror::nsresult {
        ((*self.vtable).SetTags)(self, aTags)
    }


    /// ```text
    /// /**
    ///    * If 'tagsAreNot' is true, the results are instead limited to items that
    ///    * are not tagged with any of the given tags.  This attribute is used in
    ///    * conjunction with the 'tags' attribute.
    ///    */
    /// ```
    ///

    /// `attribute boolean tagsAreNot;`
    #[inline]
    pub unsafe fn GetTagsAreNot(&self, aTagsAreNot: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetTagsAreNot)(self, aTagsAreNot)
    }


    /// ```text
    /// /**
    ///    * If 'tagsAreNot' is true, the results are instead limited to items that
    ///    * are not tagged with any of the given tags.  This attribute is used in
    ///    * conjunction with the 'tags' attribute.
    ///    */
    /// ```
    ///

    /// `attribute boolean tagsAreNot;`
    #[inline]
    pub unsafe fn SetTagsAreNot(&self, aTagsAreNot: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetTagsAreNot)(self, aTagsAreNot)
    }


    /// ```text
    /// /**
    ///    * Limit results to items that are in all of the given folders.
    ///    */
    /// ```
    ///

    /// `Array<ACString> getParents ();`
    #[inline]
    pub unsafe fn GetParents(&self, _retval: *mut thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).GetParents)(self, _retval)
    }



    /// `readonly attribute unsigned long parentCount;`
    #[inline]
    pub unsafe fn GetParentCount(&self, aParentCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetParentCount)(self, aParentCount)
    }


    /// ```text
    /// /**
    ///    * This is not recursive so results will be returned from the first level of
    ///    * that folder.
    ///    */
    /// ```
    ///

    /// `void setParents (in Array<ACString> aGuids);`
    #[inline]
    pub unsafe fn SetParents(&self, aGuids: *const thin_vec::ThinVec<::nsstring::nsCString>) -> ::nserror::nsresult {
        ((*self.vtable).SetParents)(self, aGuids)
    }


    /// ```text
    /// /**
    ///    * Creates a new query item with the same parameters of this one.
    ///    */
    /// ```
    ///

    /// `nsINavHistoryQuery clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsINavHistoryQuery) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


/// `interface nsINavHistoryQueryOptions : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryQueryOptions {
    vtable: *const nsINavHistoryQueryOptionsVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryQueryOptions.
unsafe impl XpCom for nsINavHistoryQueryOptions {
    const IID: nsIID = nsID(0x8198dfa7, 0x8061, 0x4766,
        [0x95, 0xcb, 0xfa, 0x86, 0xb3, 0xc0, 0x0a, 0x47]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryQueryOptions {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryQueryOptions.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryQueryOptionsCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryQueryOptions`.
    fn coerce_from(v: &nsINavHistoryQueryOptions) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryQueryOptionsCoerce for nsINavHistoryQueryOptions {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryOptions) -> &Self {
        v
    }
}

impl nsINavHistoryQueryOptions {
    /// Cast this `nsINavHistoryQueryOptions` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryQueryOptionsCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryQueryOptions {
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
impl<T: nsISupportsCoerce> nsINavHistoryQueryOptionsCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryQueryOptions) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryQueryOptions
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryQueryOptionsVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute unsigned short sortingMode; */
    pub GetSortingMode: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aSortingMode: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short sortingMode; */
    pub SetSortingMode: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aSortingMode: u16) -> ::nserror::nsresult,

    /* attribute unsigned short resultType; */
    pub GetResultType: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aResultType: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short resultType; */
    pub SetResultType: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aResultType: u16) -> ::nserror::nsresult,

    /* attribute boolean excludeItems; */
    pub GetExcludeItems: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aExcludeItems: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean excludeItems; */
    pub SetExcludeItems: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aExcludeItems: bool) -> ::nserror::nsresult,

    /* attribute boolean excludeQueries; */
    pub GetExcludeQueries: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aExcludeQueries: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean excludeQueries; */
    pub SetExcludeQueries: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aExcludeQueries: bool) -> ::nserror::nsresult,

    /* attribute boolean expandQueries; */
    pub GetExpandQueries: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aExpandQueries: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean expandQueries; */
    pub SetExpandQueries: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aExpandQueries: bool) -> ::nserror::nsresult,

    /* attribute boolean includeHidden; */
    pub GetIncludeHidden: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aIncludeHidden: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean includeHidden; */
    pub SetIncludeHidden: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aIncludeHidden: bool) -> ::nserror::nsresult,

    /* attribute unsigned long maxResults; */
    pub GetMaxResults: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aMaxResults: *mut u32) -> ::nserror::nsresult,

    /* attribute unsigned long maxResults; */
    pub SetMaxResults: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aMaxResults: u32) -> ::nserror::nsresult,

    /* attribute unsigned short queryType; */
    pub GetQueryType: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aQueryType: *mut u16) -> ::nserror::nsresult,

    /* attribute unsigned short queryType; */
    pub SetQueryType: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aQueryType: u16) -> ::nserror::nsresult,

    /* attribute boolean asyncEnabled; */
    pub GetAsyncEnabled: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aAsyncEnabled: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean asyncEnabled; */
    pub SetAsyncEnabled: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, aAsyncEnabled: bool) -> ::nserror::nsresult,

    /* nsINavHistoryQueryOptions clone (); */
    pub Clone: unsafe extern "system" fn (this: *const nsINavHistoryQueryOptions, _retval: *mut *const nsINavHistoryQueryOptions) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryQueryOptions {
    /// ```text
    /// /**
    ///  * This object represents the global options for executing a query.
    ///  */
    /// /**
    ///    * You can ask for the results to be pre-sorted. Since the DB has indices
    ///    * of many items, it can produce sorted results almost for free. These should
    ///    * be self-explanatory.
    ///    *
    ///    * Note: re-sorting is slower, as is sorting by title or when you have a
    ///    * host name.
    ///    *
    ///    * For bookmark items, SORT_BY_NONE means sort by the natural bookmark order.
    ///    */
    /// ```
    ///

    pub const SORT_BY_NONE: i64 = 0;


    pub const SORT_BY_TITLE_ASCENDING: i64 = 1;


    pub const SORT_BY_TITLE_DESCENDING: i64 = 2;


    pub const SORT_BY_DATE_ASCENDING: i64 = 3;


    pub const SORT_BY_DATE_DESCENDING: i64 = 4;


    pub const SORT_BY_URI_ASCENDING: i64 = 5;


    pub const SORT_BY_URI_DESCENDING: i64 = 6;


    pub const SORT_BY_VISITCOUNT_ASCENDING: i64 = 7;


    pub const SORT_BY_VISITCOUNT_DESCENDING: i64 = 8;


    pub const SORT_BY_DATEADDED_ASCENDING: i64 = 11;


    pub const SORT_BY_DATEADDED_DESCENDING: i64 = 12;


    pub const SORT_BY_LASTMODIFIED_ASCENDING: i64 = 13;


    pub const SORT_BY_LASTMODIFIED_DESCENDING: i64 = 14;


    pub const SORT_BY_TAGS_ASCENDING: i64 = 17;


    pub const SORT_BY_TAGS_DESCENDING: i64 = 18;


    pub const SORT_BY_FRECENCY_ASCENDING: i64 = 21;


    pub const SORT_BY_FRECENCY_DESCENDING: i64 = 22;

    /// ```text
    /// /**
    ///    * "URI" results, one for each URI visited in the range. Individual result
    ///    * nodes will be of type "URI".
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_URI: i64 = 0;

    /// ```text
    /// /**
    ///    * "Visit" results, with one for each time a page was visited (this will
        ///    * often give you multiple results for one URI). Individual result nodes will
    ///    * have type "Visit"
    ///    *
    ///    * @note This result type is only supported by QUERY_TYPE_HISTORY.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_VISIT: i64 = 1;

    /// ```text
    /// /**
    ///    * This returns query nodes for each predefined date range where we
    ///    * had visits. The node contains information how to load its content:
    ///    * - visits for the given date range will be loaded.
    ///    *
    ///    * @note This result type is only supported by QUERY_TYPE_HISTORY.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_DATE_QUERY: i64 = 3;

    /// ```text
    /// /**
    ///    * This returns nsINavHistoryQueryResultNode nodes for each site where we
    ///    * have visits. The node contains information how to load its content:
    ///    * - last visit for each url in the given host will be loaded.
    ///    *
    ///    * @note This result type is only supported by QUERY_TYPE_HISTORY.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_SITE_QUERY: i64 = 4;

    /// ```text
    /// /**
    ///    * This returns nsINavHistoryQueryResultNode nodes for each day where we
    ///    * have visits. The node contains information how to load its content:
    ///    * - list of hosts visited in the given period will be loaded.
    ///    *
    ///    * @note This result type is only supported by QUERY_TYPE_HISTORY.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_DATE_SITE_QUERY: i64 = 5;

    /// ```text
    /// /**
    ///    * This returns nsINavHistoryQueryResultNode nodes for each tag.
    ///    * The node contains information how to load its content:
    ///    * - list of bookmarks with the given tag will be loaded.
    ///    *
    ///    * @note Setting this resultType will force queryType to QUERY_TYPE_BOOKMARKS.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_TAGS_ROOT: i64 = 6;

    /// ```text
    /// /**
    ///    * DEPRECATED: This exists for Sync and also to avoid reusing this number.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_TAG_CONTENTS: i64 = 7;

    /// ```text
    /// /**
    ///    * This returns nsINavHistoryQueryResultNode nodes for each top-level bookmark
    ///    * root.
    ///    *
    ///    * @note Setting this resultType will force queryType to QUERY_TYPE_BOOKMARKS.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_ROOTS_QUERY: i64 = 8;

    /// ```text
    /// /**
    ///    * This returns nsINavHistoryQueryResultNode for each left-pane root.
    ///    */
    /// ```
    ///

    pub const RESULTS_AS_LEFT_PANE_QUERY: i64 = 9;


    pub const QUERY_TYPE_HISTORY: i64 = 0;


    pub const QUERY_TYPE_BOOKMARKS: i64 = 1;


    pub const QUERY_TYPE_UNIFIED: i64 = 2;

    /// ```text
    /// /**
    ///    * The sorting mode to be used for this query.
    ///    * mode is one of SORT_BY_*
    ///    */
    /// ```
    ///

    /// `attribute unsigned short sortingMode;`
    #[inline]
    pub unsafe fn GetSortingMode(&self, aSortingMode: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetSortingMode)(self, aSortingMode)
    }


    /// ```text
    /// /**
    ///    * The sorting mode to be used for this query.
    ///    * mode is one of SORT_BY_*
    ///    */
    /// ```
    ///

    /// `attribute unsigned short sortingMode;`
    #[inline]
    pub unsafe fn SetSortingMode(&self, aSortingMode: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetSortingMode)(self, aSortingMode)
    }


    /// ```text
    /// /**
    ///    * Sets the result type. One of RESULT_TYPE_* which includes how URIs are
    ///    * represented.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short resultType;`
    #[inline]
    pub unsafe fn GetResultType(&self, aResultType: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetResultType)(self, aResultType)
    }


    /// ```text
    /// /**
    ///    * Sets the result type. One of RESULT_TYPE_* which includes how URIs are
    ///    * represented.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short resultType;`
    #[inline]
    pub unsafe fn SetResultType(&self, aResultType: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetResultType)(self, aResultType)
    }


    /// ```text
    /// /**
    ///    * This option excludes all URIs and separators from a bookmarks query.
    ///    * This would be used if you just wanted a list of bookmark folders and
    ///    * queries (such as the left pane of the places page).
    ///    * Defaults to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean excludeItems;`
    #[inline]
    pub unsafe fn GetExcludeItems(&self, aExcludeItems: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetExcludeItems)(self, aExcludeItems)
    }


    /// ```text
    /// /**
    ///    * This option excludes all URIs and separators from a bookmarks query.
    ///    * This would be used if you just wanted a list of bookmark folders and
    ///    * queries (such as the left pane of the places page).
    ///    * Defaults to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean excludeItems;`
    #[inline]
    pub unsafe fn SetExcludeItems(&self, aExcludeItems: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetExcludeItems)(self, aExcludeItems)
    }


    /// ```text
    /// /**
    ///    * Set to true to exclude queries ("place:" URIs) from the query results.
    ///    * Simple folder queries (bookmark folder symlinks) will still be included.
    ///    * Defaults to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean excludeQueries;`
    #[inline]
    pub unsafe fn GetExcludeQueries(&self, aExcludeQueries: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetExcludeQueries)(self, aExcludeQueries)
    }


    /// ```text
    /// /**
    ///    * Set to true to exclude queries ("place:" URIs) from the query results.
    ///    * Simple folder queries (bookmark folder symlinks) will still be included.
    ///    * Defaults to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean excludeQueries;`
    #[inline]
    pub unsafe fn SetExcludeQueries(&self, aExcludeQueries: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetExcludeQueries)(self, aExcludeQueries)
    }


    /// ```text
    /// /**
    ///    * When set, allows items with "place:" URIs to appear as containers,
    ///    * with the container's contents filled in from the stored query.
    ///    * If not set, these will appear as normal items. Doesn't do anything if
    ///    * excludeQueries is set. Defaults to false.
    ///    *
    ///    * Note that this has no effect on folder links, which are place: URIs
    ///    * returned by nsINavBookmarkService.GetFolderURI. These are always expanded
    ///    * and will appear as bookmark folders.
    ///    */
    /// ```
    ///

    /// `attribute boolean expandQueries;`
    #[inline]
    pub unsafe fn GetExpandQueries(&self, aExpandQueries: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetExpandQueries)(self, aExpandQueries)
    }


    /// ```text
    /// /**
    ///    * When set, allows items with "place:" URIs to appear as containers,
    ///    * with the container's contents filled in from the stored query.
    ///    * If not set, these will appear as normal items. Doesn't do anything if
    ///    * excludeQueries is set. Defaults to false.
    ///    *
    ///    * Note that this has no effect on folder links, which are place: URIs
    ///    * returned by nsINavBookmarkService.GetFolderURI. These are always expanded
    ///    * and will appear as bookmark folders.
    ///    */
    /// ```
    ///

    /// `attribute boolean expandQueries;`
    #[inline]
    pub unsafe fn SetExpandQueries(&self, aExpandQueries: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetExpandQueries)(self, aExpandQueries)
    }


    /// ```text
    /// /**
    ///    * Some pages in history are marked "hidden" and thus don't appear by default
    ///    * in queries.  These include automatic framed visits and redirects.  Setting
    ///    * this attribute will return all pages, even hidden ones.  Does nothing for
    ///    * bookmark queries. Defaults to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean includeHidden;`
    #[inline]
    pub unsafe fn GetIncludeHidden(&self, aIncludeHidden: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIncludeHidden)(self, aIncludeHidden)
    }


    /// ```text
    /// /**
    ///    * Some pages in history are marked "hidden" and thus don't appear by default
    ///    * in queries.  These include automatic framed visits and redirects.  Setting
    ///    * this attribute will return all pages, even hidden ones.  Does nothing for
    ///    * bookmark queries. Defaults to false.
    ///    */
    /// ```
    ///

    /// `attribute boolean includeHidden;`
    #[inline]
    pub unsafe fn SetIncludeHidden(&self, aIncludeHidden: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetIncludeHidden)(self, aIncludeHidden)
    }


    /// ```text
    /// /**
    ///    * This is the maximum number of results that you want. The query is executed,
    ///    * the results are sorted, and then the top 'maxResults' results are taken
    ///    * and returned. Set to 0 (the default) to get all results.
    ///    *
    ///    * THIS DOES NOT WORK IN CONJUNCTION WITH SORTING BY TITLE. This is because
    ///    * sorting by title requires us to sort after using locale-sensetive sorting
    ///    * (as opposed to letting the database do it for us).
    ///    *
    ///    * Instead, we get the result ordered by date, pick the maxResult most recent
    ///    * ones, and THEN sort by title.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long maxResults;`
    #[inline]
    pub unsafe fn GetMaxResults(&self, aMaxResults: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxResults)(self, aMaxResults)
    }


    /// ```text
    /// /**
    ///    * This is the maximum number of results that you want. The query is executed,
    ///    * the results are sorted, and then the top 'maxResults' results are taken
    ///    * and returned. Set to 0 (the default) to get all results.
    ///    *
    ///    * THIS DOES NOT WORK IN CONJUNCTION WITH SORTING BY TITLE. This is because
    ///    * sorting by title requires us to sort after using locale-sensetive sorting
    ///    * (as opposed to letting the database do it for us).
    ///    *
    ///    * Instead, we get the result ordered by date, pick the maxResult most recent
    ///    * ones, and THEN sort by title.
    ///    */
    /// ```
    ///

    /// `attribute unsigned long maxResults;`
    #[inline]
    pub unsafe fn SetMaxResults(&self, aMaxResults: u32) -> ::nserror::nsresult {
        ((*self.vtable).SetMaxResults)(self, aMaxResults)
    }


    /// ```text
    /// /**
    ///    * The type of search to use when querying the DB; This attribute is only
    ///    * honored by query nodes. It is silently ignored for simple folder queries.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short queryType;`
    #[inline]
    pub unsafe fn GetQueryType(&self, aQueryType: *mut u16) -> ::nserror::nsresult {
        ((*self.vtable).GetQueryType)(self, aQueryType)
    }


    /// ```text
    /// /**
    ///    * The type of search to use when querying the DB; This attribute is only
    ///    * honored by query nodes. It is silently ignored for simple folder queries.
    ///    */
    /// ```
    ///

    /// `attribute unsigned short queryType;`
    #[inline]
    pub unsafe fn SetQueryType(&self, aQueryType: u16) -> ::nserror::nsresult {
        ((*self.vtable).SetQueryType)(self, aQueryType)
    }


    /// ```text
    /// /**
    ///    * When this is true, the root container node generated by these options and
    ///    * its descendant containers will be opened asynchronously if they support it.
    ///    * This is false by default.
    ///    *
    ///    * @note Currently only bookmark folder containers support being opened
    ///    *       asynchronously.
    ///    */
    /// ```
    ///

    /// `attribute boolean asyncEnabled;`
    #[inline]
    pub unsafe fn GetAsyncEnabled(&self, aAsyncEnabled: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetAsyncEnabled)(self, aAsyncEnabled)
    }


    /// ```text
    /// /**
    ///    * When this is true, the root container node generated by these options and
    ///    * its descendant containers will be opened asynchronously if they support it.
    ///    * This is false by default.
    ///    *
    ///    * @note Currently only bookmark folder containers support being opened
    ///    *       asynchronously.
    ///    */
    /// ```
    ///

    /// `attribute boolean asyncEnabled;`
    #[inline]
    pub unsafe fn SetAsyncEnabled(&self, aAsyncEnabled: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetAsyncEnabled)(self, aAsyncEnabled)
    }


    /// ```text
    /// /**
    ///    * Creates a new options item with the same parameters of this one.
    ///    */
    /// ```
    ///

    /// `nsINavHistoryQueryOptions clone ();`
    #[inline]
    pub unsafe fn Clone(&self, _retval: *mut *const nsINavHistoryQueryOptions) -> ::nserror::nsresult {
        ((*self.vtable).Clone)(self, _retval)
    }


}


/// `interface nsINavHistoryService : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsINavHistoryService {
    vtable: *const nsINavHistoryServiceVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsINavHistoryService.
unsafe impl XpCom for nsINavHistoryService {
    const IID: nsIID = nsID(0x20c974ff, 0xee16, 0x4828,
        [0x93, 0x26, 0x1b, 0x7c, 0x9e, 0x03, 0x66, 0x22]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsINavHistoryService {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsINavHistoryService.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsINavHistoryServiceCoerce {
    /// Cheaply cast a value of this type from a `nsINavHistoryService`.
    fn coerce_from(v: &nsINavHistoryService) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsINavHistoryServiceCoerce for nsINavHistoryService {
    #[inline]
    fn coerce_from(v: &nsINavHistoryService) -> &Self {
        v
    }
}

impl nsINavHistoryService {
    /// Cast this `nsINavHistoryService` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsINavHistoryServiceCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsINavHistoryService {
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
impl<T: nsISupportsCoerce> nsINavHistoryServiceCoerce for T {
    #[inline]
    fn coerce_from(v: &nsINavHistoryService) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsINavHistoryService
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsINavHistoryServiceVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* readonly attribute unsigned short databaseStatus; */
    pub GetDatabaseStatus: unsafe extern "system" fn (this: *const nsINavHistoryService, aDatabaseStatus: *mut u16) -> ::nserror::nsresult,

    /* void markPageAsFollowedBookmark (in nsIURI aURI); */
    pub MarkPageAsFollowedBookmark: unsafe extern "system" fn (this: *const nsINavHistoryService, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* void markPageAsTyped (in nsIURI aURI); */
    pub MarkPageAsTyped: unsafe extern "system" fn (this: *const nsINavHistoryService, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* void markPageAsFollowedLink (in nsIURI aURI); */
    pub MarkPageAsFollowedLink: unsafe extern "system" fn (this: *const nsINavHistoryService, aURI: *const nsIURI) -> ::nserror::nsresult,

    /* boolean canAddURI (in nsIURI aURI); */
    pub CanAddURI: unsafe extern "system" fn (this: *const nsINavHistoryService, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult,

    /* nsINavHistoryQuery getNewQuery (); */
    pub GetNewQuery: unsafe extern "system" fn (this: *const nsINavHistoryService, _retval: *mut *const nsINavHistoryQuery) -> ::nserror::nsresult,

    /* nsINavHistoryQueryOptions getNewQueryOptions (); */
    pub GetNewQueryOptions: unsafe extern "system" fn (this: *const nsINavHistoryService, _retval: *mut *const nsINavHistoryQueryOptions) -> ::nserror::nsresult,

    /* nsINavHistoryResult executeQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
    pub ExecuteQuery: unsafe extern "system" fn (this: *const nsINavHistoryService, aQuery: *const nsINavHistoryQuery, options: *const nsINavHistoryQueryOptions, _retval: *mut *const nsINavHistoryResult) -> ::nserror::nsresult,

    /* void queryStringToQuery (in AUTF8String aQueryString, out nsINavHistoryQuery aQuery, out nsINavHistoryQueryOptions options); */
    pub QueryStringToQuery: unsafe extern "system" fn (this: *const nsINavHistoryService, aQueryString: *const ::nsstring::nsACString, aQuery: *mut *const nsINavHistoryQuery, options: *mut *const nsINavHistoryQueryOptions) -> ::nserror::nsresult,

    /* AUTF8String queryToQueryString (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options); */
    pub QueryToQueryString: unsafe extern "system" fn (this: *const nsINavHistoryService, aQuery: *const nsINavHistoryQuery, options: *const nsINavHistoryQueryOptions, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* void addObserver (in nsINavHistoryObserver observer, [optional] in boolean ownsWeak); */
    pub AddObserver: unsafe extern "system" fn (this: *const nsINavHistoryService, observer: *const nsINavHistoryObserver, ownsWeak: bool) -> ::nserror::nsresult,

    /* void removeObserver (in nsINavHistoryObserver observer); */
    pub RemoveObserver: unsafe extern "system" fn (this: *const nsINavHistoryService, observer: *const nsINavHistoryObserver) -> ::nserror::nsresult,

    /* Array<nsINavHistoryObserver> getObservers (); */
    pub GetObservers: unsafe extern "system" fn (this: *const nsINavHistoryService, _retval: *mut thin_vec::ThinVec<RefPtr<nsINavHistoryObserver>>) -> ::nserror::nsresult,

    /* readonly attribute boolean historyDisabled; */
    pub GetHistoryDisabled: unsafe extern "system" fn (this: *const nsINavHistoryService, aHistoryDisabled: *mut bool) -> ::nserror::nsresult,

    /* ACString makeGuid (); */
    pub MakeGuid: unsafe extern "system" fn (this: *const nsINavHistoryService, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult,

    /* unsigned long long hashURL (in ACString aSpec, [optional] in ACString aMode); */
    pub HashURL: unsafe extern "system" fn (this: *const nsINavHistoryService, aSpec: *const ::nsstring::nsACString, aMode: *const ::nsstring::nsACString, _retval: *mut u64) -> ::nserror::nsresult,

    /* void recalculateOriginFrecencyStats ([optional] in nsIObserver aCallback); */
    pub RecalculateOriginFrecencyStats: unsafe extern "system" fn (this: *const nsINavHistoryService, aCallback: *const nsIObserver) -> ::nserror::nsresult,

    /* readonly attribute mozIStorageConnection DBConnection; */
    pub GetDBConnection: unsafe extern "system" fn (this: *const nsINavHistoryService, aDBConnection: *mut*const mozIStorageConnection) -> ::nserror::nsresult,

    /* mozIStoragePendingStatement asyncExecuteLegacyQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions aOptions, in mozIStorageStatementCallback aCallback); */
    pub AsyncExecuteLegacyQuery: unsafe extern "system" fn (this: *const nsINavHistoryService, aQuery: *const nsINavHistoryQuery, aOptions: *const nsINavHistoryQueryOptions, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient shutdownClient; */
    pub GetShutdownClient: unsafe extern "system" fn (this: *const nsINavHistoryService, aShutdownClient: *mut*const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* readonly attribute nsIAsyncShutdownClient connectionShutdownClient; */
    pub GetConnectionShutdownClient: unsafe extern "system" fn (this: *const nsINavHistoryService, aConnectionShutdownClient: *mut*const nsIAsyncShutdownClient) -> ::nserror::nsresult,

    /* void decayFrecency (); */
    pub DecayFrecency: unsafe extern "system" fn (this: *const nsINavHistoryService) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsINavHistoryService {
    /// ```text
    /// /**
    ///    * System Notifications:
    ///    *
    ///    * places-init-complete - Sent once the History service is completely
    ///    *                        initialized successfully.
    ///    * places-database-locked - Sent if initialization of the History service
    ///    *                          failed due to the inability to open the places.sqlite
    ///    *                          for access reasons.
    ///    */
    /// /**
    ///    * This transition type means the user followed a link and got a new toplevel
    ///    * window.
    ///    */
    /// ```
    ///

    pub const TRANSITION_LINK: i64 = 1;

    /// ```text
    /// /**
    ///    * This transition type means that the user typed the page's URL in the
    ///    * URL bar or selected it from URL bar autocomplete results, clicked on
    ///    * it from a history query (from the History sidebar, History menu,
        ///    * or history query in the personal toolbar or Places organizer.
        ///    */
        /// ```
        ///

        pub const TRANSITION_TYPED: i64 = 2;

        /// ```text
        /// /**
        ///    * This transition is set when the user followed a bookmark to get to the
        ///    * page.
        ///    */
        /// ```
        ///

        pub const TRANSITION_BOOKMARK: i64 = 3;

        /// ```text
        /// /**
        ///    * This transition type is set when some inner content is loaded. This is
        ///    * true of all images on a page, and the contents of the iframe. It is also
        ///    * true of any content in a frame if the user did not explicitly follow
        ///    * a link to get there.
        ///    */
        /// ```
        ///

        pub const TRANSITION_EMBED: i64 = 4;

        /// ```text
        /// /**
        ///    * Set when the transition was a permanent redirect.
        ///    */
        /// ```
        ///

        pub const TRANSITION_REDIRECT_PERMANENT: i64 = 5;

        /// ```text
        /// /**
        ///    * Set when the transition was a temporary redirect.
        ///    */
        /// ```
        ///

        pub const TRANSITION_REDIRECT_TEMPORARY: i64 = 6;

        /// ```text
        /// /**
        ///    * Set when the transition is a download.
        ///    */
        /// ```
        ///

        pub const TRANSITION_DOWNLOAD: i64 = 7;

        /// ```text
        /// /**
        ///    * This transition type means the user followed a link and got a visit in
        ///    * a frame.
        ///    */
        /// ```
        ///

        pub const TRANSITION_FRAMED_LINK: i64 = 8;

        /// ```text
        /// /**
        ///    * This transition type means the page has been reloaded.
        ///    */
        /// ```
        ///

        pub const TRANSITION_RELOAD: i64 = 9;

        /// ```text
        /// /**
        ///    * Set when database is coherent
        ///    */
        /// ```
        ///

        pub const DATABASE_STATUS_OK: i64 = 0;

        /// ```text
        /// /**
        ///    * Set when database did not exist and we created a new one.
        ///    */
        /// ```
        ///

        pub const DATABASE_STATUS_CREATE: i64 = 1;

        /// ```text
        /// /**
        ///    * Set when database was corrupt and we replaced it with a new one.
        ///    */
        /// ```
        ///

        pub const DATABASE_STATUS_CORRUPT: i64 = 2;

        /// ```text
        /// /**
        ///    * Set when database schema has been upgraded.
        ///    */
        /// ```
        ///

        pub const DATABASE_STATUS_UPGRADED: i64 = 3;

        /// ```text
        /// /**
        ///    * Set when database couldn't be opened.
        ///    */
        /// ```
        ///

        pub const DATABASE_STATUS_LOCKED: i64 = 4;

        /// ```text
        /// /**
        ///    * Returns the current database status
        ///    */
        /// ```
        ///

        /// `readonly attribute unsigned short databaseStatus;`
        #[inline]
        pub unsafe fn GetDatabaseStatus(&self, aDatabaseStatus: *mut u16) -> ::nserror::nsresult {
            ((*self.vtable).GetDatabaseStatus)(self, aDatabaseStatus)
        }


        /// ```text
        /// /**
        ///    * This is just like markPageAsTyped (in nsIBrowserHistory, also implemented
            ///    * by the history service), but for bookmarks. It declares that the given URI
        ///    * is being opened as a result of following a bookmark. If this URI is loaded
        ///    * soon after this message has been received, that transition will be marked
        ///    * as following a bookmark.
        ///    */
        /// ```
        ///

        /// `void markPageAsFollowedBookmark (in nsIURI aURI);`
        #[inline]
        pub unsafe fn MarkPageAsFollowedBookmark(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
            ((*self.vtable).MarkPageAsFollowedBookmark)(self, aURI)
        }


        /// ```text
        /// /**
        ///    * Designates the url as having been explicitly typed in by the user.
        ///    *
        ///    * @param aURI
        ///    *        URI of the page to be marked.
        ///    */
        /// ```
        ///

        /// `void markPageAsTyped (in nsIURI aURI);`
        #[inline]
        pub unsafe fn MarkPageAsTyped(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
            ((*self.vtable).MarkPageAsTyped)(self, aURI)
        }


        /// ```text
        /// /**
        ///    * Designates the url as coming from a link explicitly followed by
        ///    * the user (for example by clicking on it).
        ///    *
        ///    * @param aURI
        ///    *        URI of the page to be marked.
        ///    */
        /// ```
        ///

        /// `void markPageAsFollowedLink (in nsIURI aURI);`
        #[inline]
        pub unsafe fn MarkPageAsFollowedLink(&self, aURI: *const nsIURI) -> ::nserror::nsresult {
            ((*self.vtable).MarkPageAsFollowedLink)(self, aURI)
        }


        /// ```text
        /// /**
        ///    * Returns true if this URI would be added to the history. You don't have to
        ///    * worry about calling this, adding a visit will always check before
        ///    * actually adding the page. This function is public because some components
        ///    * may want to check if this page would go in the history (i.e. for
            ///    * annotations).
        ///    */
        /// ```
        ///

        /// `boolean canAddURI (in nsIURI aURI);`
        #[inline]
        pub unsafe fn CanAddURI(&self, aURI: *const nsIURI, _retval: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).CanAddURI)(self, aURI, _retval)
        }


        /// ```text
        /// /**
        ///    * This returns a new query object that you can pass to executeQuer[y/ies].
        ///    * It will be initialized to all empty (so using it will give you all history).
        ///    */
        /// ```
        ///

        /// `nsINavHistoryQuery getNewQuery ();`
        #[inline]
        pub unsafe fn GetNewQuery(&self, _retval: *mut *const nsINavHistoryQuery) -> ::nserror::nsresult {
            ((*self.vtable).GetNewQuery)(self, _retval)
        }


        /// ```text
        /// /**
        ///    * This returns a new options object that you can pass to executeQuer[y/ies]
        ///    * after setting the desired options.
        ///    */
        /// ```
        ///

        /// `nsINavHistoryQueryOptions getNewQueryOptions ();`
        #[inline]
        pub unsafe fn GetNewQueryOptions(&self, _retval: *mut *const nsINavHistoryQueryOptions) -> ::nserror::nsresult {
            ((*self.vtable).GetNewQueryOptions)(self, _retval)
        }


        /// ```text
        /// /**
        ///    * Executes a single query.
        ///    */
        /// ```
        ///

        /// `nsINavHistoryResult executeQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options);`
        #[inline]
        pub unsafe fn ExecuteQuery(&self, aQuery: *const nsINavHistoryQuery, options: *const nsINavHistoryQueryOptions, _retval: *mut *const nsINavHistoryResult) -> ::nserror::nsresult {
            ((*self.vtable).ExecuteQuery)(self, aQuery, options, _retval)
        }


        /// ```text
        /// /**
        ///    * Converts a query URI-like string to a query object.
        ///    */
        /// ```
        ///

        /// `void queryStringToQuery (in AUTF8String aQueryString, out nsINavHistoryQuery aQuery, out nsINavHistoryQueryOptions options);`
        #[inline]
        pub unsafe fn QueryStringToQuery(&self, aQueryString: *const ::nsstring::nsACString, aQuery: *mut *const nsINavHistoryQuery, options: *mut *const nsINavHistoryQueryOptions) -> ::nserror::nsresult {
            ((*self.vtable).QueryStringToQuery)(self, aQueryString, aQuery, options)
        }


        /// ```text
        /// /**
        ///    * Converts a query into an equivalent string that can be persisted. Inverse
        ///    * of queryStringToQuery()
        ///    */
        /// ```
        ///

        /// `AUTF8String queryToQueryString (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions options);`
        #[inline]
        pub unsafe fn QueryToQueryString(&self, aQuery: *const nsINavHistoryQuery, options: *const nsINavHistoryQueryOptions, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).QueryToQueryString)(self, aQuery, options, _retval)
        }


        /// ```text
        /// /**
        ///    * Adds a history observer. If ownsWeak is false, the history service will
        ///    * keep an owning reference to the observer.  If ownsWeak is true, then
        ///    * aObserver must implement nsISupportsWeakReference, and the history service
        ///    * will keep a weak reference to the observer.
        ///    */
        /// ```
        ///

        /// `void addObserver (in nsINavHistoryObserver observer, [optional] in boolean ownsWeak);`
        #[inline]
        pub unsafe fn AddObserver(&self, observer: *const nsINavHistoryObserver, ownsWeak: bool) -> ::nserror::nsresult {
            ((*self.vtable).AddObserver)(self, observer, ownsWeak)
        }


        /// ```text
        /// /**
        ///    * Removes a history observer.
        ///    */
        /// ```
        ///

        /// `void removeObserver (in nsINavHistoryObserver observer);`
        #[inline]
        pub unsafe fn RemoveObserver(&self, observer: *const nsINavHistoryObserver) -> ::nserror::nsresult {
            ((*self.vtable).RemoveObserver)(self, observer)
        }


        /// ```text
        /// /**
        ///    * Gets an array of registered nsINavHistoryObserver objects.
        ///    */
        /// ```
        ///

        /// `Array<nsINavHistoryObserver> getObservers ();`
        #[inline]
        pub unsafe fn GetObservers(&self, _retval: *mut thin_vec::ThinVec<RefPtr<nsINavHistoryObserver>>) -> ::nserror::nsresult {
            ((*self.vtable).GetObservers)(self, _retval)
        }


        /// ```text
        /// /**
        ///    * True if history is disabled. currently,
        ///    * history is disabled if the places.history.enabled pref is false.
        ///    */
        /// ```
        ///

        /// `readonly attribute boolean historyDisabled;`
        #[inline]
        pub unsafe fn GetHistoryDisabled(&self, aHistoryDisabled: *mut bool) -> ::nserror::nsresult {
            ((*self.vtable).GetHistoryDisabled)(self, aHistoryDisabled)
        }


        /// ```text
        /// /**
        ///    * Generate a guid.
        ///    * Guids can be used for any places purposes (history, bookmarks, etc.)
        ///    * Returns null if the generation of the guid failed.
        ///    */
        /// ```
        ///

        /// `ACString makeGuid ();`
        #[inline]
        pub unsafe fn MakeGuid(&self, _retval: *mut ::nsstring::nsACString) -> ::nserror::nsresult {
            ((*self.vtable).MakeGuid)(self, _retval)
        }


        /// ```text
        /// /**
        ///    * Returns a 48-bit hash for a URI spec.
        ///    *
        ///    * @param aSpec
        ///    *        The URI spec to hash.
        ///    * @param aMode
        ///    *        The hash mode: `""` (default), `"prefix_lo"`, or `"prefix_hi"`.
        ///    */
        /// ```
        ///

        /// `unsigned long long hashURL (in ACString aSpec, [optional] in ACString aMode);`
        #[inline]
        pub unsafe fn HashURL(&self, aSpec: *const ::nsstring::nsACString, aMode: *const ::nsstring::nsACString, _retval: *mut u64) -> ::nserror::nsresult {
            ((*self.vtable).HashURL)(self, aSpec, aMode, _retval)
        }


        /// ```text
        /// /**
        ///    * Resets and recalculates the origin frecency statistics that are kept in the
        ///    * moz_meta table.
        ///    *
        ///    * @param aCallback
        ///    *        Called when the recalculation is complete.  The arguments passed to
        ///    *        the observer are not defined.
        ///    */
        /// ```
        ///

        /// `void recalculateOriginFrecencyStats ([optional] in nsIObserver aCallback);`
        #[inline]
        pub unsafe fn RecalculateOriginFrecencyStats(&self, aCallback: *const nsIObserver) -> ::nserror::nsresult {
            ((*self.vtable).RecalculateOriginFrecencyStats)(self, aCallback)
        }


        /// ```text
        /// /**
        ///    * The database connection used by Places.
        ///    */
        /// ```
        ///

        /// `readonly attribute mozIStorageConnection DBConnection;`
        #[inline]
        pub unsafe fn GetDBConnection(&self, aDBConnection: *mut*const mozIStorageConnection) -> ::nserror::nsresult {
            ((*self.vtable).GetDBConnection)(self, aDBConnection)
        }


        /// ```text
        /// /**
        ///    * Asynchronously executes the statement created from a query.
        ///    *
        ///    * @see nsINavHistoryService::executeQuery
        ///    * @note THIS IS A TEMPORARY API.  Don't rely on it, since it will be replaced
        ///    *       in future versions by a real async querying API.
        ///    * @note Results obtained from this method differ from results obtained from
        ///    *       executeQuery, because there is additional filtering and sorting
        ///    *       done by the latter.  Thus you should use executeQuery, unless you
        ///    *       are absolutely sure that the returned results are fine for
        ///    *       your use-case.
        ///    */
        /// ```
        ///

        /// `mozIStoragePendingStatement asyncExecuteLegacyQuery (in nsINavHistoryQuery aQuery, in nsINavHistoryQueryOptions aOptions, in mozIStorageStatementCallback aCallback);`
        #[inline]
        pub unsafe fn AsyncExecuteLegacyQuery(&self, aQuery: *const nsINavHistoryQuery, aOptions: *const nsINavHistoryQueryOptions, aCallback: *const mozIStorageStatementCallback, _retval: *mut*const mozIStoragePendingStatement) -> ::nserror::nsresult {
            ((*self.vtable).AsyncExecuteLegacyQuery)(self, aQuery, aOptions, aCallback, _retval)
        }


        /// ```text
        /// /**
        ///    * Hook for clients who need to perform actions during/by the end of
        ///    * the shutdown of the database.
        ///    * May be null if it's too late to get one.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsIAsyncShutdownClient shutdownClient;`
        #[inline]
        pub unsafe fn GetShutdownClient(&self, aShutdownClient: *mut*const nsIAsyncShutdownClient) -> ::nserror::nsresult {
            ((*self.vtable).GetShutdownClient)(self, aShutdownClient)
        }


        /// ```text
        /// /**
        ///    * Hook for internal clients who need to perform actions just before the
        ///    * connection gets closed.
        ///    * May be null if it's too late to get one.
        ///    */
        /// ```
        ///

        /// `readonly attribute nsIAsyncShutdownClient connectionShutdownClient;`
        #[inline]
        pub unsafe fn GetConnectionShutdownClient(&self, aConnectionShutdownClient: *mut*const nsIAsyncShutdownClient) -> ::nserror::nsresult {
            ((*self.vtable).GetConnectionShutdownClient)(self, aConnectionShutdownClient)
        }


        /// ```text
        /// /**
        ///    * Asynchronously recalculates frecency for all pages where frecency < 0, then
        ///    * decays frecency and inputhistory values.
        ///    */
        /// ```
        ///

        /// `void decayFrecency ();`
        #[inline]
        pub unsafe fn DecayFrecency(&self, ) -> ::nserror::nsresult {
            ((*self.vtable).DecayFrecency)(self, )
        }


    }


