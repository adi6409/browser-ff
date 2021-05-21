//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/docshell/base/nsIDocShellTreeOwner.idl
//


/// `interface nsIDocShellTreeOwner : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsIDocShellTreeOwner {
    vtable: *const nsIDocShellTreeOwnerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsIDocShellTreeOwner.
unsafe impl XpCom for nsIDocShellTreeOwner {
    const IID: nsIID = nsID(0x0e3dc4b1, 0x4cea, 0x4a37,
        [0xaf, 0x71, 0x79, 0xf0, 0xaf, 0xd0, 0x75, 0x74]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsIDocShellTreeOwner {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsIDocShellTreeOwner.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsIDocShellTreeOwnerCoerce {
    /// Cheaply cast a value of this type from a `nsIDocShellTreeOwner`.
    fn coerce_from(v: &nsIDocShellTreeOwner) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsIDocShellTreeOwnerCoerce for nsIDocShellTreeOwner {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeOwner) -> &Self {
        v
    }
}

impl nsIDocShellTreeOwner {
    /// Cast this `nsIDocShellTreeOwner` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsIDocShellTreeOwnerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsIDocShellTreeOwner {
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
impl<T: nsISupportsCoerce> nsIDocShellTreeOwnerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsIDocShellTreeOwner) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsIDocShellTreeOwner
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsIDocShellTreeOwnerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary); */
    pub ContentShellAdded: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aContentShell: *const nsIDocShellTreeItem, aPrimary: bool) -> ::nserror::nsresult,

    /* void contentShellRemoved (in nsIDocShellTreeItem aContentShell); */
    pub ContentShellRemoved: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aContentShell: *const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* readonly attribute nsIDocShellTreeItem primaryContentShell; */
    pub GetPrimaryContentShell: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aPrimaryContentShell: *mut*const nsIDocShellTreeItem) -> ::nserror::nsresult,

    /* void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary); */
    pub RemoteTabAdded: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aTab: *const nsIRemoteTab, aPrimary: bool) -> ::nserror::nsresult,

    /* void remoteTabRemoved (in nsIRemoteTab aTab); */
    pub RemoteTabRemoved: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aTab: *const nsIRemoteTab) -> ::nserror::nsresult,

    /* readonly attribute nsIRemoteTab primaryRemoteTab; */
    pub GetPrimaryRemoteTab: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aPrimaryRemoteTab: *mut*const nsIRemoteTab) -> ::nserror::nsresult,

    /* [can_run_script] void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy); */
    pub SizeShellTo: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, shell: *const nsIDocShellTreeItem, cx: i32, cy: i32) -> ::nserror::nsresult,

    /* void getPrimaryContentSize (out long width, out long height); */
    pub GetPrimaryContentSize: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void setPrimaryContentSize (in long width, in long height); */
    pub SetPrimaryContentSize: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, width: i32, height: i32) -> ::nserror::nsresult,

    /* void getRootShellSize (out long width, out long height); */
    pub GetRootShellSize: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, width: *mut i32, height: *mut i32) -> ::nserror::nsresult,

    /* void setRootShellSize (in long width, in long height); */
    pub SetRootShellSize: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, width: i32, height: i32) -> ::nserror::nsresult,

    /* void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode); */
    pub SetPersistence: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aPersistPosition: bool, aPersistSize: bool, aPersistSizeMode: bool) -> ::nserror::nsresult,

    /* void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode); */
    pub GetPersistence: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aPersistPosition: *mut bool, aPersistSize: *mut bool, aPersistSizeMode: *mut bool) -> ::nserror::nsresult,

    /* readonly attribute unsigned long tabCount; */
    pub GetTabCount: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aTabCount: *mut u32) -> ::nserror::nsresult,

    /* readonly attribute bool hasPrimaryContent; */
    pub GetHasPrimaryContent: unsafe extern "system" fn (this: *const nsIDocShellTreeOwner, aHasPrimaryContent: *mut bool) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsIDocShellTreeOwner {

    /// ```text
    /// /**
    /// 	 * Called when a content shell is added to the docshell tree.  This is
    /// 	 * _only_ called for "root" content shells (that is, ones whose parent is a
        /// 	 * chrome shell).
    /// 	 *
    /// 	 * @param aContentShell the shell being added.
    /// 	 * @param aPrimary whether the shell is primary.
    /// 	 */
    /// ```
    ///

    /// `void contentShellAdded (in nsIDocShellTreeItem aContentShell, in boolean aPrimary);`
    #[inline]
    pub unsafe fn ContentShellAdded(&self, aContentShell: *const nsIDocShellTreeItem, aPrimary: bool) -> ::nserror::nsresult {
        ((*self.vtable).ContentShellAdded)(self, aContentShell, aPrimary)
    }


    /// ```text
    /// /**
    /// 	 * Called when a content shell is removed from the docshell tree.  This is
    /// 	 * _only_ called for "root" content shells (that is, ones whose parent is a
        /// 	 * chrome shell).  Note that if aContentShell was never added,
    /// 	 * contentShellRemoved should just do nothing.
    /// 	 *
    /// 	 * @param aContentShell the shell being removed.
    /// 	 */
    /// ```
    ///

    /// `void contentShellRemoved (in nsIDocShellTreeItem aContentShell);`
    #[inline]
    pub unsafe fn ContentShellRemoved(&self, aContentShell: *const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).ContentShellRemoved)(self, aContentShell)
    }



    /// `readonly attribute nsIDocShellTreeItem primaryContentShell;`
    #[inline]
    pub unsafe fn GetPrimaryContentShell(&self, aPrimaryContentShell: *mut*const nsIDocShellTreeItem) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryContentShell)(self, aPrimaryContentShell)
    }



    /// `void remoteTabAdded (in nsIRemoteTab aTab, in boolean aPrimary);`
    #[inline]
    pub unsafe fn RemoteTabAdded(&self, aTab: *const nsIRemoteTab, aPrimary: bool) -> ::nserror::nsresult {
        ((*self.vtable).RemoteTabAdded)(self, aTab, aPrimary)
    }



    /// `void remoteTabRemoved (in nsIRemoteTab aTab);`
    #[inline]
    pub unsafe fn RemoteTabRemoved(&self, aTab: *const nsIRemoteTab) -> ::nserror::nsresult {
        ((*self.vtable).RemoteTabRemoved)(self, aTab)
    }



    /// `readonly attribute nsIRemoteTab primaryRemoteTab;`
    #[inline]
    pub unsafe fn GetPrimaryRemoteTab(&self, aPrimaryRemoteTab: *mut*const nsIRemoteTab) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryRemoteTab)(self, aPrimaryRemoteTab)
    }



    /// `[can_run_script] void sizeShellTo (in nsIDocShellTreeItem shell, in long cx, in long cy);`
    #[inline]
    pub unsafe fn SizeShellTo(&self, shell: *const nsIDocShellTreeItem, cx: i32, cy: i32) -> ::nserror::nsresult {
        ((*self.vtable).SizeShellTo)(self, shell, cx, cy)
    }



    /// `void getPrimaryContentSize (out long width, out long height);`
    #[inline]
    pub unsafe fn GetPrimaryContentSize(&self, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetPrimaryContentSize)(self, width, height)
    }



    /// `void setPrimaryContentSize (in long width, in long height);`
    #[inline]
    pub unsafe fn SetPrimaryContentSize(&self, width: i32, height: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetPrimaryContentSize)(self, width, height)
    }



    /// `void getRootShellSize (out long width, out long height);`
    #[inline]
    pub unsafe fn GetRootShellSize(&self, width: *mut i32, height: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetRootShellSize)(self, width, height)
    }



    /// `void setRootShellSize (in long width, in long height);`
    #[inline]
    pub unsafe fn SetRootShellSize(&self, width: i32, height: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetRootShellSize)(self, width, height)
    }



    /// `void setPersistence (in boolean aPersistPosition, in boolean aPersistSize, in boolean aPersistSizeMode);`
    #[inline]
    pub unsafe fn SetPersistence(&self, aPersistPosition: bool, aPersistSize: bool, aPersistSizeMode: bool) -> ::nserror::nsresult {
        ((*self.vtable).SetPersistence)(self, aPersistPosition, aPersistSize, aPersistSizeMode)
    }



    /// `void getPersistence (out boolean aPersistPosition, out boolean aPersistSize, out boolean aPersistSizeMode);`
    #[inline]
    pub unsafe fn GetPersistence(&self, aPersistPosition: *mut bool, aPersistSize: *mut bool, aPersistSizeMode: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetPersistence)(self, aPersistPosition, aPersistSize, aPersistSizeMode)
    }



    /// `readonly attribute unsigned long tabCount;`
    #[inline]
    pub unsafe fn GetTabCount(&self, aTabCount: *mut u32) -> ::nserror::nsresult {
        ((*self.vtable).GetTabCount)(self, aTabCount)
    }



    /// `readonly attribute bool hasPrimaryContent;`
    #[inline]
    pub unsafe fn GetHasPrimaryContent(&self, aHasPrimaryContent: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetHasPrimaryContent)(self, aHasPrimaryContent)
    }


}


