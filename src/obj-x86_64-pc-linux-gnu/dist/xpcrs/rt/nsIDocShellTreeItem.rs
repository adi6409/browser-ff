//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShellTreeItem.idl
//


/// `interface nsIDocShellTreeItem : nsISupports`
///

/// ```text
/// /**
///  * The nsIDocShellTreeItem supplies the methods that are required of any item
///  * that wishes to be able to live within the docshell tree either as a middle
///  * node or a leaf.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocShellTreeItem {
    vtable: *const nsIDocShellTreeItemVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocShellTreeItem.
unsafe impl XpCom for nsIDocShellTreeItem {
    const IID: nsIID = nsID(0x9b7c586f, 0x9214, 0x480c,
        [0xa2, 0xc4, 0x49, 0xb5, 0x26, 0xff, 0xf1, 0xa6]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocShellTreeItem {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocShellTreeItem.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocShellTreeItemCoerce {
    /// Cheaply cast a value of this type from a `nsIDocShellTreeItem`.
    fn coerce_from(v: &nsIDocShellTreeItem) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocShellTreeItemCoerce for nsIDocShellTreeItem {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeItem) -> &Self {
        v
    }
}

impl nsIDocShellTreeItem {
    /// Cast this `nsIDocShellTreeItem` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocShellTreeItemCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocShellTreeItem {
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
impl<T: nsISupportsCoerce> nsIDocShellTreeItemCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeItem) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocShellTreeItem
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocShellTreeItemVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute AString name; */
    pub GetName: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult,

    /* attribute AString name; */
    pub SetName: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult,

    /* boolean nameEquals (in AString name); */
    pub NameEquals: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, name: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute long itemType; */
    pub GetItemType: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aItemType: *mut i32) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] long ItemType (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub ItemType: *const ::libc::c_void,

    /* [binaryname(InProcessParent)] readonly attribute nsIDocShellTreeItem parent; */
    pub GetInProcessParent: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aParent: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [binaryname(InProcessSameTypeParent)] readonly attribute nsIDocShellTreeItem sameTypeParent; */
    pub GetInProcessSameTypeParent: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aSameTypeParent: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [binaryname(InProcessRootTreeItem)] readonly attribute nsIDocShellTreeItem rootTreeItem; */
    pub GetInProcessRootTreeItem: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aRootTreeItem: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [binaryname(InProcessSameTypeRootTreeItem)] readonly attribute nsIDocShellTreeItem sameTypeRootTreeItem; */
    pub GetInProcessSameTypeRootTreeItem: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aSameTypeRootTreeItem: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* readonly attribute nsIDocShellTreeOwner treeOwner; */
    pub GetTreeOwner: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aTreeOwner: *mut*const nsIDocShellTreeOwner) -> ::nserror::nsresult,

    /* [noscript] void setTreeOwner (in nsIDocShellTreeOwner treeOwner); */
    pub SetTreeOwner: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, treeOwner: *const nsIDocShellTreeOwner) -> ::nserror::nsresult,

    /* [binaryname(InProcessChildCount),infallible] readonly attribute long childCount; */
    pub GetInProcessChildCount: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aChildCount: *mut i32) -> ::nserror::nsresult,

    /* [noscript] void addChild (in nsIDocShellTreeItem child); */
    pub AddChild: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, child: *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [noscript] void removeChild (in nsIDocShellTreeItem child); */
    pub RemoveChild: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, child: *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [binaryname(GetInProcessChildAt)] nsIDocShellTreeItem getChildAt (in long index); */
    pub GetInProcessChildAt: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, index: i32, _retval: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* [binaryname(BrowsingContextXPCOM)] readonly attribute BrowsingContext browsingContext; */
    pub GetBrowsingContextXPCOM: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] BrowsingContext getBrowsingContext (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetBrowsingContext: *const ::libc::c_void,

    /* readonly attribute mozIDOMWindowProxy domWindow; */
    pub GetDomWindow: unsafe extern "system" fn (this: *const nsIDocShellTreeItem, aDomWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult,

    /* [noscript,nostdcall,notxpcom] Document getDocument (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetDocument: *const ::libc::c_void,

    /* [noscript,nostdcall,notxpcom] nsPIDOMWindowOuter getWindow (); */
    /// Unable to generate binding because `nostdcall is unsupported`
    pub GetWindow: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocShellTreeItem {

    pub const typeChrome: i64 = 0;


    pub const typeContent: i64 = 1;


    pub const typeContentWrapper: i64 = 2;


    pub const typeChromeWrapper: i64 = 3;


    pub const typeAll: i64 = 2147483647;


    /// `attribute AString name;`
    #[inline]
    pub unsafe fn GetName(&self, aName: *mut ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).GetName)(self, aName)
    }



    /// `attribute AString name;`
    #[inline]
    pub unsafe fn SetName(&self, aName: *const ::nsstring::nsAString) -> ::nserror::nsresult {
        ((*self.vtable).SetName)(self, aName)
    }


    /// ```text
    /// /**
    ///          * Compares the provided name against the item's name and
    ///          * returns the appropriate result.
    ///          *
    ///          * @return <CODE>PR_TRUE</CODE> if names match;
    ///          *         <CODE>PR_FALSE</CODE> otherwise.
    ///          */
    /// ```
    ///

    /// `boolean nameEquals (in AString name);`
    #[inline]
    pub unsafe fn NameEquals(&self, name: *const ::nsstring::nsAString, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).NameEquals)(self, name, _retval)
    }



    /// `readonly attribute long itemType;`
    #[inline]
    pub unsafe fn GetItemType(&self, aItemType: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetItemType)(self, aItemType)
    }



    /// `[noscript,nostdcall,notxpcom] long ItemType ();`
    const _ItemType: () = ();


    /// `[binaryname(InProcessParent)] readonly attribute nsIDocShellTreeItem parent;`
    #[inline]
    pub unsafe fn GetInProcessParent(&self, aParent: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetInProcessParent)(self, aParent)
    }



    /// `[binaryname(InProcessSameTypeParent)] readonly attribute nsIDocShellTreeItem sameTypeParent;`
    #[inline]
    pub unsafe fn GetInProcessSameTypeParent(&self, aSameTypeParent: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetInProcessSameTypeParent)(self, aSameTypeParent)
    }



    /// `[binaryname(InProcessRootTreeItem)] readonly attribute nsIDocShellTreeItem rootTreeItem;`
    #[inline]
    pub unsafe fn GetInProcessRootTreeItem(&self, aRootTreeItem: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetInProcessRootTreeItem)(self, aRootTreeItem)
    }



    /// `[binaryname(InProcessSameTypeRootTreeItem)] readonly attribute nsIDocShellTreeItem sameTypeRootTreeItem;`
    #[inline]
    pub unsafe fn GetInProcessSameTypeRootTreeItem(&self, aSameTypeRootTreeItem: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetInProcessSameTypeRootTreeItem)(self, aSameTypeRootTreeItem)
    }



    /// `readonly attribute nsIDocShellTreeOwner treeOwner;`
    #[inline]
    pub unsafe fn GetTreeOwner(&self, aTreeOwner: *mut*const nsIDocShellTreeOwner) -> ::nserror::nsresult {
        ((*self.vtable).GetTreeOwner)(self, aTreeOwner)
    }



    /// `[noscript] void setTreeOwner (in nsIDocShellTreeOwner treeOwner);`
    #[inline]
    pub unsafe fn SetTreeOwner(&self, treeOwner: *const nsIDocShellTreeOwner) -> ::nserror::nsresult {
        ((*self.vtable).SetTreeOwner)(self, treeOwner)
    }



    /// `[binaryname(InProcessChildCount),infallible] readonly attribute long childCount;`
    #[inline]
    pub unsafe fn GetInProcessChildCount(&self) -> i32 {
        let mut result = <i32 as ::std::default::Default>::default();
        let _rv = ((*self.vtable).GetInProcessChildCount)(self, &mut result);
        debug_assert!(_rv.succeeded());
        result
    }



    /// `[noscript] void addChild (in nsIDocShellTreeItem child);`
    #[inline]
    pub unsafe fn AddChild(&self, child: *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).AddChild)(self, child)
    }



    /// `[noscript] void removeChild (in nsIDocShellTreeItem child);`
    #[inline]
    pub unsafe fn RemoveChild(&self, child: *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).RemoveChild)(self, child)
    }


    /// ```text
    /// /**
    /// 	 * Return the child at the index requested.  This is 0-based.
    /// 	 *
    /// 	 * @deprecated: Prefer using `BrowsingContext::Children()`, as this will not
    /// 	 * include out-of-process iframes.
    /// 	 *
    /// 	 * @throws NS_ERROR_UNEXPECTED if the index is out of range
    /// 	 */
    /// ```
    ///

    /// `[binaryname(GetInProcessChildAt)] nsIDocShellTreeItem getChildAt (in long index);`
    #[inline]
    pub unsafe fn GetInProcessChildAt(&self, index: i32, _retval: *mut *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetInProcessChildAt)(self, index, _retval)
    }


    /// ```text
    /// /**
    ///    * BrowsingContext associated with the DocShell.
    ///    */
    /// ```
    ///

    /// `[binaryname(BrowsingContextXPCOM)] readonly attribute BrowsingContext browsingContext;`
    #[inline]
    pub unsafe fn GetBrowsingContextXPCOM(&self, aBrowsingContext: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetBrowsingContextXPCOM)(self, aBrowsingContext)
    }



    /// `[noscript,nostdcall,notxpcom] BrowsingContext getBrowsingContext ();`
    const _GetBrowsingContext: () = ();

    /// ```text
    /// /**
    ///    * Returns the DOM outer window for the content viewer.
    ///    */
    /// ```
    ///

    /// `readonly attribute mozIDOMWindowProxy domWindow;`
    #[inline]
    pub unsafe fn GetDomWindow(&self, aDomWindow: *mut*const mozIDOMWindowProxy) -> ::nserror::nsresult {
        ((*self.vtable).GetDomWindow)(self, aDomWindow)
    }



    /// `[noscript,nostdcall,notxpcom] Document getDocument ();`
    const _GetDocument: () = ();


    /// `[noscript,nostdcall,notxpcom] nsPIDOMWindowOuter getWindow ();`
    const _GetWindow: () = ();

}


