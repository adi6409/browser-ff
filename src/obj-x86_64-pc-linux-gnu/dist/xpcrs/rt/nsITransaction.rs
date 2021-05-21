//
// DO NOT EDIT.  THIS FILE IS GENERATED FROM $SRCDIR/editor/txmgr/nsITransaction.idl
//


/// `interface nsITransaction : nsISupports`
///


// The actual type definition for the interface. This struct has methods
// declared on it which will call through its vtable. You never want to pass
// this type around by value, always pass it behind a reference.

#[repr(C)]
pub struct nsITransaction {
    vtable: *const nsITransactionVTable,

    /// This field is a phantomdata to ensure that the VTable type and any
    /// struct containing it is not safe to send across threads, as XPCOM is
    /// generally not threadsafe.
    ///
    /// XPCOM interfaces in general are not safe to send across threads.
    __nosync: ::std::marker::PhantomData<::std::rc::Rc<u8>>,
}

// Implementing XpCom for an interface exposes its IID, which allows for easy
// use of the `.query_interface<T>` helper method. This also defines that
// method for nsITransaction.
unsafe impl XpCom for nsITransaction {
    const IID: nsIID = nsID(0x58e330c1, 0x7b48, 0x11d2,
        [0x98, 0xb9, 0x00, 0x80, 0x5f, 0x29, 0x7d, 0x89]);
}

// We need to implement the RefCounted trait so we can be used with `RefPtr`.
// This trait teaches `RefPtr` how to manage our memory.
unsafe impl RefCounted for nsITransaction {
    #[inline]
    unsafe fn addref(&self) {
        self.AddRef();
    }
    #[inline]
    unsafe fn release(&self) {
        self.Release();
    }
}

// This trait is implemented on all types which can be coerced to from nsITransaction.
// It is used in the implementation of `fn coerce<T>`. We hide it from the
// documentation, because it clutters it up a lot.
#[doc(hidden)]
pub trait nsITransactionCoerce {
    /// Cheaply cast a value of this type from a `nsITransaction`.
    fn coerce_from(v: &nsITransaction) -> &Self;
}

// The trivial implementation: We can obviously coerce ourselves to ourselves.
impl nsITransactionCoerce for nsITransaction {
    #[inline]
    fn coerce_from(v: &nsITransaction) -> &Self {
        v
    }
}

impl nsITransaction {
    /// Cast this `nsITransaction` to one of its base interfaces.
    #[inline]
    pub fn coerce<T: nsITransactionCoerce>(&self) -> &T {
        T::coerce_from(self)
    }
}

// Every interface struct type implements `Deref` to its base interface. This
// causes methods on the base interfaces to be directly avaliable on the
// object. For example, you can call `.AddRef` or `.QueryInterface` directly
// on any interface which inherits from `nsISupports`.
impl ::std::ops::Deref for nsITransaction {
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
impl<T: nsISupportsCoerce> nsITransactionCoerce for T {
    #[inline]
    fn coerce_from(v: &nsITransaction) -> &Self {
        T::coerce_from(v)
    }
}

// This struct represents the interface's VTable. A pointer to a statically
// allocated version of this struct is at the beginning of every nsITransaction
// object. It contains one pointer field for each method in the interface. In
// the case where we can't generate a binding for a method, we include a void
// pointer.
#[doc(hidden)]
#[repr(C)]
pub struct nsITransactionVTable {
    /// We need to include the members from the base interface's vtable at the start
    /// of the VTable definition.
    pub __base: nsISupportsVTable,

    /* [can_run_script] void doTransaction (); */
    pub DoTransaction: unsafe extern "system" fn (this: *const nsITransaction) -> ::nserror::nsresult,

    /* [can_run_script] void undoTransaction (); */
    pub UndoTransaction: unsafe extern "system" fn (this: *const nsITransaction) -> ::nserror::nsresult,

    /* [can_run_script] void redoTransaction (); */
    pub RedoTransaction: unsafe extern "system" fn (this: *const nsITransaction) -> ::nserror::nsresult,

    /* readonly attribute boolean isTransient; */
    pub GetIsTransient: unsafe extern "system" fn (this: *const nsITransaction, aIsTransient: *mut bool) -> ::nserror::nsresult,

    /* boolean merge (in nsITransaction aTransaction); */
    pub Merge: unsafe extern "system" fn (this: *const nsITransaction, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult,

    /* [noscript] EditTransactionBasePtr getAsEditTransactionBase (); */
    /// Unable to generate binding because `native type mozilla::EditTransactionBase unsupported`
    pub GetAsEditTransactionBase: *const ::libc::c_void,
}


// The implementations of the function wrappers which are exposed to rust code.
// Call these methods rather than manually calling through the VTable struct.
impl nsITransaction {

    /// ```text
    /// /**
    ///    * Executes the transaction.
    ///    */
    /// ```
    ///

    /// `[can_run_script] void doTransaction ();`
    #[inline]
    pub unsafe fn DoTransaction(&self, ) -> ::nserror::nsresult {
        ((*self.vtable).DoTransaction)(self, )
    }


    /// ```text
    /// /**
    ///    * Restores the state to what it was before the transaction was executed.
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
    ///    * Executes the transaction again. Can only be called on a transaction that
    ///    * was previously undone.
    ///    * <P>
    ///    * In most cases, the redoTransaction() method will actually call the
    ///    * doTransaction() method to execute the transaction again.
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
    ///    * The transaction's transient state. This attribute is checked by
    ///    * the transaction manager after the transaction's Execute() method is called.
    ///    * If the transient state is false, a reference to the transaction is
    ///    * held by the transaction manager so that the transactions' undoTransaction()
    ///    * and redoTransaction() methods can be called. If the transient state is
    ///    * true, the transaction manager returns immediately after the transaction's
    ///    * doTransaction() method is called, no references to the transaction are
    ///    * maintained. Transient transactions cannot be undone or redone by the
    ///    * transaction manager.
    ///    */
    /// ```
    ///

    /// `readonly attribute boolean isTransient;`
    #[inline]
    pub unsafe fn GetIsTransient(&self, aIsTransient: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).GetIsTransient)(self, aIsTransient)
    }


    /// ```text
    /// /**
    ///    * Attempts to merge a transaction into "this" transaction. Both transactions
    ///    * must be in their undo state, doTransaction() methods already called. The
    ///    * transaction manager calls this method to coalesce a new transaction with
    ///    * the transaction on the top of the undo stack.
    ///    * This method returns a boolean value that indicates the merge result.
    ///    * A true value indicates that the transactions were merged successfully,
    ///    * a false value if the merge was not possible or failed. If true,
    ///    * the transaction manager will Release() the new transacton instead of
    ///    * pushing it on the undo stack.
    ///    * @param aTransaction the previously executed transaction to merge.
    ///    */
    /// ```
    ///

    /// `boolean merge (in nsITransaction aTransaction);`
    #[inline]
    pub unsafe fn Merge(&self, aTransaction: *const nsITransaction, _retval: *mut bool) -> ::nserror::nsresult {
        ((*self.vtable).Merge)(self, aTransaction, _retval)
    }



    /// `[noscript] EditTransactionBasePtr getAsEditTransactionBase ();`
    const _GetAsEditTransactionBase: () = ();

}


