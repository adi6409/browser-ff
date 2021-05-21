//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/layout/inspector/inIDeepTreeWalker.idl
//


/// `interface inIDeepTreeWalker : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct inIDeepTreeWalker {
    vtable: *const inIDeepTreeWalkerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for inIDeepTreeWalker.
unsafe impl XpCom for inIDeepTreeWalker {
    const IID: nsIID = nsID(0x6657e8eb, 0xb646, 0x48e7,
        [0x99, 0x3e, 0xcf, 0xa6, 0xe9, 0x64, 0x15, 0xb4]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for inIDeepTreeWalker {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from inIDeepTreeWalker.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait inIDeepTreeWalkerCoerce {
    /// Cheaply cast a value of this type from a `inIDeepTreeWalker`.
    fn coerce_from(v: &inIDeepTreeWalker) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl inIDeepTreeWalkerCoerce for inIDeepTreeWalker {
    #[inline]
    fn coerce_from(v: &inIDeepTreeWalker) -> &Self {
        v
    }
}

impl inIDeepTreeWalker {
    /// Cast this `inIDeepTreeWalker` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: inIDeepTreeWalkerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for inIDeepTreeWalker {
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
impl<T: nsISupportsCoerce> inIDeepTreeWalkerCoerce for T {
    #[inline]
    fn coerce_from(v: &inIDeepTreeWalker) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every inIDeepTreeWalker
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct inIDeepTreeWalkerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* attribute boolean showAnonymousContent; */
    pub GetShowAnonymousContent: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aShowAnonymousContent: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean showAnonymousContent; */
    pub SetShowAnonymousContent: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aShowAnonymousContent: bool) -> ::nserror::nsresult,

    /* attribute boolean showSubDocuments; */
    pub GetShowSubDocuments: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aShowSubDocuments: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean showSubDocuments; */
    pub SetShowSubDocuments: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aShowSubDocuments: bool) -> ::nserror::nsresult,

    /* attribute boolean showDocumentsAsNodes; */
    pub GetShowDocumentsAsNodes: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aShowDocumentsAsNodes: *mut bool) -> ::nserror::nsresult,

    /* attribute boolean showDocumentsAsNodes; */
    pub SetShowDocumentsAsNodes: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aShowDocumentsAsNodes: bool) -> ::nserror::nsresult,

    /* void init (in Node aRoot, in unsigned long aWhatToShow); */
    pub Init: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aRoot: *const libc::c_void, aWhatToShow: u32) -> ::nserror::nsresult,

    /* readonly attribute Node root; */
    pub GetRoot: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aRoot: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* readonly attribute unsigned long whatToShow; */
    pub GetWhatToShow: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aWhatToShow: *mut u32) -> ::nserror::nsresult,

    /* attribute Node currentNode; */
    pub GetCurrentNode: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aCurrentNode: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* attribute Node currentNode; */
    pub SetCurrentNode: unsafe extern "system" fn (this: *const inIDeepTreeWalker, aCurrentNode: *const libc::c_void) -> ::nserror::nsresult,

    /* Node parentNode (); */
    pub ParentNode: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Node firstChild (); */
    pub FirstChild: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Node lastChild (); */
    pub LastChild: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Node previousSibling (); */
    pub PreviousSibling: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Node nextSibling (); */
    pub NextSibling: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Node previousNode (); */
    pub PreviousNode: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,

    /* Node nextNode (); */
    pub NextNode: unsafe extern "system" fn (this: *const inIDeepTreeWalker, _retval: *mut *const libc::c_void) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl inIDeepTreeWalker {


    /// `attribute boolean showAnonymousContent;`
    #[inline]
    pub unsafe fn GetShowAnonymousContent(&self, aShowAnonymousContent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetShowAnonymousContent)(self, aShowAnonymousContent)
    }



    /// `attribute boolean showAnonymousContent;`
    #[inline]
    pub unsafe fn SetShowAnonymousContent(&self, aShowAnonymousContent: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShowAnonymousContent)(self, aShowAnonymousContent)
    }



    /// `attribute boolean showSubDocuments;`
    #[inline]
    pub unsafe fn GetShowSubDocuments(&self, aShowSubDocuments: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetShowSubDocuments)(self, aShowSubDocuments)
    }



    /// `attribute boolean showSubDocuments;`
    #[inline]
    pub unsafe fn SetShowSubDocuments(&self, aShowSubDocuments: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShowSubDocuments)(self, aShowSubDocuments)
    }



    /// `attribute boolean showDocumentsAsNodes;`
    #[inline]
    pub unsafe fn GetShowDocumentsAsNodes(&self, aShowDocumentsAsNodes: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetShowDocumentsAsNodes)(self, aShowDocumentsAsNodes)
    }



    /// `attribute boolean showDocumentsAsNodes;`
    #[inline]
    pub unsafe fn SetShowDocumentsAsNodes(&self, aShowDocumentsAsNodes: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetShowDocumentsAsNodes)(self, aShowDocumentsAsNodes)
    }



    /// `void init (in Node aRoot, in unsigned long aWhatToShow);`
    #[inline]
    pub unsafe fn Init(&self, aRoot: *const libc::c_void, aWhatToShow: u32) -> ::nserror::nsresult {
        ((*self.vtable).Init)(self, aRoot, aWhatToShow)
    }



    /// `readonly attribute Node root;`
    #[inline]
    pub unsafe fn GetRoot(&self, aRoot: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetRoot)(self, aRoot)
    }



    /// `readonly attribute unsigned long whatToShow;`
    #[inline]
    pub unsafe fn GetWhatToShow(&self, aWhatToShow: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetWhatToShow)(self, aWhatToShow)
    }



    /// `attribute Node currentNode;`
    #[inline]
    pub unsafe fn GetCurrentNode(&self, aCurrentNode: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).GetCurrentNode)(self, aCurrentNode)
    }



    /// `attribute Node currentNode;`
    #[inline]
    pub unsafe fn SetCurrentNode(&self, aCurrentNode: *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).SetCurrentNode)(self, aCurrentNode)
    }



    /// `Node parentNode ();`
    #[inline]
    pub unsafe fn ParentNode(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).ParentNode)(self, _retval)
    }



    /// `Node firstChild ();`
    #[inline]
    pub unsafe fn FirstChild(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).FirstChild)(self, _retval)
    }



    /// `Node lastChild ();`
    #[inline]
    pub unsafe fn LastChild(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).LastChild)(self, _retval)
    }



    /// `Node previousSibling ();`
    #[inline]
    pub unsafe fn PreviousSibling(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).PreviousSibling)(self, _retval)
    }



    /// `Node nextSibling ();`
    #[inline]
    pub unsafe fn NextSibling(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).NextSibling)(self, _retval)
    }



    /// `Node previousNode ();`
    #[inline]
    pub unsafe fn PreviousNode(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).PreviousNode)(self, _retval)
    }



    /// `Node nextNode ();`
    #[inline]
    pub unsafe fn NextNode(&self, _retval: *mut *const libc::c_void) -> ::nserror::nsresult {
        ((*self.vtable).NextNode)(self, _retval)
    }


}


