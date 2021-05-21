//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransactionManager.idl
//


/// `interface nsITransactionManager : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransactionManager {
    vtable: *const nsITransactionManagerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransactionManager.
unsafe impl XpCom for nsITransactionManager {
    const IID: nsIID = nsID(0xc77763df, 0x0fb9, 0x41a8,
        [0x80, 0x74, 0x8e, 0x88, 0x2f, 0x60, 0x57, 0x55]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransactionManager {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransactionManager.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransactionManagerCoerce {
    /// Cheaply cast a value of this type from a `nsITransactionManager`.
    fn coerce_from(v: &nsITransactionManager) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransactionManagerCoerce for nsITransactionManager {
    #[inline]
    fn coerce_from(v: &nsITransactionManager) -> &Self {
        v
    }
}

impl nsITransactionManager {
    /// Cast this `nsITransactionManager` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransactionManagerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransactionManager {
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
impl<T: nsISupportsCoerce> nsITransactionManagerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransactionManager) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransactionManager
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransactionManagerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void doTransaction (in nsITransaction aTransaction); */
    pub DoTransaction: unsafe extern "system" fn (this: *const nsITransactionManager, aTransaction: *const nsITransaction) -> ::nserror::nsresult,

    /* [can_run_script] void undoTransaction (); */
    pub UndoTransaction: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* [can_run_script] void redoTransaction (); */
    pub RedoTransaction: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* void clear (); */
    pub Clear: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* void clearUndoStack (); */
    pub ClearUndoStack: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* void clearRedoStack (); */
    pub ClearRedoStack: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* [can_run_script] void beginBatch (in nsISupports aData); */
    pub BeginBatch: unsafe extern "system" fn (this: *const nsITransactionManager, aData: *const nsISupports) -> ::nserror::nsresult,

    /* void endBatch (in boolean aAllowEmpty); */
    pub EndBatch: unsafe extern "system" fn (this: *const nsITransactionManager, aAllowEmpty: bool) -> ::nserror::nsresult,

    /* readonly attribute long numberOfUndoItems; */
    pub GetNumberOfUndoItems: unsafe extern "system" fn (this: *const nsITransactionManager, aNumberOfUndoItems: *mut i32) -> ::nserror::nsresult,

    /* readonly attribute long numberOfRedoItems; */
    pub GetNumberOfRedoItems: unsafe extern "system" fn (this: *const nsITransactionManager, aNumberOfRedoItems: *mut i32) -> ::nserror::nsresult,

    /* attribute long maxTransactionCount; */
    pub GetMaxTransactionCount: unsafe extern "system" fn (this: *const nsITransactionManager, aMaxTransactionCount: *mut i32) -> ::nserror::nsresult,

    /* attribute long maxTransactionCount; */
    pub SetMaxTransactionCount: unsafe extern "system" fn (this: *const nsITransactionManager, aMaxTransactionCount: i32) -> ::nserror::nsresult,

    /* void batchTopUndo (); */
    pub BatchTopUndo: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* void removeTopUndo (); */
    pub RemoveTopUndo: unsafe extern "system" fn (this: *const nsITransactionManager) -> ::nserror::nsresult,

    /* nsITransaction peekUndoStack (); */
    pub PeekUndoStack: unsafe extern "system" fn (this: *const nsITransactionManager, _retval: *mut *const nsITransaction) -> ::nserror::nsresult,

    /* nsITransaction peekRedoStack (); */
    pub PeekRedoStack: unsafe extern "system" fn (this: *const nsITransactionManager, _retval: *mut *const nsITransaction) -> ::nserror::nsresult,

    /* void AddListener (in nsITransactionListener aListener); */
    pub AddListener: unsafe extern "system" fn (this: *const nsITransactionManager, aListener: *const nsITransactionListener) -> ::nserror::nsresult,

    /* void RemoveListener (in nsITransactionListener aListener); */
    pub RemoveListener: unsafe extern "system" fn (this: *const nsITransactionManager, aListener: *const nsITransactionListener) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransactionManager {

    /// ```text
    /// /**
    ///  * The nsITransactionManager interface.
    ///  * <P>
    ///  * This interface is implemented by an object that wants to
    ///  * manage/track transactions.
    ///  */
    /// /**
    ///    * Calls a transaction's doTransaction() method, then pushes it on the
    ///    * undo stack.
    ///    * <P>
    ///    * This method calls the transaction's AddRef() method.
    ///    * The transaction's Release() method will be called when the undo or redo
    ///    * stack is pruned or when the transaction manager is destroyed.
    ///    * @param aTransaction the transaction to do.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void doTransaction (in nsITransaction aTransaction);`
    #[inline]
    pub unsafe fn DoTransaction(&self, aTransaction: *const nsITransaction) -> ::nserror::nsresult {
        ((*self.vtable).DoTransaction)(self, aTransaction)
    }


    /// ```text
    /// /**
    ///    * Pops the topmost transaction on the undo stack, calls its
    ///    * undoTransaction() method, then pushes it on the redo stack.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void undoTransaction ();`
    #[inline]
    pub unsafe fn UndoTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).UndoTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Pops the topmost transaction on the redo stack, calls its
    ///    * redoTransaction() method, then pushes it on the undo stack.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void redoTransaction ();`
    #[inline]
    pub unsafe fn RedoTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RedoTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Clears the undo and redo stacks.
    ///    */
    /// ```
    ///

    /// `void clear ();`
    #[inline]
    pub unsafe fn Clear(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).Clear)(self, )
    }


    /// ```text
    /// /**
    ///    * Clears the undo stack only.
    ///    */
    /// ```
    ///

    /// `void clearUndoStack ();`
    #[inline]
    pub unsafe fn ClearUndoStack(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearUndoStack)(self, )
    }


    /// ```text
    /// /**
    ///    * Clears the redo stack only.
    ///    */
    /// ```
    ///

    /// `void clearRedoStack ();`
    #[inline]
    pub unsafe fn ClearRedoStack(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).ClearRedoStack)(self, )
    }


    /// ```text
    /// /**
    ///    * Turns on the transaction manager's batch mode, forcing all transactions
    ///    * executed by the transaction manager's doTransaction() method to be
    ///    * aggregated together until EndBatch() is called.  This mode allows an
    ///    * application to execute and group together several independent transactions
    ///    * so they can be undone with a single call to undoTransaction().
    ///    * @param aData An arbitrary nsISupports object that is associated with the
    ///    * batch. Can be retrieved from the undo or redo stacks.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void beginBatch (in nsISupports aData);`
    #[inline]
    pub unsafe fn BeginBatch(&self, aData: *const nsISupports) -> ::nserror::nsresult {
        ((*self.vtable).BeginBatch)(self, aData)
    }


    /// ```text
    /// /**
    ///    * Turns off the transaction manager's batch mode.
    ///    * @param aAllowEmpty If true, a batch containing no children will be
    ///    * pushed onto the undo stack. Otherwise, ending a batch with no
    ///    * children will result in no transactions being pushed on the undo stack.
    ///    */
    /// ```
    ///

    /// `void endBatch (in boolean aAllowEmpty);`
    #[inline]
    pub unsafe fn EndBatch(&self, aAllowEmpty: bool) -> ::nserror::nsresult {
        ((*self.vtable).EndBatch)(self, aAllowEmpty)
    }


    /// ```text
    /// /**
    ///    * The number of items on the undo stack.
    ///    */
    /// ```
    ///

    /// `readonly attribute long numberOfUndoItems;`
    #[inline]
    pub unsafe fn GetNumberOfUndoItems(&self, aNumberOfUndoItems: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumberOfUndoItems)(self, aNumberOfUndoItems)
    }


    /// ```text
    /// /**
    ///    * The number of items on the redo stack.
    ///    */
    /// ```
    ///

    /// `readonly attribute long numberOfRedoItems;`
    #[inline]
    pub unsafe fn GetNumberOfRedoItems(&self, aNumberOfRedoItems: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetNumberOfRedoItems)(self, aNumberOfRedoItems)
    }


    /// ```text
    /// /**
    ///    * Sets the maximum number of transaction items the transaction manager will
    ///    * maintain at any time. This is commonly referred to as the number of levels
    ///    * of undo.
    ///    * @param aMaxCount A value of -1 means no limit. A value of zero means the
    ///    * transaction manager will execute each transaction, then immediately release
    ///    * all references it has to the transaction without pushing it on the undo
    ///    * stack. A value greater than zero indicates the max number of transactions
    ///    * that can exist at any time on both the undo and redo stacks. This method
    ///    * will prune the necessary number of transactions on the undo and redo
    ///    * stacks if the value specified is less than the number of items that exist
    ///    * on both the undo and redo stacks.
    ///    */
    /// ```
    ///

    /// `attribute long maxTransactionCount;`
    #[inline]
    pub unsafe fn GetMaxTransactionCount(&self, aMaxTransactionCount: *mut i32) -> ::nserror::nsresult {
        ((*self.vtable).GetMaxTransactionCount)(self, aMaxTransactionCount)
    }


    /// ```text
    /// /**
    ///    * Sets the maximum number of transaction items the transaction manager will
    ///    * maintain at any time. This is commonly referred to as the number of levels
    ///    * of undo.
    ///    * @param aMaxCount A value of -1 means no limit. A value of zero means the
    ///    * transaction manager will execute each transaction, then immediately release
    ///    * all references it has to the transaction without pushing it on the undo
    ///    * stack. A value greater than zero indicates the max number of transactions
    ///    * that can exist at any time on both the undo and redo stacks. This method
    ///    * will prune the necessary number of transactions on the undo and redo
    ///    * stacks if the value specified is less than the number of items that exist
    ///    * on both the undo and redo stacks.
    ///    */
    /// ```
    ///

    /// `attribute long maxTransactionCount;`
    #[inline]
    pub unsafe fn SetMaxTransactionCount(&self, aMaxTransactionCount: i32) -> ::nserror::nsresult {
        ((*self.vtable).SetMaxTransactionCount)(self, aMaxTransactionCount)
    }


    /// ```text
    /// /**
    ///    * Combines the transaction at the top of the undo stack (if any) with the
    ///    * preceding undo transaction (if any) into a batch transaction. Thus,
    ///    * a call to undoTransaction() will undo both transactions.
    ///    */
    /// ```
    ///

    /// `void batchTopUndo ();`
    #[inline]
    pub unsafe fn BatchTopUndo(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).BatchTopUndo)(self, )
    }


    /// ```text
    /// /**
    ///    * Removes the transaction at the top of the undo stack (if any) without
    ///    * transacting.
    ///    */
    /// ```
    ///

    /// `void removeTopUndo ();`
    #[inline]
    pub unsafe fn RemoveTopUndo(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).RemoveTopUndo)(self, )
    }


    /// ```text
    /// /**
    ///    * Returns an AddRef'd pointer to the transaction at the top of the
    ///    * undo stack. Callers should be aware that this method could return
    ///    * return a null in some implementations if there is a batch at the top
    ///    * of the undo stack.
    ///    */
    /// ```
    ///

    /// `nsITransaction peekUndoStack ();`
    #[inline]
    pub unsafe fn PeekUndoStack(&self, _retval: *mut *const nsITransaction) -> ::nserror::nsresult {
        ((*self.vtable).PeekUndoStack)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Returns an AddRef'd pointer to the transaction at the top of the
    ///    * redo stack. Callers should be aware that this method could return
    ///    * return a null in some implementations if there is a batch at the top
    ///    * of the redo stack.
    ///    */
    /// ```
    ///

    /// `nsITransaction peekRedoStack ();`
    #[inline]
    pub unsafe fn PeekRedoStack(&self, _retval: *mut *const nsITransaction) -> ::nserror::nsresult {
        ((*self.vtable).PeekRedoStack)(self, _retval)
    }


    /// ```text
    /// /**
    ///    * Adds a listener to the transaction manager's notification list. Listeners
    ///    * are notified whenever a transaction is done, undone, or redone.
    ///    * <P>
    ///    * The listener's AddRef() method is called.
    ///    * @param aListener the lister to add.
    ///    */
    /// ```
    ///

    /// `void AddListener (in nsITransactionListener aListener);`
    #[inline]
    pub unsafe fn AddListener(&self, aListener: *const nsITransactionListener) -> ::nserror::nsresult {
        ((*self.vtable).AddListener)(self, aListener)
    }


    /// ```text
    /// /**
    ///    * Removes a listener from the transaction manager's notification list.
    ///    * <P>
    ///    * The listener's Release() method is called.
    ///    * @param aListener the lister to remove.
    ///    */
    /// ```
    ///

    /// `void RemoveListener (in nsITransactionListener aListener);`
    #[inline]
    pub unsafe fn RemoveListener(&self, aListener: *const nsITransactionListener) -> ::nserror::nsresult {
        ((*self.vtable).RemoveListener)(self, aListener)
    }


}


