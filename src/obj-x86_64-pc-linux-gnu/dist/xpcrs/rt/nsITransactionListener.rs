//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransactionListener.idl
//


/// `interface nsITransactionListener : nsISupports`
///

/// ```text
/// /**
///  * The nsITransactionListener interface.
///  * <P>
///  * This interface is implemented by an object that tracks transactions.
///  */
/// ```
///

// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransactionListener {
    vtable: *const nsITransactionListenerVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransactionListener.
unsafe impl XpCom for nsITransactionListener {
    const IID: nsIID = nsID(0x58e330c4, 0x7b48, 0x11d2,
        [0x98, 0xb9, 0x00, 0x80, 0x5f, 0x29, 0x7d, 0x89]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransactionListener {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransactionListener.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransactionListenerCoerce {
    /// Cheaply cast a value of this type from a `nsITransactionListener`.
    fn coerce_from(v: &nsITransactionListener) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransactionListenerCoerce for nsITransactionListener {
    #[inline]
    fn coerce_from(v: &nsITransactionListener) -> &Self {
        v
    }
}

impl nsITransactionListener {
    /// Cast this `nsITransactionListener` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransactionListenerCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransactionListener {
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
impl<T: nsISupportsCoerce> nsITransactionListenerCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransactionListener) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransactionListener
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransactionListenerVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    pub WillDo: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult,

    /* void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult); */
    pub DidDo: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aDoResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    pub WillUndo: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult,

    /* void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult); */
    pub DidUndo: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aUndoResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction); */
    pub WillRedo: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult,

    /* void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult); */
    pub DidRedo: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aRedoResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* boolean willBeginBatch (in nsITransactionManager aManager); */
    pub WillBeginBatch: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, _retval: *mut bool) -> ::nserror::nsresult,

    /* void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult); */
    pub DidBeginBatch: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* boolean willEndBatch (in nsITransactionManager aManager); */
    pub WillEndBatch: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, _retval: *mut bool) -> ::nserror::nsresult,

    /* void didEndBatch (in nsITransactionManager aManager, in nsresult aResult); */
    pub DidEndBatch: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aResult: ::nserror::nsresult) -> ::nserror::nsresult,

    /* boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge); */
    pub WillMerge: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTopTransaction: *const nsITransaction, aTransactionToMerge: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult,

    /* void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult); */
    pub DidMerge: unsafe extern "system" fn (this: *const nsITransactionListener, aManager: *const nsITransactionManager, aTopTransaction: *const nsITransaction, aTransactionToMerge: *const nsITransaction, aDidMerge: bool, aMergeResult: ::nserror::nsresult) -> ::nserror::nsresult,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransactionListener {

    /// ```text
    /// /**
    ///    * Called before a transaction manager calls a transaction's
    ///    * doTransaction() method.
    ///    * @param aManager the transaction manager doing the transaction.
    ///    * @param aTransaction the transaction being executed.
    ///    * @result boolean value returned by listener which indicates
    ///    * its desire to interrupt normal control flow. Listeners should
    ///    * return true if they want to interrupt normal control flow, without
    ///    * throwing an error.
    ///    */
    /// ```
    ///

    /// `boolean willDo (in nsITransactionManager aManager, in nsITransaction aTransaction);`
    #[inline]
    pub unsafe fn WillDo(&self, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WillDo)(self, aManager, aTransaction, _retval)
    }


    /// ```text
    /// /**
    ///    * Called after a transaction manager calls the doTransaction() method of
    ///    * a transaction.
    ///    * @param aManager the transaction manager that did the transaction.
    ///    * @param aTransaction the transaction that was executed.
    ///    * @param aDoResult the nsresult returned after executing
    ///    * the transaction.
    ///    */
    /// ```
    ///

    /// `void didDo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aDoResult);`
    #[inline]
    pub unsafe fn DidDo(&self, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aDoResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidDo)(self, aManager, aTransaction, aDoResult)
    }


    /// ```text
    /// /**
    ///    * Called before a transaction manager calls the Undo() method of
    ///    * a transaction.
    ///    * @param aManager the transaction manager undoing the transaction.
    ///    * @param aTransaction the transaction being undone.
    ///    * @result boolean value returned by listener which indicates
    ///    * its desire to interrupt normal control flow. Listeners should
    ///    * return true if they want to interrupt normal control flow, without
    ///    * throwing an error. Note that listeners can also interrupt normal
    ///    * control flow by throwing an nsresult that indicates an error.
    ///    */
    /// ```
    ///

    /// `boolean willUndo (in nsITransactionManager aManager, in nsITransaction aTransaction);`
    #[inline]
    pub unsafe fn WillUndo(&self, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WillUndo)(self, aManager, aTransaction, _retval)
    }


    /// ```text
    /// /**
    ///    * Called after a transaction manager calls the Undo() method of
    ///    * a transaction.
    ///    * @param aManager the transaction manager undoing the transaction.
    ///    * @param aTransaction the transaction being undone.
    ///    * @param aUndoResult the nsresult returned after undoing the transaction.
    ///    */
    /// ```
    ///

    /// `void didUndo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aUndoResult);`
    #[inline]
    pub unsafe fn DidUndo(&self, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aUndoResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidUndo)(self, aManager, aTransaction, aUndoResult)
    }


    /// ```text
    /// /**
    ///    * Called before a transaction manager calls the Redo() method of
    ///    * a transaction.
    ///    * @param aManager the transaction manager redoing the transaction.
    ///    * @param aTransaction the transaction being redone.
    ///    * @result boolean value returned by listener which indicates
    ///    * its desire to interrupt normal control flow. Listeners should
    ///    * return true if they want to interrupt normal control flow, without
    ///    * throwing an error. Note that listeners can also interrupt normal
    ///    * control flow by throwing an nsresult that indicates an error.
    ///    */
    /// ```
    ///

    /// `boolean willRedo (in nsITransactionManager aManager, in nsITransaction aTransaction);`
    #[inline]
    pub unsafe fn WillRedo(&self, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WillRedo)(self, aManager, aTransaction, _retval)
    }


    /// ```text
    /// /**
    ///    * Called after a transaction manager calls the Redo() method of
    ///    * a transaction.
    ///    * @param aManager the transaction manager redoing the transaction.
    ///    * @param aTransaction the transaction being redone.
    ///    * @param aRedoResult the nsresult returned after redoing the transaction.
    ///    */
    /// ```
    ///

    /// `void didRedo (in nsITransactionManager aManager, in nsITransaction aTransaction, in nsresult aRedoResult);`
    #[inline]
    pub unsafe fn DidRedo(&self, aManager: *const nsITransactionManager, aTransaction: *const nsITransaction, aRedoResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidRedo)(self, aManager, aTransaction, aRedoResult)
    }


    /// ```text
    /// /**
    ///    * Called before a transaction manager begins a batch.
    ///    * @param aManager the transaction manager beginning a batch.
    ///    * @result boolean value returned by listener which indicates
    ///    * its desire to interrupt normal control flow. Listeners should
    ///    * return true if they want to interrupt normal control flow, without
    ///    * throwing an error. Note that listeners can also interrupt normal
    ///    * control flow by throwing an nsresult that indicates an error.
    ///    */
    /// ```
    ///

    /// `boolean willBeginBatch (in nsITransactionManager aManager);`
    #[inline]
    pub unsafe fn WillBeginBatch(&self, aManager: *const nsITransactionManager, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WillBeginBatch)(self, aManager, _retval)
    }


    /// ```text
    /// /**
    ///    * Called after a transaction manager begins a batch.
    ///    * @param aManager the transaction manager that began a batch.
    ///    * @param aResult the nsresult returned after beginning a batch.
    ///    */
    /// ```
    ///

    /// `void didBeginBatch (in nsITransactionManager aManager, in nsresult aResult);`
    #[inline]
    pub unsafe fn DidBeginBatch(&self, aManager: *const nsITransactionManager, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidBeginBatch)(self, aManager, aResult)
    }


    /// ```text
    /// /**
    ///    * Called before a transaction manager ends a batch.
    ///    * @param aManager the transaction manager ending a batch.
    ///    * @result boolean value returned by listener which indicates
    ///    * its desire to interrupt normal control flow. Listeners should
    ///    * return true if they want to interrupt normal control flow, without
    ///    * throwing an error. Note that listeners can also interrupt normal
    ///    * control flow by throwing an nsresult that indicates an error.
    ///    */
    /// ```
    ///

    /// `boolean willEndBatch (in nsITransactionManager aManager);`
    #[inline]
    pub unsafe fn WillEndBatch(&self, aManager: *const nsITransactionManager, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WillEndBatch)(self, aManager, _retval)
    }


    /// ```text
    /// /**
    ///    * Called after a transaction manager ends a batch.
    ///    * @param aManager the transaction manager ending a batch.
    ///    * @param aResult the nsresult returned after ending a batch.
    ///    */
    /// ```
    ///

    /// `void didEndBatch (in nsITransactionManager aManager, in nsresult aResult);`
    #[inline]
    pub unsafe fn DidEndBatch(&self, aManager: *const nsITransactionManager, aResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidEndBatch)(self, aManager, aResult)
    }


    /// ```text
    /// /**
    ///    * Called before a transaction manager tries to merge
    ///    * a transaction, that was just executed, with the
    ///    * transaction at the top of the undo stack.
    ///    * @param aManager the transaction manager ending a batch.
    ///    * @param aTopTransaction the transaction at the top of the undo stack.
    ///    * @param aTransactionToMerge the transaction to merge.
    ///    * @result boolean value returned by listener which indicates
    ///    * its desire to interrupt normal control flow. Listeners should
    ///    * return true if they want to interrupt normal control flow, without
    ///    * throwing an error. Note that listeners can also interrupt normal
    ///    * control flow by throwing an nsresult that indicates an error.
    ///    */
    /// ```
    ///

    /// `boolean willMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge);`
    #[inline]
    pub unsafe fn WillMerge(&self, aManager: *const nsITransactionManager, aTopTransaction: *const nsITransaction, aTransactionToMerge: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).WillMerge)(self, aManager, aTopTransaction, aTransactionToMerge, _retval)
    }


    /// ```text
    /// /**
    ///    * Called after a transaction manager tries to merge
    ///    * a transaction, that was just executed, with the
    ///    * transaction at the top of the undo stack.
    ///    * @param aManager the transaction manager ending a batch.
    ///    * @param aTopTransaction the transaction at the top of the undo stack.
    ///    * @param aTransactionToMerge the transaction to merge.
    ///    * @param aDidMerge true if transaction was merged, else false.
    ///    * @param aMergeResult the nsresult returned after the merge attempt.
    ///    * @param aInterrupt listeners should set this to PR_TRUE if they
    ///    * want to interrupt normal control flow, without throwing an error.
    ///    */
    /// ```
    ///

    /// `void didMerge (in nsITransactionManager aManager, in nsITransaction aTopTransaction, in nsITransaction aTransactionToMerge, in boolean aDidMerge, in nsresult aMergeResult);`
    #[inline]
    pub unsafe fn DidMerge(&self, aManager: *const nsITransactionManager, aTopTransaction: *const nsITransaction, aTransactionToMerge: *const nsITransaction, aDidMerge: bool, aMergeResult: ::nserror::nsresult) -> ::nserror::nsresult {
        ((*self.vtable).DidMerge)(self, aManager, aTopTransaction, aTransactionToMerge, aDidMerge, aMergeResult)
    }


}


